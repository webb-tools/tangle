// This file is part of Tangle.
// Copyright (C) 2022-2024 Tangle Foundation.
//
// This file is part of pallet-evm-precompileset-assets-erc20 package, originally developed by
// Purestake Inc. pallet-evm-precompileset-assets-erc20 package used in Tangle Network in terms of
// GPLv3.

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

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Display;
use fp_evm::{ExitError, PrecompileHandle};
use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	sp_runtime::traits::StaticLookup,
	traits::{
		fungibles::{
			approvals::Inspect as ApprovalInspect, metadata::Inspect as MetadataInspect, Inspect,
		},
		Get, OriginTrait,
	},
};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_core::{MaxEncodedLen, H160, H256, U256};
use sp_runtime::traits::{Bounded, Dispatchable};
use sp_std::{
	convert::{TryFrom, TryInto},
	marker::PhantomData,
};

mod eip2612;
use eip2612::Eip2612;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// Solidity selector of the Transfer log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TRANSFER: [u8; 32] = keccak256!("Transfer(address,address,uint256)");

/// Solidity selector of the Approval log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_APPROVAL: [u8; 32] = keccak256!("Approval(address,address,uint256)");

/// Alias for the Balance type for the provided Runtime and Instance.
pub type BalanceOf<Runtime, Instance = ()> = <Runtime as pallet_assets::Config<Instance>>::Balance;

/// Alias for the Asset Id type for the provided Runtime and Instance.
pub type AssetIdOf<Runtime, Instance = ()> = <Runtime as pallet_assets::Config<Instance>>::AssetId;

/// This trait ensure we can convert EVM address to AssetIds
/// We will require Runtime to have this trait implemented
pub trait AddressToAssetId<AssetId> {
	// Get assetId from address
	fn address_to_asset_id(address: H160) -> Option<AssetId>;

	// Get address from AssetId
	fn asset_id_to_address(asset_id: AssetId) -> H160;
}

/// The following distribution has been decided for the precompiles
/// 0-1023: Ethereum Mainnet Precompiles
/// 1024-2047 Precompiles that are not in Ethereum Mainnet but are neither Moonbeam specific
/// 2048-4095 Moonbeam specific precompiles
/// Asset precompiles can only fall between
/// 0xFFFFFFFF00000000000000000000000000000000 - 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
/// The precompile for AssetId X, where X is a u128 (i.e.16 bytes), if 0XFFFFFFFF + Bytes(AssetId)
/// In order to route the address to Erc20AssetsPrecompile<R>, we first check whether the AssetId
/// exists in pallet-assets
/// We cannot do this right now, so instead we check whether the total supply is zero. If so, we
/// do not route to the precompiles

/// This means that every address that starts with 0xFFFFFFFF will go through an additional db read,
/// but the probability for this to happen is 2^-32 for random addresses
pub struct Erc20AssetsPrecompileSet<Runtime, Instance: 'static = ()>(
	PhantomData<(Runtime, Instance)>,
);

impl<R, V> Clone for Erc20AssetsPrecompileSet<R, V> {
	fn clone(&self) -> Self {
		Self(PhantomData)
	}
}

impl<R, V> Default for Erc20AssetsPrecompileSet<R, V> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<Runtime, Instance> Erc20AssetsPrecompileSet<Runtime, Instance> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
#[precompile::precompile_set]
#[precompile::test_concrete_types(mock::Runtime, pallet_assets::Instance1)]
impl<Runtime, Instance> Erc20AssetsPrecompileSet<Runtime, Instance>
where
	Instance: eip2612::InstanceToPrefix + 'static,
	Runtime: pallet_assets::Config<Instance> + pallet_evm::Config + frame_system::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_assets::Call<Runtime, Instance>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	BalanceOf<Runtime, Instance>: TryFrom<U256> + Into<U256> + solidity::Codec,
	Runtime: AddressToAssetId<AssetIdOf<Runtime, Instance>>,
	<<Runtime as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin: OriginTrait,
	AssetIdOf<Runtime, Instance>: Display,
{
	/// PrecompileSet discriminant. Allows to knows if the address maps to an asset id,
	/// and if this is the case which one.
	#[precompile::discriminant]
	fn discriminant(address: H160, gas: u64) -> DiscriminantResult<AssetIdOf<Runtime, Instance>> {
		let extra_cost = RuntimeHelper::<Runtime>::db_read_gas_cost();
		if gas < extra_cost {
			return DiscriminantResult::OutOfGas;
		}

		let asset_id = match Runtime::address_to_asset_id(address) {
			Some(asset_id) => asset_id,
			None => return DiscriminantResult::None(extra_cost),
		};

		if pallet_assets::Pallet::<Runtime, Instance>::maybe_total_supply(asset_id.clone())
			.is_some()
		{
			DiscriminantResult::Some(asset_id, extra_cost)
		} else {
			DiscriminantResult::None(extra_cost)
		}
	}

	#[precompile::public("totalSupply()")]
	#[precompile::view]
	fn total_supply(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<U256> {
		// Storage item: Asset:
		// Blake2_128(16) + AssetId(16) + AssetDetails((4 * AccountId(20)) + (3 * Balance(16)) + 15)
		handle.record_db_read::<Runtime>(175)?;

		Ok(pallet_assets::Pallet::<Runtime, Instance>::total_issuance(asset_id).into())
	}

	#[precompile::public("balanceOf(address)")]
	#[precompile::view]
	fn balance_of(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		who: Address,
	) -> EvmResult<U256> {
		// Storage item: Account:
		// Blake2_128(16) + AssetId(16) + Blake2_128(16) + AccountId(20) + AssetAccount(19 + Extra)
		handle.record_db_read::<Runtime>(
			87 + <Runtime as pallet_assets::Config<Instance>>::Extra::max_encoded_len(),
		)?;

		let who: H160 = who.into();

		// Fetch info.
		let amount: U256 = {
			let who: Runtime::AccountId = Runtime::AddressMapping::into_account_id(who);
			pallet_assets::Pallet::<Runtime, Instance>::balance(asset_id, &who).into()
		};

		// Build output.
		Ok(amount)
	}

	#[precompile::public("allowance(address,address)")]
	#[precompile::view]
	fn allowance(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		owner: Address,
		spender: Address,
	) -> EvmResult<U256> {
		// Storage item: Approvals:
		// Blake2_128(16) + AssetId(16) + (2 * Blake2_128(16) + AccountId(20)) + Approval(32)
		handle.record_db_read::<Runtime>(136)?;

		let owner: H160 = owner.into();
		let spender: H160 = spender.into();

		// Fetch info.
		let amount: U256 = {
			let owner: Runtime::AccountId = Runtime::AddressMapping::into_account_id(owner);
			let spender: Runtime::AccountId = Runtime::AddressMapping::into_account_id(spender);

			// Fetch info.
			pallet_assets::Pallet::<Runtime, Instance>::allowance(asset_id, &owner, &spender).into()
		};

		// Build output.
		Ok(amount)
	}

	#[precompile::public("approve(address,uint256)")]
	fn approve(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		spender: Address,
		value: U256,
	) -> EvmResult<bool> {
		handle.record_log_costs_manual(3, 32)?;

		let spender: H160 = spender.into();

		Self::approve_inner(asset_id, handle, handle.context().caller, spender, value)?;

		log3(
			handle.context().address,
			SELECTOR_LOG_APPROVAL,
			handle.context().caller,
			spender,
			solidity::encode_event_data(value),
		)
		.record(handle)?;

		// Build output.
		Ok(true)
	}

	fn approve_inner(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		owner: H160,
		spender: H160,
		value: U256,
	) -> EvmResult {
		let owner = Runtime::AddressMapping::into_account_id(owner);
		let spender: Runtime::AccountId = Runtime::AddressMapping::into_account_id(spender);
		// Amount saturate if too high.
		let amount: BalanceOf<Runtime, Instance> =
			value.try_into().unwrap_or_else(|_| Bounded::max_value());

		// Storage item: Approvals:
		// Blake2_128(16) + AssetId(16) + (2 * Blake2_128(16) + AccountId(20)) + Approval(32)
		handle.record_db_read::<Runtime>(136)?;

		// If previous approval exists, we need to clean it
		if pallet_assets::Pallet::<Runtime, Instance>::allowance(asset_id.clone(), &owner, &spender)
			!= 0u32.into()
		{
			RuntimeHelper::<Runtime>::try_dispatch(
				handle,
				Some(owner.clone()).into(),
				pallet_assets::Call::<Runtime, Instance>::cancel_approval {
					id: asset_id.clone().into(),
					delegate: Runtime::Lookup::unlookup(spender.clone()),
				},
			)?;
		}
		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(
			handle,
			Some(owner).into(),
			pallet_assets::Call::<Runtime, Instance>::approve_transfer {
				id: asset_id.into(),
				delegate: Runtime::Lookup::unlookup(spender),
				amount,
			},
		)?;

		Ok(())
	}

	#[precompile::public("transfer(address,uint256)")]
	fn transfer(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		to: Address,
		value: U256,
	) -> EvmResult<bool> {
		handle.record_log_costs_manual(3, 32)?;

		let to: H160 = to.into();
		let value = Self::u256_to_amount(value).in_field("value")?;

		// Build call with origin.
		{
			let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
			let to = Runtime::AddressMapping::into_account_id(to);

			// Dispatch call (if enough gas).
			RuntimeHelper::<Runtime>::try_dispatch(
				handle,
				Some(origin).into(),
				pallet_assets::Call::<Runtime, Instance>::transfer {
					id: asset_id.into(),
					target: Runtime::Lookup::unlookup(to),
					amount: value,
				},
			)?;
		}

		log3(
			handle.context().address,
			SELECTOR_LOG_TRANSFER,
			handle.context().caller,
			to,
			solidity::encode_event_data(value),
		)
		.record(handle)?;

		Ok(true)
	}

	#[precompile::public("transferFrom(address,address,uint256)")]
	fn transfer_from(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		from: Address,
		to: Address,
		value: U256,
	) -> EvmResult<bool> {
		handle.record_log_costs_manual(3, 32)?;

		let from: H160 = from.into();
		let to: H160 = to.into();
		let value = Self::u256_to_amount(value).in_field("value")?;

		{
			let caller: Runtime::AccountId =
				Runtime::AddressMapping::into_account_id(handle.context().caller);
			let from: Runtime::AccountId = Runtime::AddressMapping::into_account_id(from);
			let to: Runtime::AccountId = Runtime::AddressMapping::into_account_id(to);

			// If caller is "from", it can spend as much as it wants from its own balance.
			if caller != from {
				// Dispatch call (if enough gas).
				RuntimeHelper::<Runtime>::try_dispatch(
					handle,
					Some(caller).into(),
					pallet_assets::Call::<Runtime, Instance>::transfer_approved {
						id: asset_id.into(),
						owner: Runtime::Lookup::unlookup(from),
						destination: Runtime::Lookup::unlookup(to),
						amount: value,
					},
				)?;
			} else {
				// Dispatch call (if enough gas).
				RuntimeHelper::<Runtime>::try_dispatch(
					handle,
					Some(from).into(),
					pallet_assets::Call::<Runtime, Instance>::transfer {
						id: asset_id.into(),
						target: Runtime::Lookup::unlookup(to),
						amount: value,
					},
				)?;
			}
		}

		log3(
			handle.context().address,
			SELECTOR_LOG_TRANSFER,
			from,
			to,
			solidity::encode_event_data(value),
		)
		.record(handle)?;

		// Build output.
		Ok(true)
	}

	#[precompile::public("name()")]
	#[precompile::view]
	fn name(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<UnboundedBytes> {
		// Storage item: Metadata:
		// Blake2_128(16) + AssetId(16) + AssetMetadata[deposit(16) + name(StringLimit)
		// + symbol(StringLimit) + decimals(1) + is_frozen(1)]
		handle.record_db_read::<Runtime>(
			50 + (2 * <Runtime as pallet_assets::Config<Instance>>::StringLimit::get()) as usize,
		)?;

		let name = pallet_assets::Pallet::<Runtime, Instance>::name(asset_id).as_slice().into();

		Ok(name)
	}

	#[precompile::public("symbol()")]
	#[precompile::view]
	fn symbol(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<UnboundedBytes> {
		// Storage item: Metadata:
		// Blake2_128(16) + AssetId(16) + AssetMetadata[deposit(16) + name(StringLimit)
		// + symbol(StringLimit) + decimals(1) + is_frozen(1)]
		handle.record_db_read::<Runtime>(
			50 + (2 * <Runtime as pallet_assets::Config<Instance>>::StringLimit::get()) as usize,
		)?;

		let symbol = pallet_assets::Pallet::<Runtime, Instance>::symbol(asset_id).as_slice().into();

		Ok(symbol)
	}

	#[precompile::public("decimals()")]
	#[precompile::view]
	fn decimals(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<u8> {
		// Storage item: Metadata:
		// Blake2_128(16) + AssetId(16) + AssetMetadata[deposit(16) + name(StringLimit)
		// + symbol(StringLimit) + decimals(1) + is_frozen(1)]
		handle.record_db_read::<Runtime>(
			50 + (2 * <Runtime as pallet_assets::Config<Instance>>::StringLimit::get()) as usize,
		)?;

		Ok(pallet_assets::Pallet::<Runtime, Instance>::decimals(asset_id))
	}

	#[allow(clippy::too_many_arguments)]
	#[precompile::public("permit(address,address,uint256,uint256,uint8,bytes32,bytes32)")]
	fn eip2612_permit(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		owner: Address,
		spender: Address,
		value: U256,
		deadline: U256,
		v: u8,
		r: H256,
		s: H256,
	) -> EvmResult {
		<Eip2612<Runtime, Instance>>::permit(
			asset_id, handle, owner, spender, value, deadline, v, r, s,
		)
	}

	#[precompile::public("nonces(address)")]
	#[precompile::view]
	fn eip2612_nonces(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
		owner: Address,
	) -> EvmResult<U256> {
		<Eip2612<Runtime, Instance>>::nonces(asset_id, handle, owner)
	}

	#[precompile::public("DOMAIN_SEPARATOR()")]
	#[precompile::view]
	fn eip2612_domain_separator(
		asset_id: AssetIdOf<Runtime, Instance>,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<H256> {
		<Eip2612<Runtime, Instance>>::domain_separator(asset_id, handle)
	}

	fn u256_to_amount(value: U256) -> MayRevert<BalanceOf<Runtime, Instance>> {
		value
			.try_into()
			.map_err(|_| RevertReason::value_is_too_large("balance type").into())
	}
}
