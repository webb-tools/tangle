// This file is part of Tangle.
// Copyright (C) 2022-2024 Webb Technologies Inc.
//
// Tangle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Tangle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Tangle.  If not, see <http://www.gnu.org/licenses/>.
use super::*;
use crate::{types::DelegatorStatus, CurrentRound, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::ArithmeticError;

// helper function
pub fn create_and_mint_tokens(
	asset_id: AssetId,
	recipient: <Test as frame_system::Config>::AccountId,
	amount: Balance,
) {
	assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, false, 1));
	assert_ok!(Assets::mint(RuntimeOrigin::signed(1), asset_id, recipient, amount));
}

pub fn mint_tokens(
	owner: <Test as frame_system::Config>::AccountId,
	asset_id: AssetId,
	recipient: <Test as frame_system::Config>::AccountId,
	amount: Balance,
) {
	assert_ok!(Assets::mint(RuntimeOrigin::signed(owner), asset_id, recipient, amount));
}

#[test]
fn deposit_should_work_for_fungible_asset() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let amount = 200;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), VDOT, amount,));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert_eq!(metadata.deposits.get(&VDOT), Some(&amount));
		assert_eq!(
			System::events().last().unwrap().event,
			RuntimeEvent::MultiAssetDelegation(crate::Event::Deposited {
				who,
				amount,
				asset_id: VDOT,
			})
		);
	});
}

#[test]
fn multiple_deposit_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let amount = 200;

		create_and_mint_tokens(VDOT, who, amount * 4);

		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), VDOT, amount,));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert_eq!(metadata.deposits.get(&VDOT), Some(&amount));
		assert_eq!(
			System::events().last().unwrap().event,
			RuntimeEvent::MultiAssetDelegation(crate::Event::Deposited {
				who,
				amount,
				asset_id: VDOT,
			})
		);

		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), VDOT, amount));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert_eq!(metadata.deposits.get(&VDOT), Some(&amount * 2).as_ref());
		assert_eq!(
			System::events().last().unwrap().event,
			RuntimeEvent::MultiAssetDelegation(crate::Event::Deposited {
				who,
				amount,
				asset_id: VDOT,
			})
		);
	});
}

#[test]
fn deposit_should_fail_for_insufficient_balance() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let amount = 2000;

		create_and_mint_tokens(VDOT, who, 100);

		assert_noop!(
			MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), VDOT, amount,),
			ArithmeticError::Underflow
		);
	});
}

#[test]
fn deposit_should_fail_for_bond_too_low() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let amount = 50; // Below the minimum stake amount

		create_and_mint_tokens(VDOT, who, amount);

		assert_noop!(
			MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), VDOT, amount,),
			Error::<Test>::BondTooLow
		);
	});
}

#[test]
fn schedule_withdraw_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));

		assert_ok!(MultiAssetDelegation::schedule_withdraw(
			RuntimeOrigin::signed(who),
			asset_id,
			amount,
		));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert_eq!(metadata.deposits.get(&asset_id), None);
		assert!(!metadata.withdraw_requests.is_empty());
		let request = metadata.withdraw_requests.first().unwrap();
		assert_eq!(request.asset_id, asset_id);
		assert_eq!(request.amount, amount);
	});
}

#[test]
fn schedule_withdraw_should_fail_if_not_delegator() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		assert_noop!(
			MultiAssetDelegation::schedule_withdraw(RuntimeOrigin::signed(who), asset_id, amount,),
			Error::<Test>::NotDelegator
		);
	});
}

#[test]
fn schedule_withdraw_should_fail_for_insufficient_balance() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 200;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, 100,));

		assert_noop!(
			MultiAssetDelegation::schedule_withdraw(RuntimeOrigin::signed(who), asset_id, amount,),
			Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn schedule_withdraw_should_fail_if_withdraw_request_exists() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));

		// Schedule the first withdraw
		assert_ok!(MultiAssetDelegation::schedule_withdraw(
			RuntimeOrigin::signed(who),
			asset_id,
			amount,
		));
	});
}

#[test]
fn execute_withdraw_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit and schedule withdraw first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::schedule_withdraw(
			RuntimeOrigin::signed(who),
			asset_id,
			amount,
		));

		// Simulate round passing
		let current_round = 1;
		<CurrentRound<Test>>::put(current_round);

		assert_ok!(MultiAssetDelegation::execute_withdraw(RuntimeOrigin::signed(who),));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who);
		assert!(metadata.unwrap().withdraw_requests.is_empty());

		// Check event
		System::assert_last_event(RuntimeEvent::MultiAssetDelegation(
			crate::Event::Executedwithdraw { who },
		));
	});
}

#[test]
fn execute_withdraw_should_fail_if_not_delegator() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;

		assert_noop!(
			MultiAssetDelegation::execute_withdraw(RuntimeOrigin::signed(who),),
			Error::<Test>::NotDelegator
		);
	});
}

#[test]
fn execute_withdraw_should_fail_if_no_withdraw_request() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));

		assert_noop!(
			MultiAssetDelegation::execute_withdraw(RuntimeOrigin::signed(who),),
			Error::<Test>::NowithdrawRequests
		);
	});
}

#[test]
fn execute_withdraw_should_fail_if_withdraw_not_ready() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit and schedule withdraw first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::schedule_withdraw(
			RuntimeOrigin::signed(who),
			asset_id,
			amount,
		));

		// Simulate round passing but not enough
		let current_round = 0;
		<CurrentRound<Test>>::put(current_round);

		// should not actually withdraw anything
		assert_ok!(MultiAssetDelegation::execute_withdraw(RuntimeOrigin::signed(who),));

		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(!metadata.withdraw_requests.is_empty());
	});
}

#[test]
fn cancel_withdraw_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit and schedule withdraw first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::schedule_withdraw(
			RuntimeOrigin::signed(who),
			asset_id,
			amount,
		));

		assert_ok!(MultiAssetDelegation::cancel_withdraw(
			RuntimeOrigin::signed(who),
			asset_id,
			amount
		));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(metadata.withdraw_requests.is_empty());
		assert_eq!(metadata.deposits.get(&asset_id), Some(&amount));
		assert_eq!(metadata.status, DelegatorStatus::Active);

		// Check event
		System::assert_last_event(RuntimeEvent::MultiAssetDelegation(
			crate::Event::Cancelledwithdraw { who },
		));
	});
}

#[test]
fn cancel_withdraw_should_fail_if_not_delegator() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;

		assert_noop!(
			MultiAssetDelegation::cancel_withdraw(RuntimeOrigin::signed(who), 1, 1),
			Error::<Test>::NotDelegator
		);
	});
}

#[test]
fn cancel_withdraw_should_fail_if_no_withdraw_request() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, 100);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));

		assert_noop!(
			MultiAssetDelegation::cancel_withdraw(RuntimeOrigin::signed(who), asset_id, amount),
			Error::<Test>::NoMatchingwithdrawRequest
		);
	});
}
