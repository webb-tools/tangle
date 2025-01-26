// This file is part of Tangle.
// Copyright (C) 2022-2024 Tangle Foundation.
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
use crate::RewardVaultsPotAccount;
use crate::SubaccountType;
use crate::{
	ApyBlocks, AssetLookupRewardVaults, BalanceOf, Config, Error, Event, Pallet,
	RewardConfigForAssetVault, RewardConfigStorage, RewardVaults, TotalRewardVaultDeposit,
	TotalRewardVaultScore, UserClaimedReward,
};
use frame_support::traits::Get;
use frame_support::{ensure, traits::Currency};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::prelude::vec;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{
	traits::{CheckedMul, Saturating, Zero},
	DispatchError, DispatchResult, Percent, SaturatedConversion,
};
use sp_std::vec::Vec;
use tangle_primitives::{
	services::Asset, traits::MultiAssetDelegationInfo, types::rewards::UserDepositWithLocks,
};

impl<T: Config> Pallet<T> {
	pub fn remove_asset_from_vault(
		vault_id: &T::VaultId,
		asset_id: &Asset<T::AssetId>,
	) -> DispatchResult {
		// Update RewardVaults storage
		RewardVaults::<T>::try_mutate(vault_id, |maybe_assets| -> DispatchResult {
			let assets = maybe_assets.as_mut().ok_or(Error::<T>::VaultNotFound)?;

			// Ensure the asset is in the vault
			ensure!(assets.contains(asset_id), Error::<T>::AssetNotInVault);

			assets.retain(|id| id != asset_id);

			Ok(())
		})?;

		// Update AssetLookupRewardVaults storage
		AssetLookupRewardVaults::<T>::remove(asset_id);

		Ok(())
	}

	pub fn add_asset_to_vault(
		vault_id: &T::VaultId,
		asset_id: &Asset<T::AssetId>,
	) -> DispatchResult {
		// Ensure the asset is not already associated with any vault
		ensure!(
			!AssetLookupRewardVaults::<T>::contains_key(asset_id),
			Error::<T>::AssetAlreadyInVault
		);

		// Update RewardVaults storage
		RewardVaults::<T>::try_mutate(vault_id, |maybe_assets| -> DispatchResult {
			let assets = maybe_assets.get_or_insert_with(Vec::new);

			// Ensure the asset is not already in the vault
			ensure!(!assets.contains(asset_id), Error::<T>::AssetAlreadyInVault);

			assets.push(*asset_id);

			Ok(())
		})?;

		// Update AssetLookupRewardVaults storage
		AssetLookupRewardVaults::<T>::insert(asset_id, vault_id);

		Ok(())
	}

	pub fn calculate_rewards(
		account_id: &T::AccountId,
		asset: Asset<T::AssetId>,
	) -> Result<BalanceOf<T>, DispatchError> {
		// find the vault for the asset id
		// if the asset is not in a reward vault, do nothing
		let vault_id =
			AssetLookupRewardVaults::<T>::get(asset).ok_or(Error::<T>::AssetNotInVault)?;

		// lets read the user deposits from the delegation manager
		let deposit_info =
			T::DelegationManager::get_user_deposit_with_locks(&account_id.clone(), asset)
				.ok_or(Error::<T>::NoRewardsAvailable)?;

		// read the asset reward config
		let reward_config = RewardConfigStorage::<T>::get(vault_id);

		// find the total vault score
		let total_score = TotalRewardVaultScore::<T>::get(vault_id);

		// get the users last claim
		let last_claim = UserClaimedReward::<T>::get(account_id, vault_id);

		let total_deposit = TotalRewardVaultDeposit::<T>::get(vault_id);

		Self::calculate_deposit_rewards_with_lock_multiplier(
			total_deposit,
			total_score,
			deposit_info,
			reward_config.ok_or(Error::<T>::RewardConfigNotFound)?,
			last_claim,
		)
	}

	/// Calculates and pays out rewards for a given account and asset.
	///
	/// This function orchestrates the reward calculation and payout process by:
	/// 1. Finding the vault associated with the asset
	/// 2. Retrieving user deposit information including any locked amounts
	/// 3. Calculating rewards based on deposit amounts, lock periods, and APY
	///
	/// # Arguments
	/// * `account_id` - The account to calculate rewards for
	/// * `asset` - The asset to calculate rewards for
	///
	/// # Returns
	/// * `Ok(BalanceOf<T>)` - The total rewards calculated
	/// * `Err(DispatchError)` - If any of the following conditions are met:
	///   - Asset is not in a reward vault
	///   - No rewards are available for the account
	///   - Reward configuration is not found for the vault
	///   - Arithmetic overflow occurs during calculation
	///
	/// # Assumptions
	/// * The asset must be registered in a reward vault
	/// * The reward configuration must exist for the vault
	pub fn calculate_and_payout_rewards(
		account_id: &T::AccountId,
		asset: Asset<T::AssetId>,
	) -> Result<BalanceOf<T>, DispatchError> {
		// find the vault for the asset id
		// if the asset is not in a reward vault, do nothing
		let vault_id =
			AssetLookupRewardVaults::<T>::get(asset).ok_or(Error::<T>::AssetNotInVault)?;

		let rewards_to_be_paid = Self::calculate_rewards(account_id, asset)?;

		log::debug!("rewards_to_be_paid: {:?}", rewards_to_be_paid.saturated_into::<u128>());

		// mint new TNT rewards and trasnfer to the user
		let _ = T::Currency::deposit_creating(account_id, rewards_to_be_paid);

		// update the last claim
		UserClaimedReward::<T>::try_mutate(
			account_id,
			vault_id,
			|maybe_claim| -> DispatchResult {
				let current_block = frame_system::Pallet::<T>::block_number();
				let total_claimed = maybe_claim.map(|(_, amount)| amount).unwrap_or_default();
				*maybe_claim =
					Some((current_block, total_claimed.saturating_add(rewards_to_be_paid)));
				Ok(())
			},
		)?;

		Self::deposit_event(Event::RewardsClaimed {
			account: account_id.clone(),
			asset,
			amount: rewards_to_be_paid,
		});

		Ok(rewards_to_be_paid)
	}

	pub fn create_reward_vault_pot(vault_id: T::VaultId) -> Result<T::AccountId, DispatchError> {
		// Initialize the vault pot for rewards
		let pot_account_for_vault: T::AccountId =
			T::PalletId::get().into_sub_account_truncating((SubaccountType::RewardPot, vault_id));
		// Ensure the pot does not already exist
		ensure!(RewardVaultsPotAccount::<T>::get(vault_id).is_none(), Error::<T>::PotAlreadyExists);
		// Store the pot
		RewardVaultsPotAccount::<T>::insert(vault_id, pot_account_for_vault.clone());
		Ok(pot_account_for_vault)
	}

	/// Validates a reward configuration ensuring that:
	/// 1. The incentive cap is not greater than the deposit cap
	/// 2. If boost multiplier is set, it must be 1 (current limitation)
	///
	/// # Arguments
	/// * `config` - The reward configuration to validate
	///
	/// # Returns
	/// * `DispatchResult` - Ok if validation passes, Error otherwise
	pub fn validate_reward_config(
		config: &RewardConfigForAssetVault<BalanceOf<T>>,
	) -> Result<(), Error<T>> {
		ensure!(
			config.incentive_cap <= config.deposit_cap,
			Error::<T>::IncentiveCapGreaterThanDepositCap
		);

		if let Some(boost_multiplier) = config.boost_multiplier {
			// boost multipliers are handled by locks, this ensures the multiplier is 1
			// we can change the multiplier to be customisable in the future, but for now we
			// require it to be 1
			ensure!(boost_multiplier == 1, Error::<T>::BoostMultiplierMustBeOne);
		}

		Ok(())
	}

	/// Calculate the APY based on the total deposit and deposit cap.
	/// The goal is to ensure the APY is proportional to the total deposit.
	///
	/// # Returns
	/// * `Ok(Percent)` - The normalized APY
	/// * `Err(DispatchError)` - If any arithmetic operation overflows
	///
	/// # Arguments
	/// * `total_deposit` - The total amount of deposits for the asset vault
	/// * `deposit_cap` - The maximum amount of deposits allowed for the asset vault
	/// * `original_apy` - The original APY before normalization
	pub fn calculate_propotional_apy(
		total_deposit: BalanceOf<T>,
		deposit_cap: BalanceOf<T>,
		original_apy: Percent,
	) -> Option<Percent> {
		if deposit_cap.is_zero() {
			return None;
		}

		let propotion = Percent::from_rational(total_deposit, deposit_cap);
		original_apy.checked_mul(&propotion)
	}

	/// Calculate the per-block reward amount for a given total reward
	///
	/// # Arguments
	/// * `total_reward` - The total reward amount to be distributed
	///
	/// # Returns
	/// * `Option<BalanceOf<T>>` - The per-block reward amount, or None if division fails
	pub fn calculate_reward_per_block(total_reward: BalanceOf<T>) -> Option<BalanceOf<T>> {
		let apy_blocks = ApyBlocks::<T>::get();
		if apy_blocks.is_zero() {
			return None;
		}

		log::debug!("total_reward: {:?}", total_reward);

		let apy_blocks_balance = BalanceOf::<T>::from(apy_blocks.saturated_into::<u32>());
		Some(total_reward / apy_blocks_balance)
	}

	/// Calculates rewards for deposits considering both unlocked amounts and locked amounts with
	/// their respective multipliers.
	///
	/// The reward calculation follows these formulas:
	/// 1. For unlocked amounts: ```text base_reward = APY * (user_deposit / total_deposits) *
	///    (total_deposits / deposit_capacity) ```
	///
	/// 2. For locked amounts: ```text lock_reward = amount * APY * lock_multiplier *
	///    (remaining_lock_time / total_lock_time) ```
	///
	/// # Arguments
	/// * `total_asset_score` - Total score for the asset across all deposits
	/// * `deposit` - User's deposit information including locked amounts
	/// * `reward` - Reward configuration for the asset vault
	/// * `last_claim` - Block number and amount of last claim, if any
	///
	/// # Returns
	/// * `Ok((BalanceOf<T>, BalanceOf<T>))` - Tuple of (total rewards, rewards to be paid)
	/// * `Err(DispatchError)` - If any arithmetic operation fails
	///
	/// The reward amount is affected by:
	///   - The proportion of user's deposit to total deposits
	///   - The proportion of total deposits to deposit capacity
	///   - The lock multiplier (if applicable)
	///   - The remaining time in the lock period
	pub fn calculate_deposit_rewards_with_lock_multiplier(
		total_deposit: BalanceOf<T>,
		total_asset_score: BalanceOf<T>,
		deposit: UserDepositWithLocks<BalanceOf<T>, BlockNumberFor<T>>,
		reward: RewardConfigForAssetVault<BalanceOf<T>>,
		last_claim: Option<(BlockNumberFor<T>, BalanceOf<T>)>,
	) -> Result<BalanceOf<T>, DispatchError> {
		// Calculate the propotional apy
		let deposit_cap = reward.deposit_cap;

		if reward.incentive_cap > total_deposit {
			return Err(Error::<T>::TotalDepositLessThanIncentiveCap.into());
		}

		let apy = Self::calculate_propotional_apy(total_deposit, deposit_cap, reward.apy)
			.ok_or(Error::<T>::CannotCalculatePropotionalApy)?;
		log::debug!("apy: {:?}", apy);

		// Calculate total rewards pool from total issuance
		let tnt_total_supply = T::Currency::total_issuance();
		let total_annual_rewards = apy.mul_floor(tnt_total_supply);

		// Calculate per block reward pool first to minimize precision loss
		let total_reward_per_block = Self::calculate_reward_per_block(total_annual_rewards)
			.ok_or(Error::<T>::CannotCalculateRewardPerBlock)?;
		log::debug!("total_reward_per_block: {:?}", total_reward_per_block);

		// Start with unlocked amount as base score
		let user_unlocked_score = deposit.unlocked_amount;
		let user_score = user_unlocked_score;

		// Get the current block and calculate last claim block
		let current_block = frame_system::Pallet::<T>::block_number();
		let last_claim_block = last_claim.map(|(block, _)| block).unwrap_or(current_block);
		let blocks_to_be_paid = current_block.saturating_sub(last_claim_block);
		log::debug!(
			"Current Block {:?}, Last Claim Block {:?}, Blocks to be paid {:?}",
			current_block,
			last_claim_block,
			blocks_to_be_paid
		);

		log::debug!("User unlocked score {:?}", user_score);

		// array of (score, blocks)
		let mut user_rewards_score_by_blocks: Vec<(BalanceOf<T>, BlockNumberFor<T>)> = vec![];
		user_rewards_score_by_blocks.push((user_unlocked_score, blocks_to_be_paid));

		// Add score with lock multipliers if any
		// only if the admin has enabled boost multiplier for the vault
		if reward.boost_multiplier.is_some() {
			if let Some(locks) = deposit.amount_with_locks {
				for lock in locks {
					if lock.expiry_block > last_claim_block {
						if lock.expiry_block > current_block {
							// Calculate lock reward:
							// amount * APY * lock_multiplier *
							//    (remaining_lock_time / total_lock_time)
							let multiplier = BalanceOf::<T>::from(lock.lock_multiplier.value());
							let lock_score = lock.amount.saturating_mul(multiplier);
							log::debug!("user lock has not expired and still active, lock_multiplier: {:?}, lock_score: {:?}", lock.lock_multiplier, lock_score);

							user_rewards_score_by_blocks.push((lock_score, blocks_to_be_paid));
						} else {
							// the lock has expired, so we only apply the lock multiplier during the
							// unexpired period
							let multiplier = BalanceOf::<T>::from(lock.lock_multiplier.value());
							let lock_score = lock.amount.saturating_mul(multiplier);
							let multiplier_applied_blocks =
								lock.expiry_block.saturating_sub(last_claim_block);

							log::debug!("user lock has partially expired, lock_multiplier: {:?}, lock_score: {:?}, multiplier_applied_blocks: {:?}, blocks_to_be_paid: {:?}",
								lock.lock_multiplier, lock_score, multiplier_applied_blocks, blocks_to_be_paid);

							user_rewards_score_by_blocks
								.push((lock_score, multiplier_applied_blocks));

							// for rest of the blocks, we do not apply the lock multiplier
							user_rewards_score_by_blocks.push((
								lock.amount,
								blocks_to_be_paid.saturating_sub(multiplier_applied_blocks),
							));
						}
					} else {
						// if the lock has expired, we only consider the base score
						user_rewards_score_by_blocks.push((lock.amount, blocks_to_be_paid));
					}
				}
			}
		}

		log::debug!("user rewards array {:?}", user_rewards_score_by_blocks);

		// if the user has no score, return 0
		// calculate the total score for the user
		let total_score_for_user = user_rewards_score_by_blocks
			.iter()
			.fold(BalanceOf::<T>::zero(), |acc, (score, _blocks)| acc.saturating_add(*score));
		log::debug!("total score: {:?}", total_score_for_user);
		ensure!(!total_score_for_user.is_zero(), Error::<T>::NoRewardsAvailable);

		// Calculate user's proportion of rewards based on their score

		let mut total_rewards_to_be_paid_to_user = BalanceOf::<T>::zero();
		for (score, blocks) in user_rewards_score_by_blocks {
			let user_proportion = Percent::from_rational(score, total_asset_score);
			log::debug!("user_proportion: {:?}", user_proportion);
			let user_reward_per_block = user_proportion.mul_floor(total_reward_per_block);

			// Calculate total rewards for the period
			log::debug!("last_claim_block: {:?}, total_reward_per_block: {:?}, user reward per block: {:?}, blocks: {:?}", 
				last_claim_block, total_reward_per_block, user_reward_per_block, blocks);

			let rewards_to_be_paid = user_reward_per_block
				.saturating_mul(BalanceOf::<T>::from(blocks.saturated_into::<u32>()));

			log::debug!("rewards_to_be_paid: {:?}", rewards_to_be_paid);

			total_rewards_to_be_paid_to_user =
				total_rewards_to_be_paid_to_user.saturating_add(rewards_to_be_paid);
		}

		log::debug!("total_rewards_to_be_paid_to_user: {:?}", total_rewards_to_be_paid_to_user);
		Ok(total_rewards_to_be_paid_to_user)
	}
}
