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

use sp_runtime::DispatchResult;
use tangle_primitives::misbehavior::{
	dfns_cggmp21::{DfnsCGGMP21Justification, KeyRefreshAborted, KeygenAborted, SigningAborted},
	MisbehaviorSubmission,
};

use crate::*;

pub mod aux_only;
mod hashing_rng;
mod integer;
pub mod keygen;
pub mod sign;

pub type DefaultDigest = sha2::Sha256;
pub use integer::Integer;

/// Hardcoded value for parameter $m$ of security level
///
/// Currently, [security parameter $m$](SecurityLevel::M) is hardcoded to this constant. We're going
/// to fix that once `feature(generic_const_exprs)` is stable.
pub const M: usize = 128;
pub const SECURITY_BITS: usize = 384;
pub const SECURITY_BYTES: usize = SECURITY_BITS / 8;
pub const EPSILON: usize = 230;
pub const ELL: usize = 2 * M;
pub const ELL_PRIME: usize = 848;

impl<T: Config> Pallet<T> {
	/// Verifies a given a misbehavior justification and dispatches to specific verification logic
	pub fn verify_dfns_cggmp21_misbehavior(
		data: &MisbehaviorSubmission,
		justification: &DfnsCGGMP21Justification,
	) -> DispatchResult {
		match justification {
			DfnsCGGMP21Justification::Keygen { participants, t, reason } =>
				Self::verify_dfns_cggmp21_keygen_misbehavior(data, participants, *t, reason),
			DfnsCGGMP21Justification::KeyRefresh { participants, t, reason } =>
				Self::verify_dfns_cggmp21_key_refresh_misbehavior(data, participants, *t, reason),
			DfnsCGGMP21Justification::Signing { participants, t, reason } =>
				Self::verify_dfns_cggmp21_signing_misbehavior(data, participants, *t, reason),
		}
	}

	/// given a keygen misbehavior justification, verify the misbehavior and return a dispatch
	/// result
	pub fn verify_dfns_cggmp21_keygen_misbehavior(
		data: &MisbehaviorSubmission,
		participants: &[[u8; 33]],
		t: u16,
		reason: &KeygenAborted,
	) -> DispatchResult {
		match reason {
			KeygenAborted::InvalidDecommitment { round1, round2a } =>
				Self::verify_dfns_cggmp21_keygen_invalid_decommitment(data, round1, round2a),
			KeygenAborted::InvalidDataSize { round2a } =>
				Self::verify_dfns_cggmp21_keygen_invalid_data_size(data, t, round2a),
			KeygenAborted::FeldmanVerificationFailed { round2a, round2b } =>
				Self::verify_dfns_cggmp21_keygen_feldman(data, round2a, round2b),
			KeygenAborted::InvalidSchnorrProof { round2a, round3 } =>
				Self::verify_dfns_cggmp21_schnorr_proof(data, participants, round2a, round3),
		}
	}

	/// given a key refresh misbehavior justification, verify the misbehavior and return a dispatch
	/// result
	pub fn verify_dfns_cggmp21_key_refresh_misbehavior(
		data: &MisbehaviorSubmission,
		_participants: &[[u8; 33]],
		_t: u16,
		reason: &KeyRefreshAborted,
	) -> DispatchResult {
		match reason {
			KeyRefreshAborted::InvalidDecommitment { round1, round2 } =>
				Self::verify_dfns_cggmp21_key_refresh_invalid_decommitment(data, round1, round2),
			_ => unimplemented!(),
		}
	}
	/// given a signing misbehavior justification, verify the misbehavior and return a dispatch
	/// result
	pub fn verify_dfns_cggmp21_signing_misbehavior(
		_data: &MisbehaviorSubmission,
		_participants: &[[u8; 33]],
		_t: u16,
		_reason: &SigningAborted,
	) -> DispatchResult {
		unimplemented!()
	}
}
