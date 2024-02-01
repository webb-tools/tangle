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

use frame_support::pallet_prelude::*;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

/// Represents a Signed Round Message by the offender.
#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub struct SignedRoundMessage {
	/// Index of a party who sent the message
	pub sender: u16,
	/// Received message
	pub message: Vec<u8>,
	/// Signature of sender + message.
	///
	/// This is the signature of the message by the sender.
	///
	/// # Note
	/// sender_bytes = sender.to_be_bytes();
	/// hash = keccak256(sender_bytes + message);
	/// signature = sign(hash);
	pub signature: Vec<u8>,
}

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub enum DfnsCGGMP21Justification {
	Keygen { n: u16, t: u16, reason: KeygenAborted },
	Signing { n: u16, t: u16, reason: SigningAborted },
}

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub enum KeygenAborted {
	/// party decommitment doesn't match commitment.
	InvalidDecommitment { round1: SignedRoundMessage, round2a: SignedRoundMessage },
	/// party provided invalid schnorr proof.
	InvalidSchnorrProof { round2a: SignedRoundMessage, round2b: SignedRoundMessage },
	/// party secret share is not consistent.
	FeldmanVerificationFailed { round2a: SignedRoundMessage, round2b: SignedRoundMessage },
	/// party data size is not suitable for threshold parameters.
	InvalidDataSize { round2a: SignedRoundMessage },
}

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
pub enum SigningAborted {
	/// `pi_enc::verify(K)` failed.
	EncProofOfK,
	/// ψ, ψˆ, or ψ' proofs are invalid
	InvalidPsi,
	/// ψ'' proof is invalid.
	InvalidPsiPrimePrime,
	/// Delta != G * delta
	MismatchedDelta,
}
