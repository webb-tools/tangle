// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{error::Error, Crypto};
use beefy_light_client_primitives::{MerkleHasher, SignatureWithAuthorityIndex};
use frame_support::sp_runtime::traits::Convert;
use parity_scale_codec::{Decode, Encode};
use sp_core::keccak_256;
use sp_runtime::traits::BlakeTwo256;
use sp_trie::{generate_trie_proof, TrieDBMutBuilder, TrieMut};
use std::{collections::BTreeMap, sync::Arc};
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	Config,
};

/// Holds the timestamp inherent alongside a merkle-patricia trie proof of its existence in a given
/// block.
pub struct TimeStampExtWithProof {
	/// The timestamp inherent SCALE-encoded bytes. Decode with [`UncheckedExtrinsic`]
	pub ext: Vec<u8>,
	/// Merkle-patricia trie existence proof for the extrinsic, this is generated by the relayer.
	pub proof: Vec<Vec<u8>>,
}

/// This holds the signatures of a BEEFY commitment, along side a merkle multi-proof of the
/// existence of the ethereum addresses associated with the signatures.
pub struct AuthorityProofWithSignatures {
	/// Merkle multi-proof
	pub authority_proof: Vec<[u8; 32]>,
	/// The actual signatures alongside the authority index, used in verifying the merkle proof.
	pub signatures: Vec<SignatureWithAuthorityIndex>,
}

/// This holds the proof that a parachain header was included in the parachain heads root (extra
/// data) field in an mmr leaf.
pub struct ParaHeadsProof {
	/// Merkle proof of existence of parachain header
	pub parachain_heads_proof: Vec<[u8; 32]>,
	/// This is the actual parachain header, SCALE-encoded
	pub para_head: Vec<u8>,
	/// This is the index of the parachain header in the parachain heads tree.
	pub heads_leaf_index: u32,
	/// This is the total count of parachain headers in the parachain header tree.
	pub heads_total_count: u32,
}

/// Fetch timestamp extrinsic and it's proof
pub async fn fetch_timestamp_extrinsic_with_proof<T: Config>(
	client: &RpcClient,
	block_hash: Option<T::Hash>,
) -> Result<TimeStampExtWithProof, Error> {
	let legacy_rpc_methods = LegacyRpcMethods::<T>::new(client.clone());
	let block = legacy_rpc_methods.chain_get_block(block_hash).await?.ok_or_else(|| {
		Error::Custom(format!(
			"[get_parachain_headers] Block with hash :{:?} not found",
			block_hash
		))
	})?;

	let extrinsics = block.block.extrinsics.into_iter().map(|e| e.0.encode()).collect::<Vec<_>>();
	let (ext, proof) = {
		if extrinsics.is_empty() {
			return Err(From::from("Block has no extrinsics".to_string()));
		}
		let timestamp_ext = extrinsics[0].clone();

		let mut db = sp_trie::MemoryDB::<BlakeTwo256>::default();

		let root = {
			let mut root = Default::default();
			let mut trie =
				<TrieDBMutBuilder<sp_trie::LayoutV0<BlakeTwo256>>>::new(&mut db, &mut root).build();
			for (i, ext) in extrinsics.into_iter().enumerate() {
				let key = parity_scale_codec::Compact(i as u64).encode();
				trie.insert(&key, &ext)?;
			}
			*trie.root()
		};
		let key = parity_scale_codec::Compact::<u32>(0u32).encode();

		let extrinsic_proof =
			generate_trie_proof::<sp_trie::LayoutV0<BlakeTwo256>, _, _, _>(&db, root, vec![&key])?;

		(timestamp_ext, extrinsic_proof)
	};

	Ok(TimeStampExtWithProof { ext, proof })
}

/// Parachain idenitfier type
pub type ParaId = u32;
/// SCALE-encoded parachain header
pub type HeadData = Vec<u8>;

/// Calculate the proof for the parachain heads added to this leaf
pub fn prove_parachain_headers(
	// Map of para ids to to finalized head data
	finalized_para_heads: &BTreeMap<ParaId, HeadData>,
	para_id: u32,
) -> Result<ParaHeadsProof, Error> {
	let mut index = None;
	let mut parachain_leaves = vec![];
	// Values are already sorted by key which is the para_id
	for (idx, (key, header)) in finalized_para_heads.iter().enumerate() {
		let pair = (*key, header.clone());
		parachain_leaves.push(keccak_256(pair.encode().as_slice()));
		if *key == para_id {
			index = Some(idx);
		}
	}

	let heads_leaf_index = index.ok_or_else(|| {
		Error::Custom("[get_parachain_headers] heads leaf index is None".to_string())
	})? as u32;

	let tree = rs_merkle::MerkleTree::<MerkleHasher<Crypto>>::from_leaves(&parachain_leaves);

	let proof = tree
		.proof(&[heads_leaf_index as usize])
		.proof_hashes()
		.into_iter()
		.map(|item| item.clone())
		.collect::<Vec<_>>();

	let para_head = finalized_para_heads
		.get(&para_id)
		.ok_or_else(|| {
			Error::Custom(format!(
				"[get_parachain_headers] Para Header not found for para id {}",
				para_id
			))
		})?
		.clone();
	Ok(ParaHeadsProof {
		parachain_heads_proof: proof,
		para_head,
		heads_leaf_index,
		heads_total_count: parachain_leaves.len() as u32,
	})
}

/// Get the proof for authority set that signed this commitment
pub fn prove_authority_set(
	signed_commitment: &sp_consensus_beefy::SignedCommitment<
		u32,
		sp_consensus_beefy::ecdsa_crypto::Signature,
	>,
	authority_address_hashes: Vec<[u8; 32]>,
) -> Result<AuthorityProofWithSignatures, Error> {
	let signatures = signed_commitment
		.signatures
		.iter()
		.enumerate()
		.map(|(index, x)| {
			if let Some(sig) = x {
				let mut temp = [0u8; 65];
				if sig.len() == 65 {
					temp.copy_from_slice(&*sig.encode());
					Some(SignatureWithAuthorityIndex { index: index as u32, signature: temp })
				} else {
					None
				}
			} else {
				None
			}
		})
		.filter_map(|x| x)
		.collect::<Vec<_>>();

	let signature_indices = signatures.iter().map(|x| x.index as usize).collect::<Vec<_>>();

	let tree =
		rs_merkle::MerkleTree::<MerkleHasher<Crypto>>::from_leaves(&authority_address_hashes);

	let authority_proof = tree.proof(&signature_indices);
	Ok(AuthorityProofWithSignatures {
		authority_proof: authority_proof.proof_hashes().to_vec(),
		signatures,
	})
}

/// Hash encoded authority public keys
pub fn hash_authority_addresses(encoded_public_keys: Vec<Vec<u8>>) -> Result<Vec<[u8; 32]>, Error> {
	let authority_address_hashes = encoded_public_keys
		.into_iter()
		.map(|x| {
			sp_consensus_beefy::ecdsa_crypto::AuthorityId::decode(&mut &*x)
				.map(|id| keccak_256(&pallet_beefy_mmr::BeefyEcdsaToEthereum::convert(id)))
		})
		.collect::<Result<Vec<_>, parity_scale_codec::Error>>()?;
	Ok(authority_address_hashes)
}

/// Perform a highly unsafe type-casting between two types hidden behind an Arc.
pub unsafe fn unsafe_arc_cast<T, U>(arc: Arc<T>) -> Arc<U> {
	let ptr = Arc::into_raw(arc).cast::<U>();
	Arc::from_raw(ptr)
}
