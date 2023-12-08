// This file is part of Webb.
// Copyright (C) 2022 Webb Technologies Inc.
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
use crate::roles::{RoleType, RoleTypeMetadata};
use frame_support::{dispatch::Vec, pallet_prelude::*};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct Record {
	pub metadata: RoleTypeMetadata,
	pub amount: Option<u64>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct IndependentReStakeProfile {
	pub records: Vec<Record>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct SharedReStakeProfile {
	pub records: Vec<Record>,
	pub amount: u64,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum Profile {
	Independent(IndependentReStakeProfile),
	Shared(SharedReStakeProfile),
}

impl Profile {
	/// Checks if the profile is an independent profile.
	pub fn is_independent(&self) -> bool {
		matches!(self, Profile::Independent(_))
	}

	/// Checks if the profile is a shared profile.
	pub fn is_shared(&self) -> bool {
		matches!(self, Profile::Shared(_))
	}

	/// Returns the total profile stake.
	pub fn get_total_profile_stake(&self) -> u64 {
		match self {
			Profile::Independent(profile) =>
				profile.records.iter().fold(0, |acc, record| acc + record.amount.unwrap_or(0)),
			Profile::Shared(profile) => profile.amount,
		}
	}

	/// Returns staking record details containing role metadata and stake amount.
	pub fn get_records(&self) -> Vec<Record> {
		match self {
			Profile::Independent(profile) => profile.records.clone(),
			Profile::Shared(profile) => profile.records.clone(),
		}
	}

	/// Returns staking role metadata for given role.
	pub fn get_role_metadata(&self, role_type: RoleType) -> Option<RoleTypeMetadata> {
		match self {
			Profile::Independent(profile) => profile
				.records
				.iter()
				.find(|record| record.metadata.get_role_type() == role_type)
				.map(|record| record.metadata.clone()),
			Profile::Shared(profile) => profile
				.records
				.iter()
				.find(|record| record.metadata.get_role_type() == role_type)
				.map(|record| record.metadata.clone()),
		}
	}

	/// Returns roles in the profile.
	pub fn get_roles(&self) -> Vec<RoleType> {
		match self {
			Profile::Independent(profile) =>
				profile.records.iter().map(|record| record.metadata.get_role_type()).collect(),
			Profile::Shared(profile) =>
				profile.records.iter().map(|record| record.metadata.get_role_type()).collect(),
		}
	}

	/// Checks if the profile contains given role.
	pub fn has_role(&self, role_type: RoleType) -> bool {
		match self {
			Profile::Independent(profile) => profile
				.records
				.iter()
				.any(|record| record.metadata.get_role_type() == role_type),
			Profile::Shared(profile) => profile
				.records
				.iter()
				.any(|record| record.metadata.get_role_type() == role_type),
		}
	}

	/// Removes given role from the profile.
	pub fn remove_role_from_profile(&mut self, role_type: RoleType) {
		match self {
			Profile::Independent(profile) => {
				profile.records.retain(|record| record.metadata.get_role_type() != role_type);
			},
			Profile::Shared(profile) => {
				profile.records.retain(|record| record.metadata.get_role_type() != role_type);
			},
		}
	}

	/// Return roles from current profile removed in updated profile.
	pub fn get_removed_roles(&self, updated_profile: &Profile) -> Vec<RoleType> {
		// Get the roles from the current profile.
		let roles = self.get_roles();
		let updated_roles = updated_profile.get_roles();
		// Returns roles in current profile that have been removed in updated profile.
		roles
			.iter()
			.filter_map(|role| {
				let updated_role = updated_roles.iter().find(|updated_role| *updated_role == role);
				if updated_role.is_none() {
					Some(role.clone())
				} else {
					None
				}
			})
			.collect()
	}
}
