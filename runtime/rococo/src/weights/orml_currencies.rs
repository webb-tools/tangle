
//! Autogenerated weights for `orml_currencies`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tangle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --log=warn
// --pallet=orml-currencies
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./runtime/src/weights/orml_currencies.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `orml_currencies`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_currencies::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn transfer_non_native_currency() -> Weight {
		Weight::from_ref_time(45_000_000)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn transfer_native_currency() -> Weight {
		Weight::from_ref_time(40_619_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_balance_non_native_currency() -> Weight {
		Weight::from_ref_time(32_000_000)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn update_balance_native_currency_creating() -> Weight {
		Weight::from_ref_time(24_748_000)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn update_balance_native_currency_killing() -> Weight {
		Weight::from_ref_time(30_170_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
