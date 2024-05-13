// Copyright 2022-2024 Webb Tools adapated from Moonbeam
// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! traits for restaking-delegation

use crate::weights::WeightInfo;
use frame_support::{dispatch::PostDispatchInfo, pallet_prelude::Weight};
use sp_runtime::DispatchErrorWithPostInfo;

pub trait OnOperatorPayout<AccountId, Balance> {
	fn on_operator_payout(
		for_round: crate::RoundIndex,
		operator_id: AccountId,
		amount: Balance,
	) -> Weight;
}
impl<AccountId, Balance> OnOperatorPayout<AccountId, Balance> for () {
	fn on_operator_payout(
		_for_round: crate::RoundIndex,
		_operator_id: AccountId,
		_amount: Balance,
	) -> Weight {
		Weight::zero()
	}
}
/// Defines the behavior to payout the operator's reward.
pub trait PayoutOperatorReward<Runtime: crate::Config> {
	fn payout_operator_reward(
		round_index: crate::RoundIndex,
		operator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight;
}

/// Defines the default behavior for paying out the operator's reward. The amount is directly
/// deposited into the operator's account.
impl<Runtime: crate::Config> PayoutOperatorReward<Runtime> for () {
	fn payout_operator_reward(
		for_round: crate::RoundIndex,
		operator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::mint_operator_reward(for_round, operator_id, amount)
	}
}

pub trait OnInactiveOperator<Runtime: crate::Config> {
	fn on_inactive_operator(
		operator_id: Runtime::AccountId,
		round: crate::RoundIndex,
	) -> Result<Weight, DispatchErrorWithPostInfo<PostDispatchInfo>>;
}

impl<Runtime: crate::Config> OnInactiveOperator<Runtime> for () {
	fn on_inactive_operator(
		operator_id: <Runtime>::AccountId,
		_round: crate::RoundIndex,
	) -> Result<Weight, DispatchErrorWithPostInfo<PostDispatchInfo>> {
		crate::Pallet::<Runtime>::go_offline_inner(operator_id)?;
		Ok(<Runtime as crate::Config>::WeightInfo::go_offline(crate::MAX_OPERATORS))
	}
}
