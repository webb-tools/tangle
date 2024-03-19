
//! Autogenerated weights for `pallet_airdrop_claims`
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
// --pallet=pallet-airdrop-claims
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output=./pallets/claims/src/weights.rs
// --template=./.maintain/frame-weights-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_airdrop_claims`.
pub trait WeightInfo {
	fn claim() -> Weight;
	fn mint_claim() -> Weight;
	fn claim_attest() -> Weight;
	fn move_claim() -> Weight;
	fn force_set_expiry_config() -> Weight;
}

/// Weights for `pallet_airdrop_claims` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1169), added: 3644, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `673`
		//  Estimated: `4764`
		// Minimum execution time: 157_000_000 picoseconds.
		Weight::from_parts(265_000_000, 4764)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:0 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Claims` (r:0 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:0 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn mint_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `1630`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(18_000_000, 1630)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1169), added: 3644, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn claim_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `673`
		//  Estimated: `4764`
		// Minimum execution time: 161_000_000 picoseconds.
		Weight::from_parts(252_000_000, 4764)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Claims::Claims` (r:1 w:2)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:2)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:0)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn move_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3878`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(27_000_000, 3878)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Claims::ExpiryConfig` (r:0 w:1)
	/// Proof: `Claims::ExpiryConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_set_expiry_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(2_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1169), added: 3644, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `673`
		//  Estimated: `4764`
		// Minimum execution time: 157_000_000 picoseconds.
		Weight::from_parts(265_000_000, 4764)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:0 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Claims` (r:0 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:0 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn mint_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `1630`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(18_000_000, 1630)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1169), added: 3644, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn claim_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `673`
		//  Estimated: `4764`
		// Minimum execution time: 161_000_000 picoseconds.
		Weight::from_parts(252_000_000, 4764)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Claims::Claims` (r:1 w:2)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:2)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:0)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn move_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3878`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(27_000_000, 3878)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Claims::ExpiryConfig` (r:0 w:1)
	/// Proof: `Claims::ExpiryConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_set_expiry_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(2_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
