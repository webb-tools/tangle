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
#![cfg(test)]
use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use tangle_primitives::jobs::FallbackOptions;

use pallet_roles::profile::{IndependentRestakeProfile, Profile, Record, SharedRestakeProfile};
use tangle_primitives::{
	jobs::{
		DKGTSSKeyRefreshResult, DKGTSSKeyRotationResult, DKGTSSPhaseFourJobType,
		DKGTSSPhaseOneJobType, DKGTSSPhaseThreeJobType, DKGTSSPhaseTwoJobType,
		DKGTSSSignatureResult, DigitalSignatureScheme, Groth16ProveRequest, Groth16System,
		HyperData, JobSubmission, JobType, RpcResponseJobsData, ZkSaaSCircuitResult,
		ZkSaaSPhaseOneJobType, ZkSaaSPhaseTwoJobType, ZkSaaSPhaseTwoRequest, ZkSaaSSystem,
	},
	roles::{RoleType, ThresholdSignatureRoleType, ZeroKnowledgeRoleType},
};

const ALICE: u8 = 1;
const BOB: u8 = 2;
const CHARLIE: u8 = 3;
const DAVE: u8 = 4;
const EVE: u8 = 5;

const TEN: u8 = 10;
const TWENTY: u8 = 20;
const HUNDRED: u8 = 100;

pub fn shared_profile() -> Profile<Runtime> {
	let profile = SharedRestakeProfile {
		records: BoundedVec::try_from(vec![
			Record {
				role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
				amount: None,
			},
			Record { role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16), amount: None },
		])
		.unwrap(),
		amount: 1000,
	};
	Profile::Shared(profile)
}

pub fn independent_profile() -> Profile<Runtime> {
	let profile = IndependentRestakeProfile {
		records: BoundedVec::try_from(vec![
			Record {
				role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
				amount: Some(500),
			},
			Record {
				role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
				amount: Some(500),
			},
		])
		.unwrap(),
	};
	Profile::Independent(profile)
}

#[test]
fn jobs_submission_e2e_works_for_dkg() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [HUNDRED, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};

		// should fail with invalid validator
		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(ALICE)), submission),
			Error::<Runtime>::InvalidValidator
		);

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 6,
				permitted_caller: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};

		// should fail with invalid threshold
		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(ALICE)), submission),
			Error::<Runtime>::InvalidJobParams
		);

		// should fail when caller has no balance
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};

		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission),
			sp_runtime::TokenError::FundsUnavailable
		);
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		// should work when n = t
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 5,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 5);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE].iter().map(|x| mock_pub_key(*x)) {
			assert_eq!(ValidatorRewards::<Runtime>::get(validator), Some(1));
		}

		// ensure storage is correctly setup
		assert!(KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_some());
		assert!(SubmittedJobs::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_none());

		// ---- use phase one solution in phase 2 signinig -------

		// another account cannot use solution
		let submission = JobSubmission {
			expiry: 10,
			ttl: 0,
			job_type: JobType::DKGTSSPhaseTwo(DKGTSSPhaseTwoJobType {
				phase_one_id: 0,
				submission: vec![].try_into().unwrap(),
				derivation_path: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TWENTY)), submission),
			Error::<Runtime>::InvalidJobParams
		);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 0,
			job_type: JobType::DKGTSSPhaseTwo(DKGTSSPhaseTwoJobType {
				phase_one_id: 0,
				submission: vec![].try_into().unwrap(),
				derivation_path: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 25);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(threshold_signature_role_type),
			1,
			JobResult::DKGPhaseTwo(DKGTSSSignatureResult {
				verifying_key: vec![].try_into().unwrap(),
				signature: vec![].try_into().unwrap(),
				data: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1,
				derivation_path: None,
			})
		));

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE].iter().map(|x| mock_pub_key(*x)) {
			assert_eq!(ValidatorRewards::<Runtime>::get(validator), Some(5));
		}

		// ensure storage is correctly setup
		assert!(KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_some());
		assert!(SubmittedJobs::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_none());
	});
}

#[test]
fn jobs_submission_e2e_for_dkg_refresh() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 5);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// ---- use phase one solution in phase 3 key refresh -------

		let submission = JobSubmission {
			expiry: 10,
			ttl: 0,
			job_type: JobType::DKGTSSPhaseThree(DKGTSSPhaseThreeJobType {
				phase_one_id: 0,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 25);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(threshold_signature_role_type),
			1,
			JobResult::DKGPhaseThree(DKGTSSKeyRefreshResult {
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE].iter().map(|x| mock_pub_key(*x)) {
			assert_eq!(ValidatorRewards::<Runtime>::get(validator), Some(5));
		}
	});
}

#[test]
fn jobs_submission_e2e_for_dkg_rotation() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 5);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 10);
		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			1,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// ---- use phase one solution in phase 4 key rotation -------

		let submission = JobSubmission {
			expiry: 10,
			ttl: 0,
			job_type: JobType::DKGTSSPhaseFour(DKGTSSPhaseFourJobType {
				phase_one_id: 0,
				new_phase_one_id: 1,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 30);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(threshold_signature_role_type),
			2,
			JobResult::DKGPhaseFour(DKGTSSKeyRotationResult {
				key: vec![].try_into().unwrap(),
				new_key: vec![].try_into().unwrap(),
				signature: vec![].try_into().unwrap(),
				phase_one_id: 0,
				new_phase_one_id: 1,
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1,
				derivation_path: None,
			})
		));

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE].iter().map(|x| mock_pub_key(*x)) {
			assert_eq!(ValidatorRewards::<Runtime>::get(validator), Some(6));
		}
	});
}

#[test]
fn jobs_rpc_tests() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		let stored_job =
			SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 0).unwrap();
		let expected_rpc_response = RpcResponseJobsData {
			job_id: 0,
			job_type: stored_job.job_type,
			ttl: stored_job.ttl,
			expiry: stored_job.expiry,
		};

		// query jobs by validator should work
		for validator in participants.iter().map(|x| mock_pub_key(*x)).collect::<Vec<_>>() {
			assert_eq!(
				Jobs::query_jobs_by_validator(validator),
				Some(vec![expected_rpc_response.clone()])
			);
		}

		assert_eq!(
			Jobs::query_job_by_id(RoleType::Tss(threshold_signature_role_type), 0),
			Some(expected_rpc_response)
		);
		assert_eq!(Jobs::query_next_job_id(), 1);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		assert_eq!(Jobs::query_job_by_id(RoleType::Tss(threshold_signature_role_type), 0), None);
		assert_eq!(Jobs::query_next_job_id(), 1);

		let expected_result = KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
		);
		assert_eq!(
			Jobs::query_job_result(RoleType::Tss(threshold_signature_role_type), 0),
			expected_result
		);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 0,
			job_type: JobType::DKGTSSPhaseTwo(DKGTSSPhaseTwoJobType {
				phase_one_id: 0,
				submission: vec![].try_into().unwrap(),
				derivation_path: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		let stored_job =
			SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 1).unwrap();
		let expected_rpc_response = RpcResponseJobsData {
			job_id: 1,
			job_type: stored_job.job_type,
			ttl: stored_job.ttl,
			expiry: stored_job.expiry,
		};

		assert_eq!(
			Jobs::query_job_by_id(RoleType::Tss(threshold_signature_role_type), 1),
			Some(expected_rpc_response)
		);
		assert_eq!(Jobs::query_next_job_id(), 2);

		let expected_result = KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			1,
		);
		assert_eq!(
			Jobs::query_job_result(RoleType::Tss(threshold_signature_role_type), 1),
			expected_result
		);
	});
}

#[test]
fn jobs_submission_e2e_works_for_zksaas() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		let dummy_system = ZkSaaSSystem::Groth16(Groth16System {
			circuit: HyperData::Raw(vec![].try_into().unwrap()),
			num_inputs: 0,
			num_constraints: 0,
			proving_key: HyperData::Raw(vec![].try_into().unwrap()),
			verifying_key: vec![].try_into().unwrap(),
			wasm: HyperData::Raw(vec![].try_into().unwrap()),
		});

		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::ZkSaaSPhaseOne(ZkSaaSPhaseOneJobType {
				permitted_caller: None,
				system: dummy_system.clone(),
				role_type: ZeroKnowledgeRoleType::ZkSaaSGroth16,
				participants: [HUNDRED, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
			}),
			fallback: FallbackOptions::Destroy,
		};

		// should fail with invalid validator
		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(ALICE)), submission),
			Error::<Runtime>::InvalidValidator
		);

		// should fail when caller has no balance
		let _submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::<_, _, _, MaxAdditionalParamsLen>::ZkSaaSPhaseOne(
				ZkSaaSPhaseOneJobType::<sp_runtime::AccountId32, MaxParticipants, MaxSubmissionLen> {
					permitted_caller: None,
					system: dummy_system.clone(),
					role_type: ZeroKnowledgeRoleType::ZkSaaSGroth16,
					participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
						.iter()
						.map(|x| mock_pub_key(*x))
						.collect::<Vec<_>>()
						.try_into()
						.unwrap(),
				},
			),
			fallback: FallbackOptions::Destroy,
		};

		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::ZkSaaSPhaseOne(ZkSaaSPhaseOneJobType {
				permitted_caller: Some(mock_pub_key(TEN)),
				system: dummy_system.clone(),
				role_type: ZeroKnowledgeRoleType::ZkSaaSGroth16,
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 10);

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
			0,
			JobResult::ZkSaaSPhaseOne(ZkSaaSCircuitResult {
				job_id: 0,
				participants: vec![].try_into().unwrap()
			}),
		));

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_eq!(ValidatorRewards::<Runtime>::get(mock_pub_key(validator)), Some(2));
		}

		// ensure storage is correctly setup
		assert!(KnownResults::<Runtime>::get(
			RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
			0
		)
		.is_some());
		assert!(SubmittedJobs::<Runtime>::get(
			RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
			0
		)
		.is_none());

		// ---- use phase one solution in phase 2 proving -------
		let dummy_req = ZkSaaSPhaseTwoRequest::Groth16(Groth16ProveRequest {
			public_input: vec![].try_into().unwrap(),
			a_shares: vec![].try_into().unwrap(),
			ax_shares: vec![].try_into().unwrap(),
			qap_shares: vec![].try_into().unwrap(),
		});
		// another account cannot use solution
		let submission = JobSubmission {
			expiry: 100,
			ttl: 200,
			job_type: JobType::ZkSaaSPhaseTwo(ZkSaaSPhaseTwoJobType {
				phase_one_id: 0,
				request: dummy_req.clone(),
				role_type: ZeroKnowledgeRoleType::ZkSaaSGroth16,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TWENTY)), submission),
			Error::<Runtime>::InvalidJobParams
		);

		let submission = JobSubmission {
			expiry: 100,
			ttl: 200,
			job_type: JobType::ZkSaaSPhaseTwo(ZkSaaSPhaseTwoJobType {
				phase_one_id: 0,
				request: dummy_req,
				role_type: ZeroKnowledgeRoleType::ZkSaaSGroth16,
			}),
			fallback: FallbackOptions::Destroy,
		};

		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 30);

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE]
			.iter()
			.map(|x| mock_pub_key(*x))
			.collect::<Vec<_>>()
		{
			assert_eq!(ValidatorRewards::<Runtime>::get(validator), Some(2));
		}

		// ensure storage is correctly setup
		assert!(KnownResults::<Runtime>::get(
			RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
			1
		)
		.is_none());
		assert!(SubmittedJobs::<Runtime>::get(
			RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
			1
		)
		.is_some());
	});
}

#[test]
fn reduce_active_role_restake_should_fail() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// ======= active validator cannot reduce stake ===============
		let reduced_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: None,
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: None,
				},
			])
			.unwrap(),
			amount: 500, // reduce stake by 50%
		};

		for validator in participants.clone() {
			assert_noop!(
				Roles::update_profile(
					RuntimeOrigin::signed(mock_pub_key(validator)),
					Profile::Shared(reduced_profile.clone())
				),
				pallet_roles::Error::<Runtime>::InsufficientRestakingBond
			);
		}
	});
}

#[test]
fn delete_profile_with_active_role_should_fail() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// ========= active validator cannot delete profile with active job =============
		for validator in participants.clone() {
			assert_noop!(
				Roles::delete_profile(RuntimeOrigin::signed(mock_pub_key(validator)),),
				pallet_roles::Error::<Runtime>::ProfileDeleteRequestFailed
			);
		}
	});
}

#[test]
fn remove_active_role_should_fail() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);
		Roles::set_min_restaking_bond(RuntimeOrigin::root(), Default::default()).unwrap();

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// ========= active validator cannot remove role with active job =============
		let reduced_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![Record {
				role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
				amount: None,
			}])
			.unwrap(),
			amount: 500, // reduce stake by 50%
		};
		for validator in participants.clone() {
			assert_noop!(
				Roles::update_profile(
					RuntimeOrigin::signed(mock_pub_key(validator)),
					Profile::Shared(reduced_profile.clone())
				),
				pallet_roles::Error::<Runtime>::RoleCannotBeRemoved
			);
		}
	});
}

#[test]
fn remove_role_without_active_jobs_should_work() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// =========  active validator can remove role without active job =========
		let reduced_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![Record {
				role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
				amount: None,
			}])
			.unwrap(),
			amount: 1000,
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Shared(reduced_profile.clone())
			));
		}
	});
}

#[test]
fn add_role_to_active_profile_should_work() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// =========  active validator can add a new role with current active role =========
		let updated_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: None,
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: None,
				},
			])
			.unwrap(),
			amount: 1000,
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Shared(updated_profile.clone())
			));
		}
	});
}

#[test]
fn reduce_stake_on_non_active_role_should_work() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);
		Roles::set_min_restaking_bond(RuntimeOrigin::root(), Default::default()).unwrap();

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// =========  active validator can reduce stake on non active role =========
		let updated_profile = IndependentRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: Some(1500),
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: Some(500), // reduced by 3x
				},
			])
			.unwrap(),
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Independent(updated_profile.clone())
			));
		}
	});
}

#[test]
fn increase_stake_on_active_role_should_work() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// =========  active validator can increase stake with current active role =========
		let updated_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: None,
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: None,
				},
			])
			.unwrap(),
			amount: 1500,
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Shared(updated_profile.clone())
			));
		}
	});
}

#[test]
fn switch_non_active_profile_should_work() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];
		Roles::set_min_restaking_bond(RuntimeOrigin::root(), Default::default()).unwrap();

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// =========  active validator can switch shared to independent profile =========
		let updated_profile = IndependentRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: Some(1500),
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: Some(500),
				},
			])
			.unwrap(),
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Independent(updated_profile.clone())
			));
		}

		// =========  active validator can switch independent to shared profile =========
		let updated_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: None,
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: None,
				},
			])
			.unwrap(),
			amount: 2000,
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Shared(updated_profile.clone())
			));
		}
	});
}

#[test]
fn switch_active_shared_profile_to_independent_should_work_if_active_stake_preserved() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);
		Roles::set_min_restaking_bond(RuntimeOrigin::root(), Default::default()).unwrap();

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// =========  active validator cannot switch shared to independent profile =========
		let updated_profile = IndependentRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: Some(500), // <---------- ACTIVE STAKE NOT PRESERVED
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: Some(500),
				},
			])
			.unwrap(),
		};

		for validator in participants.clone() {
			assert_noop!(
				Roles::update_profile(
					RuntimeOrigin::signed(mock_pub_key(validator)),
					Profile::Independent(updated_profile.clone())
				),
				pallet_roles::Error::<Runtime>::InsufficientRestakingBond
			);
		}

		// =========  active validator can switch shared to independent profile =========
		let updated_profile = IndependentRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: Some(1000), // <---------- ACTIVE STAKE PRESERVED
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: Some(500),
				},
			])
			.unwrap(),
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Independent(updated_profile.clone())
			));
		}
	});
}

#[test]
fn switch_active_independent_profile_to_shared_should_work_if_active_restake_sum_preserved() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);
		Roles::set_min_restaking_bond(RuntimeOrigin::root(), Default::default()).unwrap();

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet w/ independent profile
		let profile = independent_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// =========  active validator can not switch independent to shared profile =========
		let updated_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: None,
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: None,
				},
			])
			.unwrap(),
			amount: 400, // <---------- ACTIVE RESTAKE SUM NOT PRESERVED
		};

		for validator in participants.clone() {
			assert_noop!(
				Roles::update_profile(
					RuntimeOrigin::signed(mock_pub_key(validator)),
					Profile::Shared(updated_profile.clone())
				),
				pallet_roles::Error::<Runtime>::InsufficientRestakingBond
			);
		}

		// =========  active validator can switch independent to shared profile =========
		let updated_profile = SharedRestakeProfile {
			records: BoundedVec::try_from(vec![
				Record {
					role: RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
					amount: None,
				},
				Record {
					role: RoleType::ZkSaaS(ZeroKnowledgeRoleType::ZkSaaSGroth16),
					amount: None,
				},
			])
			.unwrap(),
			amount: 1500,
		};

		for validator in participants.clone() {
			assert_ok!(Roles::update_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				Profile::Shared(updated_profile.clone())
			));
		}
	});
}

#[test]
fn test_fee_charged_for_jobs_submission() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		// setup time fees
		assert_ok!(Jobs::set_time_fee(RuntimeOrigin::root(), 1));

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 20,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// Fees charged
		// 1. 1unit per participant
		// 2. 1unit per ttl block (20)
		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 5 - 20);
	});
}

#[test]
fn try_validator_removal_from_job_with_destory_fallback_works() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};

		let job_creator_balance = Balances::free_balance(mock_pub_key(TEN));
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// sanity check, creator was charged for job
		assert!(Balances::free_balance(mock_pub_key(TEN)) < job_creator_balance);

		// ==== now we try to remove inactive validator in the set ====
		assert_ok!(Jobs::try_validator_removal_from_job(
			RoleType::Tss(threshold_signature_role_type),
			0,
			mock_pub_key(ALICE)
		));

		assert_events(vec![RuntimeEvent::Jobs(crate::Event::JobRefunded {
			job_id: 0,
			role_type: RoleType::Tss(threshold_signature_role_type),
		})]);

		// jobs creator should recieve all fees back
		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), job_creator_balance);

		// the job should be removed from storage
		assert!(SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 0)
			.is_none());
	});
}

#[test]
fn try_validator_removal_from_job_with_retry_works_phase_one() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::RegenerateWithThreshold(3),
		};

		let job_creator_balance = Balances::free_balance(mock_pub_key(TEN));
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// sanity check, creator was charged for job
		assert!(Balances::free_balance(mock_pub_key(TEN)) < job_creator_balance);

		let job_info =
			SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 0).unwrap();

		// ==== now we try to remove inactive validator in the set ====
		assert_ok!(Jobs::try_validator_removal_from_job(
			RoleType::Tss(threshold_signature_role_type),
			0,
			mock_pub_key(ALICE)
		));

		// the job will have two changes
		let mut new_job_info = job_info.clone();
		new_job_info.fallback = FallbackOptions::Destroy;
		System::assert_has_event(RuntimeEvent::Jobs(crate::Event::JobParticipantsUpdated {
			job_id: 0,
			role_type: RoleType::Tss(threshold_signature_role_type),
			details: new_job_info,
		}));
	});
}

#[test]
fn try_validator_removal_from_job_with_retry_works_phase_two() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let participants = vec![ALICE, BOB, CHARLIE, DAVE, EVE];

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in participants.clone() {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		// submit job with existing validators
		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: participants
					.clone()
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::RegenerateWithThreshold(3),
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		let _job_info =
			SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 0).unwrap();

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// ensure the job reward is distributed correctly
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE].iter().map(|x| mock_pub_key(*x)) {
			assert_eq!(ValidatorRewards::<Runtime>::get(validator), Some(1));
		}

		// ensure storage is correctly setup
		assert!(KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_some());
		assert!(SubmittedJobs::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_none());

		// ---- use phase one solution in phase 2 signinig -------
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseTwo(DKGTSSPhaseTwoJobType {
				phase_one_id: 0,
				submission: vec![].try_into().unwrap(),
				derivation_path: None,
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::RegenerateWithThreshold(3),
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// ==== now we try to remove inactive validator in the set ====
		assert_ok!(Jobs::try_validator_removal_from_job(
			RoleType::Tss(threshold_signature_role_type),
			1,
			mock_pub_key(ALICE)
		));

		// the job will have two changes
		let job_info =
			SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 2).unwrap();

		System::assert_has_event(RuntimeEvent::Jobs(crate::Event::JobReSubmitted {
			job_id: 2,
			role_type: RoleType::Tss(threshold_signature_role_type),
			details: job_info,
		}));

		assert!(KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_none());

		// the phase two job should ahve new job id for phase one
		let job_info =
			SubmittedJobs::<Runtime>::get(RoleType::Tss(threshold_signature_role_type), 1).unwrap();
		assert_eq!(job_info.job_type.get_phase_one_id().unwrap(), 2);
		assert_eq!(job_info.fallback, FallbackOptions::Destroy);
	});
}

#[test]
fn test_validator_limit_is_counted_for_jobs_submission() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		// setup time fees
		assert_ok!(Jobs::set_time_fee(RuntimeOrigin::root(), 1));

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				Some(1)
			));
		}

		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		let submission = JobSubmission {
			expiry: 10,
			ttl: 20,
			fallback: FallbackOptions::Destroy,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 3,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission.clone()));

		// submitting again should fail since the max validators can accept is one
		assert_noop!(
			Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission),
			Error::<Runtime>::TooManyJobsForValidator
		);
	});
}

#[test]
fn jobs_extend_result_works() {
	new_test_ext(vec![ALICE, BOB, CHARLIE, DAVE, EVE]).execute_with(|| {
		System::set_block_number(1);

		let threshold_signature_role_type = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;

		// all validators sign up in roles pallet
		let profile = shared_profile();
		for validator in [ALICE, BOB, CHARLIE, DAVE, EVE] {
			assert_ok!(Roles::create_profile(
				RuntimeOrigin::signed(mock_pub_key(validator)),
				profile.clone(),
				None
			));
		}

		Balances::make_free_balance_be(&mock_pub_key(TEN), 100);

		// should work when n = t
		let submission = JobSubmission {
			expiry: 10,
			ttl: 200,
			job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
				participants: [ALICE, BOB, CHARLIE, DAVE, EVE]
					.iter()
					.map(|x| mock_pub_key(*x))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap(),
				threshold: 5,
				permitted_caller: Some(mock_pub_key(TEN)),
				role_type: threshold_signature_role_type,
			}),
			fallback: FallbackOptions::Destroy,
		};
		assert_ok!(Jobs::submit_job(RuntimeOrigin::signed(mock_pub_key(TEN)), submission));

		// submit a solution for this job
		assert_ok!(Jobs::submit_job_result(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			JobResult::DKGPhaseOne(DKGTSSKeySubmissionResult {
				signatures: vec![].try_into().unwrap(),
				threshold: 3,
				participants: vec![].try_into().unwrap(),
				key: vec![].try_into().unwrap(),
				signature_scheme: DigitalSignatureScheme::EcdsaSecp256k1
			})
		));

		// ensure storage is correctly setup
		assert!(KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0
		)
		.is_some());

		// extend result by paying fee
		assert_ok!(Jobs::extend_job_result_ttl(
			RuntimeOrigin::signed(mock_pub_key(TEN)),
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
			100
		));

		// should be charged 5 for creation and 20 for extension
		assert_eq!(Balances::free_balance(mock_pub_key(TEN)), 100 - 25);

		let known_result = KnownResults::<Runtime>::get(
			RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
			0,
		)
		.unwrap();

		assert_eq!(known_result.ttl, 300);
	});
}
