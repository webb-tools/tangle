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

use crate::{jobs::JobId, roles::RoleType};

use frame_support::pallet_prelude::*;
use sp_core::RuntimeDebug;

pub mod dfns_cggmp21;
pub mod traits;

pub use traits::*;

/// Represents a Misbehavior submission
#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub struct MisbehaviorSubmission<AccountId> {
	/// The role type of the misbehaving node
	pub role_type: RoleType,
	/// The misbehaving node.
	pub offender: AccountId,
	/// The current Job id.
	pub job_id: JobId,
	/// The justification for the misbehavior
	pub justification: MisbehaviorJustification,
}

/// Represents a Misbehavior Justification kind
#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub enum MisbehaviorJustification {
	DKGTSS(DKGTranscriptSubmissionJustification),
	ZkSaaS(ZkSaaSSubmissionJustification),
}

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub enum DKGTranscriptSubmissionJustification {
	/// dfns CGGMP21 Implementation-specific justification
	DfnsCGGMP21(dfns_cggmp21::DfnsCGGMP21Justification),
}

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub enum ZkSaaSSubmissionJustification {}
