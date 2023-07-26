// This file is part of Webb.

// Copyright (C) 2021 Webb Technologies Inc.
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

use crate::{
	precompiles::{PrecompileName, WebbPrecompiles},
	*,
};
use frame_support::{
	pallet_prelude::*,
	parameter_types,
	traits::FindAuthor,
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use sp_core::{crypto::ByteArray, H160, U256};
use sp_runtime::{traits::BlakeTwo256, ConsensusEngineId, Permill};
use sp_std::{marker::PhantomData, prelude::*};
// Frontier
use pallet_ethereum::PostLogContent;
use pallet_evm::HashedAddressMapping;

impl pallet_evm_chain_id::Config for Runtime {}

pub struct FindAuthorTruncated<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Aura::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]))
		}
		None
	}
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, MaxEncodedLen, TypeInfo,
)]
pub enum ProxyType {
	/// All calls can be proxied. This is the trivial/most permissive filter.
	Any = 0,
	/// Only extrinsics related to governance (democracy and collectives).
	Governance = 1,
	/// Allow to veto an announced proxy call.
	CancelProxy = 2,
	/// Allow extrinsic related to Balances.
	Balances = 3,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

fn is_governance_precompile(precompile_name: &precompiles::PrecompileName) -> bool {
	matches!(
		precompile_name,
		PrecompileName::DemocracyPrecompile | PrecompileName::PreimagePrecompile
	)
}

// Be careful: Each time this filter is modified, the substrate filter must also be modified
// consistently.
impl pallet_evm_precompile_proxy::EvmProxyCallFilter for ProxyType {
	fn is_evm_proxy_call_allowed(
		&self,
		call: &pallet_evm_precompile_proxy::EvmSubCall,
		recipient_has_code: bool,
		gas: u64,
	) -> precompile_utils::EvmResult<bool> {
		Ok(match self {
			ProxyType::Any => true,
			ProxyType::Governance =>
				call.value == U256::zero() &&
					matches!(
						PrecompileName::from_address(call.to.0),
						Some(ref precompile) if is_governance_precompile(precompile)
					),
			// The proxy precompile does not contain method cancel_proxy
			ProxyType::CancelProxy => false,
			ProxyType::Balances => {
				// Allow only "simple" accounts as recipient (no code nor precompile).
				// Note: Checking the presence of the code is not enough because some precompiles
				// have no code.
				!recipient_has_code &&
					!precompile_utils::precompile_set::is_precompile_or_fail::<Runtime>(
						call.to.0, gas,
					)?
			},
		})
	}
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND / GAS_PER_SECOND;

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
// Here we assume Ethereum's base fee of 21000 gas and convert to weight, but we
// subtract roughly the cost of a balance transfer from it (about 1/3 the cost)
// and some cost to account for per-byte-fee.
// TODO: we should use benchmarking's overhead feature to measure this
pub const EXTRINSIC_BASE_WEIGHT: Weight = Weight::from_parts(10000 * WEIGHT_PER_GAS, 0);

parameter_types! {
	pub BlockGasLimit: U256
		= U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
	/// The amount of gas per pov. A ratio of 4 if we convert ref_time to gas and we compare
	/// it with the pov_size for a block. E.g.
	/// ceil(
	///     (max_extrinsic.ref_time() / max_extrinsic.proof_size()) / WEIGHT_PER_GAS
	/// )
	pub const GasLimitPovSizeRatio: u64 = 4;
	pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
	pub PrecompilesValue: WebbPrecompiles<Runtime> = WebbPrecompiles::<_>::new();
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = BaseFee;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	type AddressMapping = HashedAddressMapping<BlakeTwo256>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = WebbPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = EVMChainId;
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction =
		pallet_evm::EVMCurrencyAdapter<Balances, impls::DealWithFees<Runtime>>;
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated<Aura>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type Timestamp = Timestamp;
	type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}

parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
	type PostLogContent = PostBlockAndTxnHashes;
	type ExtraDataLength = ConstU32<30>;
}

parameter_types! {
	pub BoundDivision: U256 = U256::from(1024);
}

impl pallet_dynamic_fee::Config for Runtime {
	type MinGasPriceBoundDivisor = BoundDivision;
}

parameter_types! {
	pub DefaultBaseFeePerGas: U256 = U256::from(WEIGHT_FEE);
	// 1.5%
	pub DefaultElasticity: Permill = Permill::from_parts(15_000);
}

pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
	fn lower() -> Permill {
		Permill::zero()
	}
	fn ideal() -> Permill {
		Permill::from_parts(100_000)
	}
	fn upper() -> Permill {
		Permill::from_parts(250_000)
	}
}

impl pallet_base_fee::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Threshold = BaseFeeThreshold;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
}

impl pallet_hotfix_sufficients::Config for Runtime {
	type AddressMapping = HashedAddressMapping<BlakeTwo256>;
	type WeightInfo = pallet_hotfix_sufficients::weights::SubstrateWeight<Runtime>;
}
