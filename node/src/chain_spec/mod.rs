// Copyright 2022 Webb Technologies Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use arkworks_setups::{common::setup_params, Curve};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, ByteArray, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use tangle_rococo_runtime::{
	AccountId, AssetRegistryConfig, AuraId, ClaimsConfig, DKGId, HasherBn254Config,
	MerkleTreeBn254Config, MixerBn254Config, MixerVerifierBn254Config, Signature,
	VAnchorBn254Config, VAnchorVerifier2x2Bn254Config, EXISTENTIAL_DEPOSIT, MILLIUNIT, UNIT,
};

pub mod rococo;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<tangle_rococo_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain
/// in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Generate DKG keys from seed.
///
/// This function's return type must always match the session keys of the chain
/// in tuple format.
pub fn get_dkg_keys_from_seed(seed: &str) -> DKGId {
	get_from_seed::<DKGId>(seed)
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we
/// have just one key).
pub fn dkg_session_keys(keys: AuraId, dkg_keys: DKGId) -> tangle_rococo_runtime::SessionKeys {
	tangle_rococo_runtime::SessionKeys { aura: keys, dkg: dkg_keys }
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Convert public keys to Acco, Aura and DKG keys
fn generate_invulnerables<PK: Clone + Into<AccountId>>(
	public_keys: &[(PK, DKGId)],
) -> Vec<(AccountId, AuraId, DKGId)> {
	public_keys
		.iter()
		.map(|pk| {
			let account: AccountId = pk.0.clone().into();
			let aura_id = AuraId::from_slice(account.as_ref()).unwrap();
			(account, aura_id, pk.clone().1)
		})
		.collect()
}

pub fn development_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "tTNT".into());
	properties.insert("tokenDecimals".into(), 12u32.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
						get_dkg_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
						get_dkg_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("tangle-dev"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "tTNT".into());
	properties.insert("tokenDecimals".into(), 12u32.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// root
				hex!["a62a5c2e22ebd14273f1e6552ba0ee07937ff3d859f53475296bbcbb8af1752e"].into(),
				// invulnerables
				generate_invulnerables::<[u8; 32]>(&[
					(
						// publickey
						hex!["a62a5c2e22ebd14273f1e6552ba0ee07937ff3d859f53475296bbcbb8af1752e"],
						// DKG key --scheme Ecdsa
						hex!["03fd0f9d6e4ef6eeb0718866a43c04764177f3fc03203e9ff7ed4dd2885cb52943"]
							.unchecked_into(),
					),
					(
						// publickey
						hex!["6850cc5d0369d11f93c820b91f7bfed4f6fc8b3a5f70a80171183129face154b"],
						// DKG key --scheme Ecdsa
						hex!["03ae1a02a91d59ff20ece458640afbbb672b9335f7da4c9f7d699129d431680ae9"]
							.unchecked_into(),
					),
					(
						// publickey
						hex!["1469f5f6719beaa0a7364259e5fb10846a4457f181807a0c00a6a9cdf14a260d"],
						// DKG key --scheme Ecdsa
						hex!["0252abf0dd2ed408700de539fd65dfc2f6d201e76a4c2e19b875d7b3176a468b0f"]
							.unchecked_into(),
					),
				]),
				vec![
					// aura accounts
					hex!["a62a5c2e22ebd14273f1e6552ba0ee07937ff3d859f53475296bbcbb8af1752e"].into(),
					hex!["6850cc5d0369d11f93c820b91f7bfed4f6fc8b3a5f70a80171183129face154b"].into(),
					hex!["1469f5f6719beaa0a7364259e5fb10846a4457f181807a0c00a6a9cdf14a260d"].into(),
					// acco accounts
					hex!["703ba5a042652271121c13137a4b1f3bc237c79e44beb1cad069d194f66e1131"].into(),
					hex!["c0005f98dec97a11a8537735c4dfc9edc253cc4914b86830af11b2a9b132897b"].into(),
					hex!["a43f0787f3156b00b30ccc19462146b8a3481e85dcdfc2a9ccb4b16347b65e69"].into(),
				],
				id,
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("tangle-template-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId, DKGId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> tangle_rococo_runtime::GenesisConfig {
	let curve_bn254 = Curve::Bn254;

	log::info!("Bn254 x5 w3 params");
	let bn254_x5_3_params = setup_params::<ark_bn254::Fr>(curve_bn254, 5, 3);

	log::info!("Verifier params for mixer");
	let mixer_verifier_bn254_params = {
		let vk_bytes = include_bytes!("../../../verifying_keys/mixer/bn254/verifying_key.bin");
		vk_bytes.to_vec()
	};

	log::info!("Verifier params for vanchor");
	let vanchor_verifier_bn254_params = {
		let vk_bytes =
			include_bytes!("../../../verifying_keys/vanchor/bn254/x5/2-2-2/verifying_key.bin");
		vk_bytes.to_vec()
	};

	tangle_rococo_runtime::GenesisConfig {
		system: tangle_rococo_runtime::SystemConfig {
			code: tangle_rococo_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		claims: ClaimsConfig { claims: vec![], vesting: vec![], expiry: None },
		sudo: tangle_rococo_runtime::SudoConfig { key: Some(root_key) },
		balances: tangle_rococo_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, MILLIUNIT * 4_096_000))
				.collect(),
		},
		democracy: Default::default(),
		council: Default::default(),
		indices: Default::default(),
		parachain_info: tangle_rococo_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: tangle_rococo_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: tangle_rococo_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura, dkg)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						dkg_session_keys(aura, dkg), // session keys
					)
				})
				.collect(),
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		dkg: tangle_rococo_runtime::DKGConfig {
			authorities: invulnerables.iter().map(|x| x.2.clone()).collect::<_>(),
			keygen_threshold: 3,
			signature_threshold: 1,
			authority_ids: invulnerables.iter().map(|x| x.0.clone()).collect::<_>(),
		},
		dkg_proposals: Default::default(),
		asset_registry: AssetRegistryConfig {
			asset_names: vec![],
			native_asset_name: b"WEBB".to_vec(),
			native_existential_deposit: tangle_rococo_runtime::EXISTENTIAL_DEPOSIT,
		},
		hasher_bn_254: HasherBn254Config {
			parameters: Some(bn254_x5_3_params.to_bytes()),
			phantom: Default::default(),
		},
		mixer_verifier_bn_254: MixerVerifierBn254Config {
			parameters: Some(mixer_verifier_bn254_params),
			phantom: Default::default(),
		},
		merkle_tree_bn_254: MerkleTreeBn254Config {
			phantom: Default::default(),
			default_hashes: None,
		},
		mixer_bn_254: MixerBn254Config {
			mixers: vec![(0, 10 * UNIT), (0, 100 * UNIT), (0, 1000 * UNIT)],
		},
		v_anchor_verifier_2x_2_bn_254: VAnchorVerifier2x2Bn254Config {
			parameters: Some(vanchor_verifier_bn254_params),
			phantom: Default::default(),
		},
		v_anchor_bn_254: VAnchorBn254Config {
			max_deposit_amount: 1_000_000 * UNIT,
			min_withdraw_amount: 0,
			vanchors: vec![(0, 1)],
			phantom: Default::default(),
		},
		treasury: Default::default(),
		vesting: Default::default(),
	}
}
