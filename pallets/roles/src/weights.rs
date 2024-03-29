
//! Autogenerated weights for `pallet_roles`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Salmans-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/tangle
// benchmark
// pallet
// --chain
// dev
// --pallet=pallet-roles
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output=./pallets/roles/src/weights.rs
// --template=./.maintain/frame-weights-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_roles`.
pub trait WeightInfo {
	fn create_profile() -> Weight;
	fn update_profile() -> Weight;
	fn delete_profile() -> Weight;
	fn chill() -> Weight;
	fn unbond_funds() -> Weight;
	fn withdraw_unbonded() -> Weight;
}

/// Weights for `pallet_roles` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Roles::Ledger` (r:1 w:1)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Roles::MinRestakingBond` (r:1 w:0)
	/// Proof: `Roles::MinRestakingBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Roles::AccountRolesMapping` (r:0 w:1)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn create_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1654`
		//  Estimated: `6944`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 6944)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Roles::Ledger` (r:1 w:1)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Roles::MinRestakingBond` (r:1 w:0)
	/// Proof: `Roles::MinRestakingBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Jobs::ValidatorJobIdLookup` (r:1 w:0)
	/// Proof: `Jobs::ValidatorJobIdLookup` (`max_values`: None, `max_size`: Some(1042), added: 3517, mode: `MaxEncodedLen`)
	/// Storage: `Roles::AccountRolesMapping` (r:0 w:1)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn update_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1445`
		//  Estimated: `6944`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(33_000_000, 6944)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Roles::Ledger` (r:1 w:1)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Jobs::ValidatorJobIdLookup` (r:1 w:0)
	/// Proof: `Jobs::ValidatorJobIdLookup` (`max_values`: None, `max_size`: Some(1042), added: 3517, mode: `MaxEncodedLen`)
	/// Storage: `Roles::AccountRolesMapping` (r:0 w:1)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1100`
		//  Estimated: `6944`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 6944)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Roles::AccountRolesMapping` (r:1 w:0)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Validators` (r:1 w:1)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CounterForValidators` (r:1 w:1)
	/// Proof: `Staking::CounterForValidators` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BagsList::ListNodes` (r:3 w:3)
	/// Proof: `BagsList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
	/// Storage: `BagsList::CounterForListNodes` (r:1 w:1)
	/// Proof: `BagsList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Nominators` (r:1 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `MaxEncodedLen`)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2204`
		//  Estimated: `8877`
		// Minimum execution time: 47_000_000 picoseconds.
		Weight::from_parts(48_000_000, 8877)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Roles::AccountRolesMapping` (r:1 w:0)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:1)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Nominators` (r:1 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::MinValidatorBond` (r:1 w:0)
	/// Proof: `Staking::MinValidatorBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Bonded` (r:1 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `BagsList::ListNodes` (r:1 w:1)
	/// Proof: `BagsList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
	fn unbond_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2632`
		//  Estimated: `4764`
		// Minimum execution time: 62_000_000 picoseconds.
		Weight::from_parts(63_000_000, 4764)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Roles::AccountRolesMapping` (r:1 w:0)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Roles::Ledger` (r:1 w:0)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:1)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Bonded` (r:1 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn withdraw_unbonded() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1749`
		//  Estimated: `6944`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(41_000_000, 6944)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Roles::Ledger` (r:1 w:1)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Roles::MinRestakingBond` (r:1 w:0)
	/// Proof: `Roles::MinRestakingBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Roles::AccountRolesMapping` (r:0 w:1)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn create_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1654`
		//  Estimated: `6944`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 6944)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Roles::Ledger` (r:1 w:1)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Roles::MinRestakingBond` (r:1 w:0)
	/// Proof: `Roles::MinRestakingBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Jobs::ValidatorJobIdLookup` (r:1 w:0)
	/// Proof: `Jobs::ValidatorJobIdLookup` (`max_values`: None, `max_size`: Some(1042), added: 3517, mode: `MaxEncodedLen`)
	/// Storage: `Roles::AccountRolesMapping` (r:0 w:1)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn update_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1445`
		//  Estimated: `6944`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(33_000_000, 6944)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Roles::Ledger` (r:1 w:1)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Jobs::ValidatorJobIdLookup` (r:1 w:0)
	/// Proof: `Jobs::ValidatorJobIdLookup` (`max_values`: None, `max_size`: Some(1042), added: 3517, mode: `MaxEncodedLen`)
	/// Storage: `Roles::AccountRolesMapping` (r:0 w:1)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1100`
		//  Estimated: `6944`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 6944)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Roles::AccountRolesMapping` (r:1 w:0)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Validators` (r:1 w:1)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CounterForValidators` (r:1 w:1)
	/// Proof: `Staking::CounterForValidators` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BagsList::ListNodes` (r:3 w:3)
	/// Proof: `BagsList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
	/// Storage: `BagsList::CounterForListNodes` (r:1 w:1)
	/// Proof: `BagsList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Nominators` (r:1 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `MaxEncodedLen`)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2204`
		//  Estimated: `8877`
		// Minimum execution time: 47_000_000 picoseconds.
		Weight::from_parts(48_000_000, 8877)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Roles::AccountRolesMapping` (r:1 w:0)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:1)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Nominators` (r:1 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::MinValidatorBond` (r:1 w:0)
	/// Proof: `Staking::MinValidatorBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Bonded` (r:1 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `BagsList::ListNodes` (r:1 w:1)
	/// Proof: `BagsList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
	fn unbond_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2632`
		//  Estimated: `4764`
		// Minimum execution time: 62_000_000 picoseconds.
		Weight::from_parts(63_000_000, 4764)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Roles::AccountRolesMapping` (r:1 w:0)
	/// Proof: `Roles::AccountRolesMapping` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Roles::Ledger` (r:1 w:0)
	/// Proof: `Roles::Ledger` (`max_values`: None, `max_size`: Some(3479), added: 5954, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1 w:1)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1075), added: 3550, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Bonded` (r:1 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn withdraw_unbonded() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1749`
		//  Estimated: `6944`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(41_000_000, 6944)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
