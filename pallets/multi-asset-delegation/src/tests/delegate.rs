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
#![allow(clippy::all)]
use super::*;
use crate::{
	types::{
		rewards::StakePoints, DelegatorBlueprintSelection, DelegatorBond, OperatorSnapshot,
		RewardConfig, RewardConfigForAssetVault,
	},
	AssetLookupRewardVaults, AtStake, CurrentRound, Error, RewardConfigStorage,
};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::Percent;
use std::collections::BTreeMap;

#[test]
fn delegate_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		create_and_mint_tokens(VDOT, who, amount);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));

		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(metadata.deposits.get(&asset_id).is_none());
		assert_eq!(metadata.delegations.len(), 1);
		let delegation = &metadata.delegations[0];
		assert_eq!(delegation.operator, operator);
		assert_eq!(delegation.amount, amount);
		assert_eq!(delegation.asset_id, asset_id);

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 1);
		assert_eq!(operator_metadata.delegations.len(), 1);
		let operator_delegation = &operator_metadata.delegations[0];
		assert_eq!(operator_delegation.delegator, who);
		assert_eq!(operator_delegation.amount, amount);
		assert_eq!(operator_delegation.asset_id, asset_id);
	});
}

#[test]
fn schedule_delegator_unstake_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit and delegate first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));

		assert_ok!(MultiAssetDelegation::schedule_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
		));

		// Assert
		// Check the delegator metadata
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(!metadata.delegator_unstake_requests.is_empty());
		let request = &metadata.delegator_unstake_requests[0];
		assert_eq!(request.asset_id, asset_id);
		assert_eq!(request.amount, amount);

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 0);
		assert_eq!(operator_metadata.delegations.len(), 0);
	});
}

#[test]
fn execute_delegator_unstake_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit, delegate and schedule unstake first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));
		assert_ok!(MultiAssetDelegation::schedule_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
		));

		// Simulate round passing
		CurrentRound::<Test>::put(10);

		assert_ok!(MultiAssetDelegation::execute_delegator_unstake(RuntimeOrigin::signed(who),));

		// Assert
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(metadata.delegator_unstake_requests.is_empty());
		assert!(metadata.deposits.get(&asset_id).is_some());
		assert_eq!(metadata.deposits.get(&asset_id).unwrap(), &amount);
	});
}

#[test]
fn cancel_delegator_unstake_should_work() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit, delegate and schedule unstake first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));

		assert_ok!(MultiAssetDelegation::schedule_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
		));

		// Assert
		// Check the delegator metadata
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(!metadata.delegator_unstake_requests.is_empty());
		let request = &metadata.delegator_unstake_requests[0];
		assert_eq!(request.asset_id, asset_id);
		assert_eq!(request.amount, amount);

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 0);
		assert_eq!(operator_metadata.delegations.len(), 0);

		assert_ok!(MultiAssetDelegation::cancel_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount
		));

		// Assert
		// Check the delegator metadata
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(metadata.delegator_unstake_requests.is_empty());

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 1);
		assert_eq!(operator_metadata.delegations.len(), 1);
		let operator_delegation = &operator_metadata.delegations[0];
		assert_eq!(operator_delegation.delegator, who);
		assert_eq!(operator_delegation.amount, amount); // Amount added back
		assert_eq!(operator_delegation.asset_id, asset_id);
	});
}

#[test]
fn cancel_delegator_unstake_should_update_already_existing() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit, delegate and schedule unstake first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));

		assert_ok!(MultiAssetDelegation::schedule_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			10,
		));

		// Assert
		// Check the delegator metadata
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(!metadata.delegator_unstake_requests.is_empty());
		let request = &metadata.delegator_unstake_requests[0];
		assert_eq!(request.asset_id, asset_id);
		assert_eq!(request.amount, 10);

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 1);
		assert_eq!(operator_metadata.delegations.len(), 1);
		let operator_delegation = &operator_metadata.delegations[0];
		assert_eq!(operator_delegation.delegator, who);
		assert_eq!(operator_delegation.amount, amount - 10);
		assert_eq!(operator_delegation.asset_id, asset_id);

		assert_ok!(MultiAssetDelegation::cancel_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			10
		));

		// Assert
		// Check the delegator metadata
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(metadata.delegator_unstake_requests.is_empty());

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 1);
		assert_eq!(operator_metadata.delegations.len(), 1);
		let operator_delegation = &operator_metadata.delegations[0];
		assert_eq!(operator_delegation.delegator, who);
		assert_eq!(operator_delegation.amount, amount); // Amount added back
		assert_eq!(operator_delegation.asset_id, asset_id);
	});
}

#[test]
fn delegate_should_fail_if_not_enough_balance() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 10_000;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		assert_ok!(MultiAssetDelegation::deposit(
			RuntimeOrigin::signed(who),
			asset_id,
			amount - 20,
		));

		assert_noop!(
			MultiAssetDelegation::delegate(
				RuntimeOrigin::signed(who),
				operator,
				asset_id,
				amount,
				Default::default()
			),
			Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn schedule_delegator_unstake_should_fail_if_no_delegation() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));

		assert_noop!(
			MultiAssetDelegation::schedule_delegator_unstake(
				RuntimeOrigin::signed(who),
				operator,
				asset_id,
				amount,
			),
			Error::<Test>::NoActiveDelegation
		);
	});
}

#[test]
fn execute_delegator_unstake_should_fail_if_not_ready() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, who, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit, delegate and schedule unstake first
		assert_ok!(MultiAssetDelegation::deposit(RuntimeOrigin::signed(who), asset_id, amount,));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));

		assert_noop!(
			MultiAssetDelegation::cancel_delegator_unstake(
				RuntimeOrigin::signed(who),
				operator,
				asset_id,
				amount
			),
			Error::<Test>::NoBondLessRequest
		);

		assert_ok!(MultiAssetDelegation::schedule_delegator_unstake(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
		));

		assert_noop!(
			MultiAssetDelegation::execute_delegator_unstake(RuntimeOrigin::signed(who),),
			Error::<Test>::BondLessNotReady
		);
	});
}

#[test]
fn delegate_should_not_create_multiple_on_repeat_delegation() {
	new_test_ext().execute_with(|| {
		// Arrange
		let who = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;
		let additional_amount = 50;

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		create_and_mint_tokens(VDOT, who, amount + additional_amount);

		// Deposit first
		assert_ok!(MultiAssetDelegation::deposit(
			RuntimeOrigin::signed(who),
			asset_id,
			amount + additional_amount,
		));

		// Delegate first time
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			amount,
			Default::default()
		));

		// Assert first delegation
		let metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(metadata.deposits.get(&asset_id).is_some());
		assert_eq!(metadata.delegations.len(), 1);
		let delegation = &metadata.delegations[0];
		assert_eq!(delegation.operator, operator);
		assert_eq!(delegation.amount, amount);
		assert_eq!(delegation.asset_id, asset_id);

		// Check the operator metadata
		let operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(operator_metadata.delegation_count, 1);
		assert_eq!(operator_metadata.delegations.len(), 1);
		let operator_delegation = &operator_metadata.delegations[0];
		assert_eq!(operator_delegation.delegator, who);
		assert_eq!(operator_delegation.amount, amount);
		assert_eq!(operator_delegation.asset_id, asset_id);

		// Delegate additional amount
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(who),
			operator,
			asset_id,
			additional_amount,
			Default::default()
		));

		// Assert updated delegation
		let updated_metadata = MultiAssetDelegation::delegators(who).unwrap();
		assert!(updated_metadata.deposits.get(&asset_id).is_none());
		assert_eq!(updated_metadata.delegations.len(), 1);
		let updated_delegation = &updated_metadata.delegations[0];
		assert_eq!(updated_delegation.operator, operator);
		assert_eq!(updated_delegation.amount, amount + additional_amount);
		assert_eq!(updated_delegation.asset_id, asset_id);

		// Check the updated operator metadata
		let updated_operator_metadata = MultiAssetDelegation::operator_info(operator).unwrap();
		assert_eq!(updated_operator_metadata.delegation_count, 1);
		assert_eq!(updated_operator_metadata.delegations.len(), 1);
		let updated_operator_delegation = &updated_operator_metadata.delegations[0];
		assert_eq!(updated_operator_delegation.delegator, who);
		assert_eq!(updated_operator_delegation.amount, amount + additional_amount);
		assert_eq!(updated_operator_delegation.asset_id, asset_id);
	});
}

#[test]
fn distribute_rewards_should_work() {
	new_test_ext().execute_with(|| {
		let round = 1;
		let operator = 1;
		let delegator = 2;
		let asset_id = 1;
		let amount = 100;
		let cap = 50;
		let apy = Percent::from_percent(10); // 10%

		// Set up reward configuration
		let reward_config = RewardConfig {
			configs: {
				let mut map = BTreeMap::new();
				map.insert(
					asset_id,
					RewardConfigForAssetVault { apy, cap, tnt_boost_multiplier: 1 },
				);
				map
			},
			whitelisted_blueprint_ids: vec![],
			lock_multipliers: vec![],
		};
		RewardConfigStorage::<Test>::put(reward_config);

		// Set up asset vault lookup
		AssetLookupRewardVaults::<Test>::insert(asset_id, asset_id);

		// Set up stake points for the delegator
		crate::StakePoints::<Test>::insert(
			delegator,
			asset_id,
			StakePoints {
				base_points: amount,
				lock_multiplier: 1,
				expiry: 1000, // Set a future expiry
				auto_compound: false,
			},
		);

		// Add delegation information
		AtStake::<Test>::insert(
			round,
			operator,
			OperatorSnapshot {
				delegations: vec![DelegatorBond { delegator, amount, asset_id }]
					.try_into()
					.unwrap(),
				stake: amount,
			},
		);

		// Distribute rewards
		assert_ok!(MultiAssetDelegation::distribute_rewards(round));

		// Check if rewards were stored correctly in PendingRewards
		let pending_rewards = MultiAssetDelegation::pending_rewards(&delegator, &asset_id);

		// Calculate expected rewards based on APY
		let expected_reward = apy.mul_floor(amount);

		assert_eq!(pending_rewards, expected_reward);

		// Also verify the total unclaimed rewards
		assert_eq!(MultiAssetDelegation::total_unclaimed_rewards(&asset_id), expected_reward);
	});
}

#[test]
fn distribute_rewards_with_multiple_delegators_and_operators_should_work() {
	new_test_ext().execute_with(|| {
		let round = 1;

		let operator1 = 1;
		let operator2 = 2;
		let delegator1 = 3;
		let delegator2 = 4;

		let asset_id1 = 1;
		let asset_id2 = 2;

		let amount1 = 100;
		let amount2 = 200;

		let cap1 = 50;
		let cap2 = 150;

		let apy1 = Percent::from_percent(10); // 10%
		let apy2 = Percent::from_percent(20); // 20%

		// Set up reward configuration
		let reward_config = RewardConfig {
			configs: {
				let mut map = BTreeMap::new();
				map.insert(
					asset_id1,
					RewardConfigForAssetVault { apy: apy1, cap: cap1, tnt_boost_multiplier: 1 },
				);
				map.insert(
					asset_id2,
					RewardConfigForAssetVault { apy: apy2, cap: cap2, tnt_boost_multiplier: 1 },
				);
				map
			},
			whitelisted_blueprint_ids: vec![],
			lock_multipliers: vec![],
		};
		RewardConfigStorage::<Test>::put(reward_config);

		// Set up asset vault lookup
		AssetLookupRewardVaults::<Test>::insert(asset_id1, asset_id1);
		AssetLookupRewardVaults::<Test>::insert(asset_id2, asset_id2);

		// Set up stake points for both delegators
		crate::StakePoints::<Test>::insert(
			delegator1,
			asset_id1,
			StakePoints {
				base_points: amount1,
				lock_multiplier: 1,
				expiry: 1000, // Set a future expiry
				auto_compound: false,
			},
		);

		crate::StakePoints::<Test>::insert(
			delegator2,
			asset_id2,
			StakePoints {
				base_points: amount2,
				lock_multiplier: 1,
				expiry: 1000, // Set a future expiry
				auto_compound: false,
			},
		);

		// Add delegation information
		AtStake::<Test>::insert(
			round,
			operator1,
			OperatorSnapshot {
				delegations: vec![DelegatorBond {
					delegator: delegator1,
					amount: amount1,
					asset_id: asset_id1,
				}]
				.try_into()
				.unwrap(),
				stake: amount1,
			},
		);

		AtStake::<Test>::insert(
			round,
			operator2,
			OperatorSnapshot {
				delegations: vec![DelegatorBond {
					delegator: delegator2,
					amount: amount2,
					asset_id: asset_id2,
				}]
				.try_into()
				.unwrap(),
				stake: amount2,
			},
		);

		// Distribute rewards
		assert_ok!(MultiAssetDelegation::distribute_rewards(round));

		// Check if rewards were stored correctly in PendingRewards
		let pending_rewards1 = MultiAssetDelegation::pending_rewards(&delegator1, &asset_id1);
		let pending_rewards2 = MultiAssetDelegation::pending_rewards(&delegator2, &asset_id2);

		// Calculate expected rewards based on APY
		let expected_reward1 = apy1.mul_floor(amount1); // 10% of 100 = 10
		let expected_reward2 = apy2.mul_floor(amount2); // 20% of 200 = 40

		assert_eq!(pending_rewards1, expected_reward1);
		assert_eq!(pending_rewards2, expected_reward2);

		// Also verify the total unclaimed rewards
		assert_eq!(MultiAssetDelegation::total_unclaimed_rewards(&asset_id1), expected_reward1);
		assert_eq!(MultiAssetDelegation::total_unclaimed_rewards(&asset_id2), expected_reward2);
	});
}

#[test]
fn delegator_can_add_blueprints() {
	new_test_ext().execute_with(|| {
		let delegator = 1;
		let blueprint_id = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, delegator, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit, delegate and schedule unstake first
		assert_ok!(MultiAssetDelegation::deposit(
			RuntimeOrigin::signed(delegator),
			asset_id,
			amount,
		));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(delegator),
			operator,
			asset_id,
			amount,
			DelegatorBlueprintSelection::Fixed(vec![200].try_into().unwrap()),
		));

		// Add a blueprint
		assert_ok!(MultiAssetDelegation::add_blueprint_id(
			RuntimeOrigin::signed(delegator),
			blueprint_id
		));

		// Verify the blueprint was added
		let metadata = Delegators::<Test>::get(delegator).unwrap();
		assert!(metadata.delegations.iter().any(|d| match d.blueprint_selection {
			DelegatorBlueprintSelection::Fixed(ref blueprints) => {
				blueprints.contains(&blueprint_id)
			},
			_ => false,
		}));

		// Try to add the same blueprint again
		assert_noop!(
			MultiAssetDelegation::add_blueprint_id(RuntimeOrigin::signed(delegator), blueprint_id),
			Error::<Test>::DuplicateBlueprintId
		);
	});
}

#[test]
fn delegator_can_remove_blueprints() {
	new_test_ext().execute_with(|| {
		let delegator = 1;
		let blueprint_id = 1;
		let operator = 2;
		let asset_id = VDOT;
		let amount = 100;

		create_and_mint_tokens(VDOT, delegator, amount);

		assert_ok!(MultiAssetDelegation::join_operators(RuntimeOrigin::signed(operator), 10_000));

		// Deposit, delegate and schedule unstake first
		assert_ok!(MultiAssetDelegation::deposit(
			RuntimeOrigin::signed(delegator),
			asset_id,
			amount,
		));
		assert_ok!(MultiAssetDelegation::delegate(
			RuntimeOrigin::signed(delegator),
			operator,
			asset_id,
			amount,
			DelegatorBlueprintSelection::Fixed(vec![blueprint_id].try_into().unwrap()),
		));

		// Verify it was added
		let metadata = Delegators::<Test>::get(delegator).unwrap();
		assert!(metadata.delegations.iter().any(|d| match d.blueprint_selection {
			DelegatorBlueprintSelection::Fixed(ref blueprints) => {
				blueprints.contains(&blueprint_id)
			},
			_ => false,
		}));

		// Remove the blueprint
		assert_ok!(MultiAssetDelegation::remove_blueprint_id(
			RuntimeOrigin::signed(delegator),
			blueprint_id
		));

		// Verify it was removed
		let metadata = Delegators::<Test>::get(delegator).unwrap();
		assert!(metadata.delegations.iter().all(|d| match d.blueprint_selection {
			DelegatorBlueprintSelection::Fixed(ref blueprints) => {
				!blueprints.contains(&blueprint_id)
			},
			_ => true,
		}));

		// Try to remove a non-existent blueprint
		assert_noop!(
			MultiAssetDelegation::remove_blueprint_id(
				RuntimeOrigin::signed(delegator),
				blueprint_id
			),
			Error::<Test>::BlueprintIdNotFound
		);
	});
}
