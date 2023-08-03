// Copyright 2022 Webb Technologies Inc.
//
// This file is part of pallet-evm-precompile-staking package, originally developed by Purestake
// Inc. Pallet-evm-precompile-staking package used in Tangle Network in terms of GPLv3.

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

use crate::mock::{new_test_ext, PCall, Precompiles, PrecompilesValue, Runtime, TestAccount};
use precompile_utils::testing::*;
use sp_core::H160;

fn precompiles() -> Precompiles<Runtime> {
	PrecompilesValue::get()
}

#[test]
fn max_validator_count_works() {
	new_test_ext(vec![1, 2, 3, 4]).execute_with(|| {
		precompiles()
			.prepare_test(
				TestAccount::Alex,
				H160::from_low_u64_be(5),
				PCall::max_validator_count {},
			)
			.expect_cost(0) // TODO: Test db read/write costs
			.expect_no_logs()
			.execute_returns(5u32)
	});
}

#[test]
fn current_era_works() {
	new_test_ext(vec![1, 2, 3, 4]).execute_with(|| {
		precompiles()
			.prepare_test(TestAccount::Alex, H160::from_low_u64_be(5), PCall::current_era {})
			.expect_cost(0) // TODO: Test db read/write costs
			.expect_no_logs()
			.execute_returns(0u32);
	});
}

#[test]
fn validator_count_works() {
	new_test_ext(vec![1, 2, 3, 4]).execute_with(|| {
		precompiles()
			.prepare_test(TestAccount::Alex, H160::from_low_u64_be(5), PCall::validator_count {})
			.expect_cost(0) // TODO: Test db read/write costs
			.expect_no_logs()
			.execute_returns(2u32);
	});
}

#[test]
fn max_nominator_count_works() {
	new_test_ext(vec![1, 2, 3, 4]).execute_with(|| {
		precompiles()
			.prepare_test(
				TestAccount::Alex,
				H160::from_low_u64_be(5),
				PCall::max_nominator_count {},
			)
			.expect_cost(0) // TODO: Test db read/write costs
			.expect_no_logs()
			.execute_returns(5u32);
	});
}

#[test]
fn is_validator_works() {
	new_test_ext(vec![1, 2, 3, 4]).execute_with(|| {
		precompiles()
			.prepare_test(
				TestAccount::Alex,
				H160::from_low_u64_be(5),
				PCall::is_validator { validator: H160::from(TestAccount::Alex).into() },
			)
			.expect_cost(0) // TODO: Test db read/write costs
			.expect_no_logs()
			.execute_returns(true);
	});
}
