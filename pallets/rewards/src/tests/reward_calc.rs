use super::*;
use crate::ApyBlocks;
use frame_support::assert_ok;
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_balances::TotalIssuance;
use sp_runtime::Percent;
use tangle_primitives::types::rewards::{LockInfo, LockMultiplier, UserDepositWithLocks};

// Helper function to setup test environment with consistent values
fn setup_test_env() {
	ApyBlocks::<Runtime>::put(BLOCKS_PER_YEAR); // ~6 second blocks = ~1 year
	System::set_block_number(1000); // Set current block to 1000
}

// Mock values for consistent testing
const MOCK_DEPOSIT_CAP: u128 = 1_000_000_000_000_000_000_000_000; // 1M tokens with 18 decimals
const MOCK_TOTAL_ISSUANCE: u128 = 100_000_000_000_000_000_000_000_000; // 100M tokens with 18 decimals
const MOCK_INCENTIVE_CAP: u128 = 10_000_000_000_000_000_000_000; // 10k tokens with 18 decimals
const MOCK_APY: u8 = 10; // 10% APY
const MOCK_DEPOSIT: u128 = 100_000_000_000_000_000_000_000; // 100k tokens with 18 decimals
const BLOCKS_PER_YEAR: u32 = 5_256_000; // ~6 second blocks = ~1 year

#[test]
fn test_calculate_rewards_zero_deposit() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = 0;
		let total_asset_score = 0;
		let deposit = UserDepositWithLocks { unlocked_amount: 0, amount_with_locks: None };
		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: MOCK_DEPOSIT_CAP,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: None,
		};

		let last_claim = Some((0, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		assert_ok!(result, 0);
	});
}

#[test]
fn test_calculate_rewards_only_unlocked() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = MOCK_DEPOSIT;
		let total_asset_score = MOCK_DEPOSIT;
		let user_deposit = 10_000_000_000_000_000_000; // 10k tokens with 18 decimals
		let deposit =
			UserDepositWithLocks { unlocked_amount: user_deposit, amount_with_locks: None };
		let total_issuance = TotalIssuance::<Runtime>::set(MOCK_TOTAL_ISSUANCE);

		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: MOCK_DEPOSIT_CAP,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: None,
		};

		// Use genesis block as last claim
		let last_claim = Some((0, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		// Calculate expected rewards:
		// 1. APY adjustment: 10% * (100k/1M) = 1% effective APY
		// 2. Total annual rewards = 100M * 1% = 1M tokens
		// 3. User score = 10k (unlocked amount)
		// 4. User annual reward = 1M * (10k/100k) = 100k
		// 5. Per block = 100k / 5_256_000 blocks = 0.019 tokens
		// 6. Blocks since last claim = 1000 (current) - 0 = 1000
		// 7. Total reward = 0.019 tokens per block * 1000 blocks = 19 tokens
		let expected_to_pay = 19_000_000_000_000_000_000; // 19 tokens with 18 decimals

		assert_ok!(result, expected_to_pay);
	});
}

#[test]
fn test_calculate_rewards_with_expired_lock() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = MOCK_DEPOSIT;
		let total_asset_score = MOCK_DEPOSIT * 2; // Due to lock multipliers
		let user_deposit = 10_000_000_000_000_000_000; // 10k tokens with 18 decimals
		let deposit = UserDepositWithLocks {
			unlocked_amount: user_deposit,
			amount_with_locks: Some(vec![LockInfo {
				amount: user_deposit,
				lock_multiplier: LockMultiplier::TwoMonths,
				expiry_block: 900,
			}]),
		};

		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: MOCK_DEPOSIT_CAP,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: Some(1),
		};

		let last_claim = Some((0, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		// Only unlocked amount should count since lock is expired
		let expected_to_pay = 19_000_000_000_000_000_000; // 19 tokens with 18 decimals

		assert_ok!(result, expected_to_pay);
	});
}

#[test]
fn test_calculate_rewards_with_active_locks() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = MOCK_DEPOSIT;
		let total_asset_score = MOCK_DEPOSIT * 3; // Average multiplier effect
		let user_deposit = 10_000_000_000_000_000_000; // 10k tokens with 18 decimals
		let deposit = UserDepositWithLocks {
			unlocked_amount: user_deposit,
			amount_with_locks: Some(vec![
				LockInfo {
					amount: user_deposit * 2,
					lock_multiplier: LockMultiplier::TwoMonths,
					expiry_block: 2000,
				},
				LockInfo {
					amount: user_deposit * 3,
					lock_multiplier: LockMultiplier::ThreeMonths,
					expiry_block: 2000,
				},
			]),
		};

		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: MOCK_DEPOSIT_CAP,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: Some(1),
		};

		let last_claim = Some((0, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		// Calculate expected rewards:
		// 1. Score = 10k (unlocked) + 20k (2x lock) + 30k (3x lock) = 60k
		// 2. APY adjustment: 10% * (100k/1M) = 1% effective APY
		// 3. Total annual rewards = 100M * 1% = 1M tokens
		// 4. User annual reward = 1M * (60k/100k) = 600k
		// 5. Per block = 600k / 5_256_000 blocks = 0.114 tokens
		// 6. Blocks since last claim = 1000
		let expected_to_pay = 114_000_000_000_000_000_000; // 114 tokens with 18 decimals

		assert_ok!(result, expected_to_pay);
	});
}

#[test]
fn test_calculate_rewards_with_previous_claim() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = MOCK_DEPOSIT;
		let total_asset_score = MOCK_DEPOSIT;
		let user_deposit = 10_000_000_000_000_000_000; // 10k tokens with 18 decimals
		let deposit =
			UserDepositWithLocks { unlocked_amount: user_deposit, amount_with_locks: None };
		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: MOCK_DEPOSIT_CAP,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: None,
		};

		// Set last claim to 100 blocks ago
		let last_claim = Some((900, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		// Calculate expected rewards:
		// 1. Total annual rewards = 100M * 1% = 1M tokens
		// 2. User annual reward = 1M * (10k/100k) = 100k
		// 3. Per block = 100k / 5_256_000 blocks = 0.019 tokens
		// 4. Blocks since last claim = 100
		let expected_to_pay = 1_900_000_000_000_000_000; // 1.9 tokens with 18 decimals

		assert_ok!(result, expected_to_pay);
	});
}

#[test]
fn test_calculate_rewards_zero_cap() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = MOCK_DEPOSIT;
		let total_asset_score = MOCK_DEPOSIT;
		let deposit = UserDepositWithLocks {
			unlocked_amount: 10_000_000_000_000_000_000,
			amount_with_locks: None,
		};
		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: 0,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: None,
		};

        let last_claim = Some((0, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		assert_ok!(result, 0);
	});
}

#[test]
fn test_calculate_rewards_same_block_claim() {
	new_test_ext().execute_with(|| {
		setup_test_env();

		let total_deposit = MOCK_DEPOSIT;
		let total_asset_score = MOCK_DEPOSIT;
		let user_deposit = 10_000_000_000_000_000_000; // 10k tokens with 18 decimals
		let deposit =
			UserDepositWithLocks { unlocked_amount: user_deposit, amount_with_locks: None };
		let reward = RewardConfigForAssetVault {
			apy: Percent::from_percent(MOCK_APY),
			deposit_cap: MOCK_DEPOSIT_CAP,
			incentive_cap: MOCK_INCENTIVE_CAP,
			boost_multiplier: None,
		};

		// Set last claim to current block
		let last_claim = Some((1000, 0));

		let result = RewardsPallet::<Runtime>::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_asset_score,
			deposit,
			reward,
			last_claim,
		);

		// Calculate expected rewards:
		// 1. Total annual rewards = 100M * 1% = 1M tokens
		// 2. User annual reward = 1M * (10k/100k) = 100k
		// 3. Per block = 100k / 5_256_000 blocks = 0.019 tokens
		// 4. Blocks since last claim = 0
		let expected_to_pay = 0; // 0 blocks passed

		assert_ok!(result, expected_to_pay);
	});
}
