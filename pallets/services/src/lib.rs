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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::type_complexity)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, ReservableCurrency},
};
use frame_system::pallet_prelude::*;
use sp_runtime::{traits::Get, DispatchResult};

mod functions;
mod impls;
mod rpc;
pub mod types;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod mock_evm;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use module::*;
use tangle_primitives::BlueprintId;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
pub use impls::BenchmarkingOperatorDelegationManager;

#[allow(clippy::too_many_arguments)]
#[frame_support::pallet(dev_mode)]
pub mod module {
	use super::*;
	use frame_support::{
		dispatch::PostDispatchInfo,
		traits::{
			fungibles::{Inspect, Mutate},
			tokens::Preservation,
		},
	};
	use sp_core::H160;
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Zero},
		Percent,
	};
	use sp_std::vec::Vec;
	use tangle_primitives::{
		services::{MasterBlueprintServiceManagerRevision, *},
		Account, MultiAssetDelegationInfo,
	};
	use types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The origin which may set filter.
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The fungibles trait used for managing fungible assets.
		type Fungibles: Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = BalanceOf<Self>>
			+ Mutate<Self::AccountId, AssetId = Self::AssetId>;

		/// `Pallet` EVM Address.
		#[pallet::constant]
		type PalletEVMAddress: Get<H160>;

		/// A type that implements the `EvmRunner` trait for the execution of EVM
		/// transactions.
		type EvmRunner: tangle_primitives::services::EvmRunner<Self>;

		/// A type that implements the `EvmGasWeightMapping` trait for the conversion of EVM gas to
		/// Substrate weight and vice versa.
		type EvmGasWeightMapping: tangle_primitives::services::EvmGasWeightMapping;

		/// A type that implements the `EvmAddressMapping` trait for the conversion of EVM address
		type EvmAddressMapping: tangle_primitives::services::EvmAddressMapping<Self::AccountId>;

		/// The asset ID type.
		type AssetId: AtLeast32BitUnsigned
			+ Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Clone
			+ Copy
			+ PartialOrd
			+ MaxEncodedLen;

		/// Maximum number of fields in a job call.
		#[pallet::constant]
		type MaxFields: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum size of a field in a job call.
		#[pallet::constant]
		type MaxFieldsSize: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum length of metadata string length.
		#[pallet::constant]
		type MaxMetadataLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of jobs per service.
		#[pallet::constant]
		type MaxJobsPerService: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of Operators per service.
		#[pallet::constant]
		type MaxOperatorsPerService: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of permitted callers per service.
		#[pallet::constant]
		type MaxPermittedCallers: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of services per operator.
		#[pallet::constant]
		type MaxServicesPerOperator: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of blueprints per operator.
		#[pallet::constant]
		type MaxBlueprintsPerOperator: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of services per user.
		#[pallet::constant]
		type MaxServicesPerUser: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of binaries per gadget.
		#[pallet::constant]
		type MaxBinariesPerGadget: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of sources per gadget.
		#[pallet::constant]
		type MaxSourcesPerGadget: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Git owner maximum length.
		#[pallet::constant]
		type MaxGitOwnerLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Git repository maximum length.
		#[pallet::constant]
		type MaxGitRepoLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Git tag maximum length.
		#[pallet::constant]
		type MaxGitTagLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// binary name maximum length.
		#[pallet::constant]
		type MaxBinaryNameLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// IPFS hash maximum length.
		#[pallet::constant]
		type MaxIpfsHashLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Container registry maximum length.
		#[pallet::constant]
		type MaxContainerRegistryLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Container image name maximum length.
		#[pallet::constant]
		type MaxContainerImageNameLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Container image tag maximum length.
		#[pallet::constant]
		type MaxContainerImageTagLength: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of assets per service.
		#[pallet::constant]
		type MaxAssetsPerService: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;
		/// Maximum number of versions of Master Blueprint Service Manager allowed.
		#[pallet::constant]
		type MaxMasterBlueprintServiceManagerVersions: Get<u32>
			+ Default
			+ Parameter
			+ MaybeSerializeDeserialize;

		/// The constraints for the service module.
		/// use [crate::types::ConstraintsOf] with `Self` to implement this trait.
		type Constraints: Constraints;

		/// Manager for getting operator stake and delegation info
		type OperatorDelegationManager: tangle_primitives::traits::MultiAssetDelegationInfo<
			Self::AccountId,
			BalanceOf<Self>,
		>;

		/// Number of eras that slashes are deferred by, after computation.
		///
		/// This should be less than the bonding duration. Set to 0 if slashes
		/// should be applied immediately, without opportunity for intervention.
		#[pallet::constant]
		type SlashDeferDuration: Get<u32> + Default + Parameter + MaybeSerializeDeserialize;

		/// The origin which can manage Add a new Master Blueprint Service Manager revision.
		type MasterBlueprintServiceManagerUpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn integrity_test() {
			// Ensure that the pallet's configuration is valid.
			// 1. Make sure that pallet's associated AccountId value maps correctly to the EVM
			//    address.
			let account_id = T::EvmAddressMapping::into_account_id(Self::address());
			assert_eq!(account_id, Self::account_id(), "Services: AccountId mapping is incorrect.");
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The service blueprint was not found.
		BlueprintNotFound,
		/// Blueprint creation is interrupted.
		BlueprintCreationInterrupted,
		/// The caller is already registered as a operator.
		AlreadyRegistered,
		/// The caller does not have the requirements to be a operator.
		InvalidRegistrationInput,
		/// The Operator is not allowed to unregister.
		NotAllowedToUnregister,
		/// The Operator is not allowed to update their price targets.
		NotAllowedToUpdatePriceTargets,
		/// The caller does not have the requirements to request a service.
		InvalidRequestInput,
		/// The caller does not have the requirements to call a job.
		InvalidJobCallInput,
		/// The caller provided an invalid job result.
		InvalidJobResult,
		/// The caller is not registered as a operator.
		NotRegistered,
		/// Approval Process is interrupted.
		ApprovalInterrupted,
		/// Rejection Process is interrupted.
		RejectionInterrupted,
		/// The service request was not found.
		ServiceRequestNotFound,
		/// Service Initialization interrupted.
		ServiceInitializationInterrupted,
		/// The service was not found.
		ServiceNotFound,
		/// The termination of the service was interrupted.
		TerminationInterrupted,
		/// An error occurred while type checking the provided input input.
		TypeCheck(TypeCheckError),
		/// The maximum number of permitted callers per service has been exceeded.
		MaxPermittedCallersExceeded,
		/// The maximum number of operators per service has been exceeded.
		MaxServiceProvidersExceeded,
		/// The maximum number of services per user has been exceeded.
		MaxServicesPerUserExceeded,
		/// The maximum number of fields per request has been exceeded.
		MaxFieldsExceeded,
		/// The approval is not requested for the operator (the caller).
		ApprovalNotRequested,
		/// The requested job definition does not exist.
		/// This error is returned when the requested job definition does not exist in the service
		/// blueprint.
		JobDefinitionNotFound,
		/// Either the service or the job call was not found.
		ServiceOrJobCallNotFound,
		/// The result of the job call was not found.
		JobCallResultNotFound,
		/// An error occurred while encoding the EVM ABI.
		EVMAbiEncode,
		/// An error occurred while decoding the EVM ABI.
		EVMAbiDecode,
		/// Operator profile not found.
		OperatorProfileNotFound,
		/// Maximum number of services per Provider reached.
		MaxServicesPerProviderExceeded,
		/// The operator is not active, ensure operator status is ACTIVE in multi-asset-delegation
		OperatorNotActive,
		/// No assets provided for the service, at least one asset is required.
		NoAssetsProvided,
		/// The maximum number of assets per service has been exceeded.
		MaxAssetsPerServiceExceeded,
		/// Offender is not a registered operator.
		OffenderNotOperator,
		/// Offender is not an active operator.
		OffenderNotActiveOperator,
		/// The Service Blueprint did not return a slashing origin for this service.
		NoSlashingOrigin,
		/// The Service Blueprint did not return a dispute origin for this service.
		NoDisputeOrigin,
		/// The Unapplied Slash are not found.
		UnappliedSlashNotFound,
		/// The Supplied Master Blueprint Service Manager Revision is not found.
		MasterBlueprintServiceManagerRevisionNotFound,
		/// Maximum number of Master Blueprint Service Manager revisions reached.
		MaxMasterBlueprintServiceManagerVersionsExceeded,
		/// The ERC20 transfer failed.
		ERC20TransferFailed,
		/// Missing EVM Origin for the EVM execution.
		MissingEVMOrigin,
		/// Expected the account to be an EVM address.
		ExpectedEVMAddress,
		/// Expected the account to be an account ID.
		ExpectedAccountId,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new service blueprint has been created.
		BlueprintCreated {
			/// The account that created the service blueprint.
			owner: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},
		/// An operator has pre-registered for a service blueprint.
		PreRegistration {
			/// The account that pre-registered as an operator.
			operator: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},
		/// An new operator has been registered.
		Registered {
			/// The account that registered as a operator.
			provider: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The preferences for the operator for this specific blueprint.
			preferences: OperatorPreferences,
			/// The arguments used for registration.
			registration_args: Vec<Field<T::Constraints, T::AccountId>>,
		},
		/// An operator has been unregistered.
		Unregistered {
			/// The account that unregistered as am operator.
			operator: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},

		/// The price targets for an operator has been updated.
		PriceTargetsUpdated {
			/// The account that updated the approval preference.
			operator: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The new price targets.
			price_targets: PriceTargets,
		},

		/// A new service has been requested.
		ServiceRequested {
			/// The account that requested the service.
			owner: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The list of operators that need to approve the service.
			pending_approvals: Vec<T::AccountId>,
			/// The list of operators that atomaticaly approved the service.
			approved: Vec<T::AccountId>,
			/// The list of asset IDs that are being used to secure the service.
			assets: Vec<T::AssetId>,
		},
		/// A service request has been approved.
		ServiceRequestApproved {
			/// The account that approved the service.
			operator: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The list of operators that need to approve the service.
			pending_approvals: Vec<T::AccountId>,
			/// The list of operators that atomaticaly approved the service.
			approved: Vec<T::AccountId>,
		},
		/// A service request has been rejected.
		ServiceRequestRejected {
			/// The account that rejected the service.
			operator: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},
		/// A service has been initiated.
		ServiceInitiated {
			/// The owner of the service.
			owner: T::AccountId,
			/// The ID of the service request that got approved.
			request_id: u64,
			/// The ID of the service.
			service_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The list of asset IDs that are being used to secure the service.
			assets: Vec<T::AssetId>,
		},

		/// A service has been terminated.
		ServiceTerminated {
			/// The owner of the service.
			owner: T::AccountId,
			/// The ID of the service.
			service_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},

		/// A job has been called.
		JobCalled {
			/// The account that called the job.
			caller: T::AccountId,
			/// The ID of the service.
			service_id: u64,
			/// The ID of the call.
			call_id: u64,
			/// The index of the job.
			job: u8,
			/// The arguments of the job.
			args: Vec<Field<T::Constraints, T::AccountId>>,
		},

		/// A job result has been submitted.
		JobResultSubmitted {
			/// The account that submitted the job result.
			operator: T::AccountId,
			/// The ID of the service.
			service_id: u64,
			/// The ID of the call.
			call_id: u64,
			/// The index of the job.
			job: u8,
			/// The result of the job.
			result: Vec<Field<T::Constraints, T::AccountId>>,
		},
		/// EVM execution reverted with a reason.
		EvmReverted { from: H160, to: H160, data: Vec<u8>, reason: Vec<u8> },
		/// An Operator has an unapplied slash.
		UnappliedSlash {
			/// The index of the slash.
			index: u32,
			/// The account that has an unapplied slash.
			operator: T::AccountId,
			/// The amount of the slash.
			amount: BalanceOf<T>,
			/// Service ID
			service_id: u64,
			/// Blueprint ID
			blueprint_id: u64,
			/// Era index
			era: u32,
		},
		/// An Unapplied Slash got discarded.
		SlashDiscarded {
			/// The index of the slash.
			index: u32,
			/// The account that has an unapplied slash.
			operator: T::AccountId,
			/// The amount of the slash.
			amount: BalanceOf<T>,
			/// Service ID
			service_id: u64,
			/// Blueprint ID
			blueprint_id: u64,
			/// Era index
			era: u32,
		},

		/// The Master Blueprint Service Manager has been revised.
		MasterBlueprintServiceManagerRevised {
			/// The revision number of the Master Blueprint Service Manager.
			revision: u32,
			/// The address of the Master Blueprint Service Manager.
			address: H160,
		},
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Counters

	/// The next free ID for a service blueprint.
	#[pallet::storage]
	#[pallet::getter(fn next_blueprint_id)]
	pub type NextBlueprintId<T> = StorageValue<_, BlueprintId, ValueQuery>;

	/// The next free ID for a service request.
	#[pallet::storage]
	#[pallet::getter(fn next_service_request_id)]
	pub type NextServiceRequestId<T> = StorageValue<_, u64, ValueQuery>;

	/// The next free ID for a service Instance.
	#[pallet::storage]
	#[pallet::getter(fn next_instance_id)]
	pub type NextInstanceId<T> = StorageValue<_, u64, ValueQuery>;

	/// The next free ID for a service call.
	#[pallet::storage]
	#[pallet::getter(fn next_job_call_id)]
	pub type NextJobCallId<T> = StorageValue<_, u64, ValueQuery>;

	/// The next free ID for a unapplied slash.
	#[pallet::storage]
	#[pallet::getter(fn next_unapplied_slash_index)]
	pub type NextUnappliedSlashIndex<T> = StorageValue<_, u32, ValueQuery>;

	/// The service blueprints along with their owner.
	#[pallet::storage]
	#[pallet::getter(fn blueprints)]
	pub type Blueprints<T: Config> = StorageMap<
		_,
		Identity,
		u64,
		(T::AccountId, ServiceBlueprint<T::Constraints>),
		ResultQuery<Error<T>::BlueprintNotFound>,
	>;

	/// The operators for a specific service blueprint.
	/// Blueprint ID -> Operator -> Operator Preferences
	#[pallet::storage]
	#[pallet::getter(fn operators)]
	pub type Operators<T: Config> = StorageDoubleMap<
		_,
		Identity,
		u64,
		Identity,
		T::AccountId,
		OperatorPreferences,
		ResultQuery<Error<T>::NotRegistered>,
	>;

	/// The service requests along with their owner.
	/// Request ID -> Service Request
	#[pallet::storage]
	#[pallet::getter(fn service_requests)]
	pub type ServiceRequests<T: Config> = StorageMap<
		_,
		Identity,
		u64,
		ServiceRequest<T::Constraints, T::AccountId, BlockNumberFor<T>, T::AssetId>,
		ResultQuery<Error<T>::ServiceRequestNotFound>,
	>;

	/// The Services Instances
	/// Service ID -> Service
	#[pallet::storage]
	#[pallet::getter(fn services)]
	pub type Instances<T: Config> = StorageMap<
		_,
		Identity,
		u64,
		Service<T::Constraints, T::AccountId, BlockNumberFor<T>, T::AssetId>,
		ResultQuery<Error<T>::ServiceNotFound>,
	>;

	/// User Service Instances
	/// User Account ID -> Service ID
	#[pallet::storage]
	#[pallet::getter(fn user_services)]
	pub type UserServices<T: Config> = StorageMap<
		_,
		Identity,
		T::AccountId,
		BoundedBTreeSet<u64, MaxServicesPerUserOf<T>>,
		ValueQuery,
	>;

	/// The Service Job Calls
	/// Service ID -> Call ID -> Job Call
	#[pallet::storage]
	#[pallet::getter(fn job_calls)]
	pub type JobCalls<T: Config> = StorageDoubleMap<
		_,
		Identity,
		u64,
		Identity,
		u64,
		JobCall<T::Constraints, T::AccountId>,
		ResultQuery<Error<T>::ServiceOrJobCallNotFound>,
	>;

	/// The Service Job Call Results
	/// Service ID -> Call ID -> Job Call Result
	#[pallet::storage]
	#[pallet::getter(fn job_results)]
	pub type JobResults<T: Config> = StorageDoubleMap<
		_,
		Identity,
		u64,
		Identity,
		u64,
		JobCallResult<T::Constraints, T::AccountId>,
		ResultQuery<Error<T>::ServiceOrJobCallNotFound>,
	>;

	/// All unapplied slashes that are queued for later.
	///
	/// EraIndex -> Index -> UnappliedSlash
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn unapplied_slashes)]
	pub type UnappliedSlashes<T: Config> = StorageDoubleMap<
		_,
		Identity,
		u32,
		Identity,
		u32,
		UnappliedSlash<T::AccountId, BalanceOf<T>>,
		ResultQuery<Error<T>::UnappliedSlashNotFound>,
	>;

	/// All the Master Blueprint Service Managers revisions.
	///
	/// Where the index is the revision number.
	#[pallet::storage]
	#[pallet::getter(fn mbsm_revisions)]
	pub type MasterBlueprintServiceManagerRevisions<T: Config> =
		StorageValue<_, BoundedVec<H160, T::MaxMasterBlueprintServiceManagerVersions>, ValueQuery>;

	// *** auxiliary storage and maps ***
	#[pallet::storage]
	#[pallet::getter(fn operator_profile)]
	pub type OperatorsProfile<T: Config> = StorageMap<
		_,
		Identity,
		T::AccountId,
		OperatorProfile<T::Constraints>,
		ResultQuery<Error<T>::OperatorProfileNotFound>,
	>;
	/// Holds the service payment information for a service request.
	/// Once the service is initiated, the payment is transferred to the MBSM and this
	/// information is removed.
	///
	/// Service Requst ID -> Service Payment
	#[pallet::storage]
	#[pallet::getter(fn service_payment)]
	pub type StagingServicePayments<T: Config> =
		StorageMap<_, Identity, u64, StagingServicePayment<T::AccountId, T::AssetId, BalanceOf<T>>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new service blueprint.
		///
		/// A Service Blueprint is a template for a service that can be instantiated by users. The blueprint
		/// defines the service's constraints, requirements and behavior, including the master blueprint service
		/// manager revision to use.
		///
		/// # Permissions
		///
		/// * The origin must be signed by the account that will own the blueprint
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call, must be signed by the account creating the blueprint
		/// * `blueprint` - The service blueprint containing:
		///   - Service constraints and requirements
		///   - Master blueprint service manager revision (Latest or Specific)
		///   - Template configuration for service instantiation
		///
		/// # Errors
		///
		/// * [`Error::BadOrigin`] - Origin is not signed
		/// * [`Error::MasterBlueprintServiceManagerRevisionNotFound`] - Specified MBSM revision does not exist
		/// * [`Error::BlueprintCreationInterrupted`] - Blueprint creation is interrupted by hooks
		///
		/// # Returns
		///
		/// Returns a `DispatchResultWithPostInfo` which on success emits a [`Event::BlueprintCreated`] event
		/// containing the owner and blueprint ID.
		#[pallet::weight(T::WeightInfo::create_blueprint())]
		pub fn create_blueprint(
			origin: OriginFor<T>,
			mut blueprint: ServiceBlueprint<T::Constraints>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let blueprint_id = Self::next_blueprint_id();
			// Ensure the master blueprint service manager exists and if it uses
			// latest, pin it to the latest revision.
			match blueprint.master_manager_revision {
				MasterBlueprintServiceManagerRevision::Latest => {
					let latest_revision = Self::mbsm_latest_revision();
					blueprint.master_manager_revision =
						MasterBlueprintServiceManagerRevision::Specific(latest_revision);
				},
				MasterBlueprintServiceManagerRevision::Specific(rev) => {
					ensure!(
						rev <= Self::mbsm_latest_revision(),
						Error::<T>::MasterBlueprintServiceManagerRevisionNotFound,
					);
				},
				_ => unreachable!("MasterBlueprintServiceManagerRevision case is not implemented"),
			};

			let (allowed, _weight) =
				Self::on_blueprint_created_hook(&blueprint, blueprint_id, &owner)?;

			ensure!(allowed, Error::<T>::BlueprintCreationInterrupted);

			Blueprints::<T>::insert(blueprint_id, (owner.clone(), blueprint));
			NextBlueprintId::<T>::set(blueprint_id.saturating_add(1));

			Self::deposit_event(Event::BlueprintCreated { owner, blueprint_id });
			// TODO: update weight for the creation of the blueprint.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Pre-register the caller as an operator for a specific blueprint.
		///
		/// This function allows an account to signal intent to become an operator for a blueprint by emitting
		/// a `PreRegistration` event. The operator node can listen for this event to execute any custom
		/// registration logic defined in the blueprint.
		///
		/// Pre-registration is the first step in the operator registration flow. After pre-registering,
		/// operators must complete the full registration process by calling `register()` with their preferences
		/// and registration arguments.
		///
		/// # Arguments
		///
		/// * `origin: OriginFor<T>` - The origin of the call. Must be signed by the account that wants to
		///   become an operator.
		/// * `blueprint_id: u64` - The identifier of the service blueprint to pre-register for. Must refer
		///   to an existing blueprint.
		///
		/// # Permissions
		///
		/// * The caller must be a signed account.
		///
		/// # Events
		///
		/// * [`Event::PreRegistration`] - Emitted when pre-registration is successful, containing:
		///   - `operator: T::AccountId` - The account ID of the pre-registering operator
		///   - `blueprint_id: u64` - The ID of the blueprint being pre-registered for
		///
		/// # Errors
		///
		/// * [`Error::BadOrigin`] - The origin was not signed.
		#[pallet::weight(T::WeightInfo::pre_register())]
		pub fn pre_register(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
		) -> DispatchResult {
			let operator_controller = ensure_signed(origin)?;

			// Emit the PreRegistration event
			Self::deposit_event(Event::PreRegistration {
				operator: operator_controller.clone(),
				blueprint_id,
			});

			Ok(())
		}

		/// Register the caller as an operator for a specific blueprint.
		///
		/// This function allows an account to register as an operator for a blueprint by providing their
		/// service preferences, registration arguments, and staking the required tokens. The operator must
		/// be active in the delegation system and may require approval before accepting service requests.
		///
		/// # Permissions
		///
		/// * The caller must be a signed account
		/// * The caller must be an active operator in the delegation system
		/// * The caller must not already be registered for this blueprint
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call. Must be signed.
		/// * `blueprint_id` - The identifier of the service blueprint to register for
		/// * `preferences` - The operator's service preferences and configuration
		/// * `registration_args` - Registration arguments required by the blueprint
		/// * `value` - Amount of tokens to stake for registration
		///
		/// # Errors
		///
		/// * [`Error::OperatorNotActive`] - Caller is not an active operator in the delegation system
		/// * [`Error::AlreadyRegistered`] - Caller is already registered for this blueprint
		/// * [`Error::TypeCheck`] - Registration arguments failed type checking
		/// * [`Error::InvalidRegistrationInput`] - Registration hook rejected the registration
		/// * [`Error::MaxServicesPerProviderExceeded`] - Operator has reached maximum services limit
		#[pallet::weight(T::WeightInfo::register())]
		pub fn register(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
			preferences: OperatorPreferences,
			registration_args: Vec<Field<T::Constraints, T::AccountId>>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;

			ensure!(
				T::OperatorDelegationManager::is_operator_active(&caller),
				Error::<T>::OperatorNotActive
			);

			let already_registered = Operators::<T>::contains_key(blueprint_id, &caller);
			ensure!(!already_registered, Error::<T>::AlreadyRegistered);
			blueprint
				.type_check_registration(&registration_args)
				.map_err(Error::<T>::TypeCheck)?;

			// Transfer the registration value to the pallet
			T::Currency::transfer(
				&caller,
				&Self::account_id(),
				value,
				ExistenceRequirement::KeepAlive,
			)?;

			let (allowed, _weight) = Self::on_register_hook(
				&blueprint,
				blueprint_id,
				&preferences,
				&registration_args,
				value,
			)?;

			ensure!(allowed, Error::<T>::InvalidRegistrationInput);

			Operators::<T>::insert(blueprint_id, &caller, preferences);

			OperatorsProfile::<T>::try_mutate(&caller, |profile| {
				match profile {
					Ok(p) => {
						p.blueprints
							.try_insert(blueprint_id)
							.map_err(|_| Error::<T>::MaxServicesPerProviderExceeded)?;
					},
					Err(_) => {
						let mut blueprints = BoundedBTreeSet::new();
						blueprints
							.try_insert(blueprint_id)
							.map_err(|_| Error::<T>::MaxServicesPerProviderExceeded)?;
						*profile = Ok(OperatorProfile { blueprints, ..Default::default() });
					},
				};
				Result::<_, Error<T>>::Ok(())
			})?;

			Self::deposit_event(Event::Registered {
				provider: caller.clone(),
				blueprint_id,
				preferences,
				registration_args,
			});

			// TODO: update weight for the registration.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Unregisters a service provider from a specific service blueprint.
		///
		/// After unregistering, the provider will no longer receive new service assignments for this blueprint.
		/// However, they must continue servicing any active assignments until completion to avoid penalties.
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call. Must be signed.
		/// * `blueprint_id` - The identifier of the service blueprint to unregister from.
		///
		/// # Permissions
		///
		/// * Must be signed by a registered service provider
		///
		/// # Errors
		///
		/// * [`Error::NotRegistered`] - The caller is not registered for this blueprint
		/// * [`Error::NotAllowedToUnregister`] - Unregistration is currently restricted
		/// * [`Error::BlueprintNotFound`] - The blueprint_id does not exist
		#[pallet::weight(T::WeightInfo::unregister())]
		pub fn unregister(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;
			let preferences = Operators::<T>::get(blueprint_id, &caller)?;
			let (allowed, _weight) =
				Self::on_unregister_hook(&blueprint, blueprint_id, &preferences)?;
			ensure!(allowed, Error::<T>::NotAllowedToUnregister);
			// TODO: check if the caller is not providing any service for the blueprint.
			Operators::<T>::remove(blueprint_id, &caller);

			// TODO: also remove all the services that uses this blueprint?
			let removed = OperatorsProfile::<T>::try_mutate_exists(&caller, |profile| {
				profile
					.as_mut()
					.map(|p| p.blueprints.remove(&blueprint_id))
					.ok_or(Error::<T>::NotRegistered)
			})?;

			ensure!(removed, Error::<T>::NotRegistered);
			Self::deposit_event(Event::Unregistered { operator: caller.clone(), blueprint_id });
			// TODO: update weight for the unregistration.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Updates the price targets for a registered operator's service blueprint.
		///
		/// Allows an operator to modify their price targets for a specific blueprint they are registered for.
		/// The operator must already be registered for the blueprint to update prices.
		///
		/// # Arguments
		///
		/// * `origin: OriginFor<T>` - The origin of the call. Must be signed by the operator.
		/// * `blueprint_id: u64` - The identifier of the blueprint to update price targets for.
		/// * `price_targets: PriceTargets` - The new price targets to set for the blueprint.
		///
		/// # Permissions
		///
		/// * Must be signed by a registered operator for this blueprint.
		///
		/// # Errors
		///
		/// * [`Error::NotRegistered`] - The caller is not registered for this blueprint.
		/// * [`Error::NotAllowedToUpdatePriceTargets`] - Price target updates are currently restricted.
		/// * [`Error::BlueprintNotFound`] - The blueprint_id does not exist.
		#[pallet::weight(T::WeightInfo::update_price_targets())]
		pub fn update_price_targets(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
			price_targets: PriceTargets,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;

			let updated_preferences =
				Operators::<T>::try_mutate_exists(blueprint_id, &caller, |current_preferences| {
					current_preferences
						.as_mut()
						.map(|v| {
							v.price_targets = price_targets;
							*v
						})
						.ok_or(Error::<T>::NotRegistered)
				})?;

			let (allowed, _weight) =
				Self::on_update_price_targets(&blueprint, blueprint_id, &updated_preferences)?;

			ensure!(allowed, Error::<T>::NotAllowedToUpdatePriceTargets);

			Self::deposit_event(Event::PriceTargetsUpdated {
				operator: caller.clone(),
				blueprint_id,
				price_targets,
			});
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Request a new service using a blueprint and specified operators.
		///
		/// # Arguments
		///
		/// * `origin: OriginFor<T>` - The origin of the call. Must be signed.
		/// * `evm_origin: Option<H160>` - Optional EVM address for ERC20 payments.
		/// * `blueprint_id: u64` - The identifier of the blueprint to use.
		/// * `permitted_callers: Vec<T::AccountId>` - Accounts allowed to call the service. If empty, only owner can call.
		/// * `operators: Vec<T::AccountId>` - List of operators that will run the service.
		/// * `request_args: Vec<Field<T::Constraints, T::AccountId>>` - Blueprint initialization arguments.
		/// * `assets: Vec<T::AssetId>` - Required assets for the service.
		/// * `ttl: BlockNumberFor<T>` - Time-to-live in blocks for the service request.
		/// * `payment_asset: Asset<T::AssetId>` - Asset used for payment (native, custom or ERC20).
		/// * `value: BalanceOf<T>` - Payment amount for the service.
		///
		/// # Permissions
		///
		/// * Must be signed by an account with sufficient balance to pay for the service.
		/// * For ERC20 payments, the EVM origin must match the caller's mapped account.
		///
		/// # Errors
		///
		/// * [`Error::TypeCheck`] - Request arguments fail blueprint type checking.
		/// * [`Error::NoAssetsProvided`] - No assets were specified.
		/// * [`Error::MissingEVMOrigin`] - EVM origin required but not provided for ERC20 payment.
		/// * [`Error::ERC20TransferFailed`] - ERC20 token transfer failed.
		/// * [`Error::NotRegistered`] - One or more operators not registered for blueprint.
		/// * [`Error::BlueprintNotFound`] - The blueprint_id does not exist.
		#[pallet::weight(T::WeightInfo::request())]
		pub fn request(
			origin: OriginFor<T>,
			evm_origin: Option<H160>,
			#[pallet::compact] blueprint_id: u64,
			permitted_callers: Vec<T::AccountId>,
			operators: Vec<T::AccountId>,
			request_args: Vec<Field<T::Constraints, T::AccountId>>,
			assets: Vec<T::AssetId>,
			#[pallet::compact] ttl: BlockNumberFor<T>,
			payment_asset: Asset<T::AssetId>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;

			blueprint.type_check_request(&request_args).map_err(Error::<T>::TypeCheck)?;
			// ensure we at least have one asset
			ensure!(!assets.is_empty(), Error::<T>::NoAssetsProvided);

			let mut preferences = Vec::new();
			let mut pending_approvals = Vec::new();
			for provider in &operators {
				let prefs = Self::operators(blueprint_id, provider)?;
				pending_approvals.push(provider.clone());
				preferences.push(prefs);
			}

			let mut native_value = Zero::zero();
			let request_id = NextServiceRequestId::<T>::get();

			if value != Zero::zero() {
				// Payment transfer
				let refund_to = match payment_asset {
					// Handle the case of native currency.
					Asset::Custom(asset_id) if asset_id == Zero::zero() => {
						T::Currency::transfer(
							&caller,
							&Self::account_id(),
							value,
							ExistenceRequirement::KeepAlive,
						)?;
						native_value = value;
						Account::id(caller.clone())
					},
					Asset::Custom(asset_id) => {
						T::Fungibles::transfer(
							asset_id,
							&caller,
							&Self::account_id(),
							value,
							Preservation::Preserve,
						)?;
						Account::id(caller.clone())
					},
					Asset::Erc20(token) => {
						// origin check.
						let evm_origin = evm_origin.ok_or(Error::<T>::MissingEVMOrigin)?;
						let mapped_origin = T::EvmAddressMapping::into_account_id(evm_origin);
						ensure!(mapped_origin == caller, DispatchError::BadOrigin);
						let (success, _weight) =
							Self::erc20_transfer(token, evm_origin, Self::address(), value)?;
						ensure!(success, Error::<T>::ERC20TransferFailed);
						Account::from(evm_origin)
					},
				};

				// Save the payment information for the service request.
				let payment = StagingServicePayment {
					request_id,
					refund_to,
					asset: payment_asset,
					amount: value,
				};

				StagingServicePayments::<T>::insert(request_id, payment);
			}

			let (allowed, _weight) = Self::on_request_hook(
				&blueprint,
				blueprint_id,
				&caller,
				request_id,
				&preferences,
				&request_args,
				&permitted_callers,
				&assets,
				ttl,
				payment_asset,
				value,
				native_value,
			)?;

			let permitted_callers =
				BoundedVec::<_, MaxPermittedCallersOf<T>>::try_from(permitted_callers)
					.map_err(|_| Error::<T>::MaxPermittedCallersExceeded)?;
			let assets = BoundedVec::<_, MaxAssetsPerServiceOf<T>>::try_from(assets)
				.map_err(|_| Error::<T>::MaxAssetsPerServiceExceeded)?;
			let operators = pending_approvals
				.iter()
				.cloned()
				.map(|v| (v, ApprovalState::Pending))
				.collect::<Vec<_>>();

			let args = BoundedVec::<_, MaxFieldsOf<T>>::try_from(request_args)
				.map_err(|_| Error::<T>::MaxFieldsExceeded)?;

			let operators_with_approval_state =
				BoundedVec::<_, MaxOperatorsPerServiceOf<T>>::try_from(operators)
					.map_err(|_| Error::<T>::MaxServiceProvidersExceeded)?;

			let service_request = ServiceRequest {
				blueprint: blueprint_id,
				owner: caller.clone(),
				assets: assets.clone(),
				ttl,
				args,
				permitted_callers,
				operators_with_approval_state,
			};

			ensure!(allowed, Error::<T>::InvalidRequestInput);
			ServiceRequests::<T>::insert(request_id, service_request);
			NextServiceRequestId::<T>::set(request_id.saturating_add(1));

			Self::deposit_event(Event::ServiceRequested {
				owner: caller.clone(),
				request_id,
				blueprint_id,
				pending_approvals,
				approved: Default::default(),
				assets: assets.to_vec(),
			});

			// TODO: add weight for the request to the total weight.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Approve a service request, allowing it to be initiated once all required approvals are received.
		///
		/// # Permissions
		///
		/// * Caller must be a registered operator for the service blueprint
		/// * Caller must be in the pending approvals list for this request
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call, must be a signed account
		/// * `request_id` - The ID of the service request to approve
		/// * `restaking_percent` - Percentage of staked tokens to expose to this service (0-100)
		///
		/// # Errors
		///
		/// * [`Error::ApprovalNotRequested`] - Caller is not in the pending approvals list
		/// * [`Error::ApprovalInterrupted`] - Approval was rejected by blueprint hook
		#[pallet::weight(T::WeightInfo::approve())]
		pub fn approve(
			origin: OriginFor<T>,
			#[pallet::compact] request_id: u64,
			#[pallet::compact] restaking_percent: Percent,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let mut request = Self::service_requests(request_id)?;
			let updated = request
				.operators_with_approval_state
				.iter_mut()
				.find(|(v, _)| v == &caller)
				.map(|(_, s)| *s = ApprovalState::Approved { restaking_percent });
			ensure!(updated.is_some(), Error::<T>::ApprovalNotRequested);

			let blueprint_id = request.blueprint;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;
			let preferences = Operators::<T>::get(blueprint_id, caller.clone())?;
			let approved = request
				.operators_with_approval_state
				.iter()
				.filter_map(|(v, s)| {
					if matches!(*s, ApprovalState::Approved { .. }) {
						Some(v.clone())
					} else {
						None
					}
				})
				.collect::<Vec<_>>();
			let pending_approvals = request
				.operators_with_approval_state
				.iter()
				.filter_map(
					|(v, s)| if *s == ApprovalState::Pending { Some(v.clone()) } else { None },
				)
				.collect::<Vec<_>>();

			let (allowed, _weight) = Self::on_approve_hook(
				&blueprint,
				blueprint_id,
				&preferences,
				request_id,
				restaking_percent.deconstruct(),
			)?;

			ensure!(allowed, Error::<T>::ApprovalInterrupted);
			// we emit this event regardless of the outcome of the approval.
			Self::deposit_event(Event::ServiceRequestApproved {
				operator: caller.clone(),
				request_id,
				blueprint_id: request.blueprint,
				pending_approvals,
				approved,
			});

			if request.is_approved() {
				// remove the service request.
				ServiceRequests::<T>::remove(request_id);

				let service_id = Self::next_instance_id();
				let operators = request
					.operators_with_approval_state
					.into_iter()
					.filter_map(|(v, state)| match state {
						ApprovalState::Approved { restaking_percent } => {
							Some((v, restaking_percent))
						},
						// N.B: this should not happen, as all operators are approved and checked
						// above.
						_ => None,
					})
					.collect::<Vec<_>>();

				// add the service id to the list of services for each operator's profile.
				for (operator, _) in &operators {
					OperatorsProfile::<T>::try_mutate_exists(operator, |profile| {
						profile
							.as_mut()
							.and_then(|p| p.services.try_insert(service_id).ok())
							.ok_or(Error::<T>::NotRegistered)
					})?;
				}
				let operators = BoundedVec::<_, MaxOperatorsPerServiceOf<T>>::try_from(operators)
					.map_err(|_| Error::<T>::MaxServiceProvidersExceeded)?;
				let service = Service {
					id: service_id,
					blueprint: request.blueprint,
					owner: request.owner.clone(),
					assets: request.assets.clone(),
					permitted_callers: request.permitted_callers.clone(),
					operators,
					ttl: request.ttl,
				};

				UserServices::<T>::try_mutate(&request.owner, |service_ids| {
					Instances::<T>::insert(service_id, service);
					NextInstanceId::<T>::set(service_id.saturating_add(1));
					service_ids
						.try_insert(service_id)
						.map_err(|_| Error::<T>::MaxServicesPerUserExceeded)
				})?;

				// Payment
				if let Some(payment) = Self::service_payment(request_id) {
					// send payments to the MBSM
					let mbsm_address = Self::mbsm_address_of(&blueprint)?;
					let mbsm_account_id = T::EvmAddressMapping::into_account_id(mbsm_address);
					match payment.asset {
						Asset::Custom(asset_id) if asset_id == Zero::zero() => {
							T::Currency::transfer(
								&Self::account_id(),
								&mbsm_account_id,
								payment.amount,
								ExistenceRequirement::AllowDeath,
							)?;
						},
						Asset::Custom(asset_id) => {
							T::Fungibles::transfer(
								asset_id,
								&Self::account_id(),
								&mbsm_account_id,
								payment.amount,
								Preservation::Expendable,
							)?;
						},
						Asset::Erc20(token) => {
							let (success, _weight) = Self::erc20_transfer(
								token,
								Self::address(),
								mbsm_address,
								payment.amount,
							)?;
							ensure!(success, Error::<T>::ERC20TransferFailed);
						},
					}

					// Remove the payment information.
					StagingServicePayments::<T>::remove(request_id);
				}

				let (allowed, _weight) = Self::on_service_init_hook(
					&blueprint,
					blueprint_id,
					request_id,
					service_id,
					&request.owner,
					&request.permitted_callers,
					&request.assets,
					request.ttl,
				)?;

				ensure!(allowed, Error::<T>::ServiceInitializationInterrupted);

				Self::deposit_event(Event::ServiceInitiated {
					owner: request.owner,
					request_id,
					assets: request.assets.to_vec(),
					service_id,
					blueprint_id: request.blueprint,
				});
			} else {
				// Update the service request.
				ServiceRequests::<T>::insert(request_id, request);
			}

			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Reject a service request, preventing its initiation.
		///
		/// The service request will remain in the system but marked as rejected. The requester will
		/// need to update the service request to proceed.
		///
		/// # Permissions
		///
		/// * Caller must be a registered operator for the blueprint associated with this request
		/// * Caller must be one of the operators required to approve this request
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call, must be a signed account
		/// * `request_id` - The ID of the service request to reject
		///
		/// # Errors
		///
		/// * [`Error::ApprovalNotRequested`] - Caller is not one of the operators required to approve this request
		/// * [`Error::ExpectedAccountId`] - Failed to convert refund address to account ID when refunding payment
		/// * [`Error::RejectionInterrupted`] - Rejection was interrupted by blueprint hook
		#[pallet::weight(T::WeightInfo::reject())]
		pub fn reject(
			origin: OriginFor<T>,
			#[pallet::compact] request_id: u64,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let mut request = Self::service_requests(request_id)?;
			let updated = request.operators_with_approval_state.iter_mut().find_map(|(v, s)| {
				if v == &caller {
					*s = ApprovalState::Rejected;
					Some(())
				} else {
					None
				}
			});

			ensure!(updated.is_some(), Error::<T>::ApprovalNotRequested);

			let blueprint_id = request.blueprint;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;
			let prefs = Operators::<T>::get(blueprint_id, caller.clone())?;

			let (allowed, _weight) =
				Self::on_reject_hook(&blueprint, blueprint_id, &prefs, request_id)?;

			ensure!(allowed, Error::<T>::RejectionInterrupted);
			Self::deposit_event(Event::ServiceRequestRejected {
				operator: caller,
				blueprint_id: request.blueprint,
				request_id,
			});

			// Refund the payment
			if let Some(payment) = Self::service_payment(request_id) {
				match payment.asset {
					Asset::Custom(asset_id) if asset_id == Zero::zero() => {
						let refund_to = payment
							.refund_to
							.try_into_account_id()
							.map_err(|_| Error::<T>::ExpectedAccountId)?;
						T::Currency::transfer(
							&Self::account_id(),
							&refund_to,
							payment.amount,
							ExistenceRequirement::AllowDeath,
						)?;
					},
					Asset::Custom(asset_id) => {
						let refund_to = payment
							.refund_to
							.try_into_account_id()
							.map_err(|_| Error::<T>::ExpectedAccountId)?;
						T::Fungibles::transfer(
							asset_id,
							&Self::account_id(),
							&refund_to,
							payment.amount,
							Preservation::Expendable,
						)?;
					},
					Asset::Erc20(token) => {
						let refund_to = payment
							.refund_to
							.try_into_address()
							.map_err(|_| Error::<T>::ExpectedEVMAddress)?;
						let (success, _weight) = Self::erc20_transfer(
							token,
							Self::address(),
							refund_to,
							payment.amount,
						)?;
						ensure!(success, Error::<T>::ERC20TransferFailed);
					},
				}
				StagingServicePayments::<T>::remove(request_id);
			}

			// TODO: make use of the returned weight from the hook.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Terminates a running service instance.
		///
		/// # Permissions
		///
		/// * Must be signed by the service owner
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call
		/// * `service_id` - The identifier of the service to terminate
		///
		/// # Errors
		///
		/// * [`Error::ServiceNotFound`] - The service_id does not exist
		/// * [`Error::NotRegistered`] - Service operator not registered
		/// * [`Error::TerminationInterrupted`] - Service termination was interrupted by hooks
		/// * [`DispatchError::BadOrigin`] - Caller is not the service owner
		#[pallet::weight(T::WeightInfo::terminate())]
		pub fn terminate(
			origin: OriginFor<T>,
			#[pallet::compact] service_id: u64,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let service = Self::services(service_id)?;
			// TODO: allow permissioned callers to terminate the service?
			ensure!(service.owner == caller, DispatchError::BadOrigin);
			let removed = UserServices::<T>::try_mutate(&caller, |service_ids| {
				Result::<_, Error<T>>::Ok(service_ids.remove(&service_id))
			})?;
			ensure!(removed, Error::<T>::ServiceNotFound);
			Instances::<T>::remove(service_id);
			let blueprint_id = service.blueprint;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;
			let (allowed, _weight) = Self::on_service_termination_hook(
				&blueprint,
				blueprint_id,
				service_id,
				&service.owner,
			)?;

			ensure!(allowed, Error::<T>::TerminationInterrupted);
			// Remove the service from the operator's profile.
			for (operator, _) in &service.operators {
				OperatorsProfile::<T>::try_mutate_exists(operator, |profile| {
					profile
						.as_mut()
						.map(|p| p.services.remove(&service_id))
						.ok_or(Error::<T>::NotRegistered)
				})?;
			}

			Self::deposit_event(Event::ServiceTerminated {
				owner: caller.clone(),
				service_id,
				blueprint_id,
			});
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Call a job in the service with the provided arguments.
		///
		/// # Permissions
		///
		/// * Must be signed by the service owner or a permitted caller
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call
		/// * `service_id` - The service identifier
		/// * `job` - The job index to call
		/// * `args` - The arguments to pass to the job
		///
		/// # Errors
		///
		/// * [`Error::ServiceNotFound`] - The service_id does not exist
		/// * [`Error::JobDefinitionNotFound`] - The job index is invalid
		/// * [`Error::MaxFieldsExceeded`] - Too many arguments provided
		/// * [`Error::TypeCheck`] - Arguments fail type checking
		/// * [`Error::InvalidJobCallInput`] - Job call was rejected by hooks
		/// * [`DispatchError::BadOrigin`] - Caller is not owner or permitted caller
		#[pallet::weight(T::WeightInfo::call())]
		pub fn call(
			origin: OriginFor<T>,
			#[pallet::compact] service_id: u64,
			#[pallet::compact] job: u8,
			args: Vec<Field<T::Constraints, T::AccountId>>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let service = Self::services(service_id)?;
			let blueprint_id = service.blueprint;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;
			let is_permitted_caller = service.permitted_callers.iter().any(|v| v == &caller);
			ensure!(service.owner == caller || is_permitted_caller, DispatchError::BadOrigin);

			let job_def =
				blueprint.jobs.get(usize::from(job)).ok_or(Error::<T>::JobDefinitionNotFound)?;
			let bounded_args = BoundedVec::<_, MaxFieldsOf<T>>::try_from(args.clone())
				.map_err(|_| Error::<T>::MaxFieldsExceeded)?;
			let job_call = JobCall { service_id, job, args: bounded_args };

			job_call.type_check(job_def).map_err(Error::<T>::TypeCheck)?;
			let call_id = Self::next_job_call_id();

			let (allowed, _weight) =
				Self::on_job_call_hook(&blueprint, blueprint_id, service_id, job, call_id, &args)?;

			ensure!(allowed, Error::<T>::InvalidJobCallInput);

			JobCalls::<T>::insert(service_id, call_id, job_call);
			NextJobCallId::<T>::set(call_id.saturating_add(1));
			Self::deposit_event(Event::JobCalled {
				caller: caller.clone(),
				service_id,
				call_id,
				job,
				args,
			});
			// TODO: add weight for the call to the total weight.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Submit a result for a previously called job.
		///
		/// # Arguments
		///
		/// * `service_id` - ID of the service
		/// * `call_id` - ID of the job call
		/// * `result` - Vector of result fields
		///
		/// # Permissions
		///
		/// * Caller must be an operator of the service
		///
		/// # Errors
		///
		/// * [`Error::ServiceNotFound`] - The service_id does not exist
		/// * [`Error::JobCallNotFound`] - The call_id does not exist
		/// * [`Error::JobDefinitionNotFound`] - The job index is invalid
		/// * [`Error::MaxFieldsExceeded`] - Too many result fields provided
		/// * [`Error::TypeCheck`] - Result fields fail type checking
		/// * [`Error::InvalidJobResult`] - Job result was rejected by hooks
		/// * [`DispatchError::BadOrigin`] - Caller is not an operator
		#[pallet::weight(T::WeightInfo::submit_result())]
		pub fn submit_result(
			origin: OriginFor<T>,
			#[pallet::compact] service_id: u64,
			#[pallet::compact] call_id: u64,
			result: Vec<Field<T::Constraints, T::AccountId>>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let job_call = Self::job_calls(service_id, call_id)?;
			let service = Self::services(job_call.service_id)?;
			let blueprint_id = service.blueprint;
			let (_, blueprint) = Self::blueprints(blueprint_id)?;

			let is_operator = service.operators.iter().any(|(v, _)| v == &caller);
			ensure!(is_operator, DispatchError::BadOrigin);
			let operator_preferences = Operators::<T>::get(blueprint_id, &caller)?;

			let job_def = blueprint
				.jobs
				.get(usize::from(job_call.job))
				.ok_or(Error::<T>::JobDefinitionNotFound)?;

			let bounded_result = BoundedVec::<_, MaxFieldsOf<T>>::try_from(result.clone())
				.map_err(|_| Error::<T>::MaxFieldsExceeded)?;

			let job_result = JobCallResult { service_id, call_id, result: bounded_result };
			job_result.type_check(job_def).map_err(Error::<T>::TypeCheck)?;

			let (allowed, _weight) = Self::on_job_result_hook(
				&blueprint,
				blueprint_id,
				service_id,
				job_call.job,
				call_id,
				&operator_preferences,
				&job_call.args,
				&result,
			)?;

			ensure!(allowed, Error::<T>::InvalidJobResult);

			JobResults::<T>::insert(service_id, call_id, job_result);
			Self::deposit_event(Event::JobResultSubmitted {
				operator: caller.clone(),
				service_id,
				call_id,
				job: job_call.job,
				result,
			});
			// TODO: add weight for the call to the total weight.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Slash an operator's stake for a service by scheduling a deferred slashing action.
		///
		/// This function schedules a deferred slashing action against an operator's stake for a specific service.
		/// The slash is not applied immediately, but rather queued to be executed by another entity later.
		///
		/// # Permissions
		///
		/// * The caller must be an authorized Slash Origin for the target service, as determined by
		///   `query_slashing_origin`. If no slashing origin is set, or the caller does not match, the call
		///   will fail.
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the call. Must be signed by an authorized Slash Origin.
		/// * `offender` - The account ID of the operator to be slashed.
		/// * `service_id` - The ID of the service for which to slash the operator.
		/// * `percent` - The percentage of the operator's exposed stake to slash, as a `Percent` value.
		///
		/// # Errors
		///
		/// * `NoSlashingOrigin` - No slashing origin is set for the service
		/// * `BadOrigin` - Caller is not the authorized slashing origin
		/// * `OffenderNotOperator` - Target account is not an operator for this service
		/// * `OffenderNotActiveOperator` - Target operator is not currently active
		pub fn slash(
			origin: OriginFor<T>,
			offender: T::AccountId,
			#[pallet::compact] service_id: u64,
			#[pallet::compact] percent: Percent,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let service = Self::services(service_id)?;
			let (maybe_slashing_origin, _used_weight) = Self::query_slashing_origin(&service)?;
			let slashing_origin = maybe_slashing_origin.ok_or(Error::<T>::NoSlashingOrigin)?;
			ensure!(slashing_origin == caller, DispatchError::BadOrigin);

			let (operator, restake_percent) =
				match service.operators.iter().find(|(operator, _)| operator == &offender) {
					Some((operator, restake_percent)) => (operator, restake_percent),
					None => return Err(Error::<T>::OffenderNotOperator.into()),
				};
			let operator_is_active = T::OperatorDelegationManager::is_operator_active(&offender);
			ensure!(operator_is_active, Error::<T>::OffenderNotActiveOperator);

			let total_own_stake = T::OperatorDelegationManager::get_operator_stake(operator);
			// Only take the exposed restake percentage for this service.
			let own_stake = restake_percent.mul_floor(total_own_stake);
			let delegators = T::OperatorDelegationManager::get_delegators_for_operator(operator);
			let exposed_stake = percent.mul_floor(own_stake);
			let others_slash = delegators
				.into_iter()
				.map(|(delegator, stake, _asset_id)| (delegator, percent.mul_floor(stake)))
				.collect::<Vec<_>>();
			let total_slash =
				others_slash.iter().fold(exposed_stake, |acc, (_, slash)| acc + *slash);
			// TODO: take into account the delegators' asset kind.
			// for now, we treat all assets equally, which is not the case in reality.
			let unapplied_slash = UnappliedSlash {
				service_id,
				operator: offender.clone(),
				own: exposed_stake,
				others: others_slash,
				reporters: Vec::from([caller]),
				payout: total_slash,
			};

			let index = Self::next_unapplied_slash_index();
			let era = T::OperatorDelegationManager::get_current_round();
			UnappliedSlashes::<T>::insert(era, index, unapplied_slash);
			NextUnappliedSlashIndex::<T>::set(index.saturating_add(1));

			Self::deposit_event(Event::<T>::UnappliedSlash {
				index,
				operator: offender.clone(),
				blueprint_id: service.blueprint,
				service_id,
				amount: total_slash,
				era,
			});

			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Disputes and removes an [UnappliedSlash] from storage.
		///
		/// The slash will not be applied once disputed and is permanently removed.
		///
		/// # Permissions
		///
		/// * Caller must be the authorized dispute origin for the service
		///
		/// # Arguments
		///
		/// * `origin` - Origin of the call
		/// * `era` - Era containing the slash to dispute  
		/// * `index` - Index of the slash within the era
		///
		/// # Errors
		///
		/// * [Error::NoDisputeOrigin] - Service has no dispute origin configured
		/// * [DispatchError::BadOrigin] - Caller is not the authorized dispute origin
		///

		pub fn dispute(
			origin: OriginFor<T>,
			#[pallet::compact] era: u32,
			#[pallet::compact] index: u32,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let unapplied_slash = Self::unapplied_slashes(era, index)?;
			let service = Self::services(unapplied_slash.service_id)?;
			let (maybe_dispute_origin, _used_weight) = Self::query_dispute_origin(&service)?;
			let dispute_origin = maybe_dispute_origin.ok_or(Error::<T>::NoDisputeOrigin)?;
			ensure!(dispute_origin == caller, DispatchError::BadOrigin);

			UnappliedSlashes::<T>::remove(era, index);

			Self::deposit_event(Event::<T>::SlashDiscarded {
				index,
				operator: unapplied_slash.operator,
				blueprint_id: service.blueprint,
				service_id: unapplied_slash.service_id,
				amount: unapplied_slash.payout,
				era,
			});

			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}

		/// Updates the Master Blueprint Service Manager by adding a new revision.
		///
		/// # Permissions
		///
		/// * Caller must be an authorized Master Blueprint Service Manager Update Origin
		///
		/// # Arguments
		///
		/// * `origin` - Origin of the call
		/// * `address` - New manager address to add
		///
		/// # Errors
		///
		/// * [Error::MaxMasterBlueprintServiceManagerVersionsExceeded] - Maximum number of revisions reached
		pub fn update_master_blueprint_service_manager(
			origin: OriginFor<T>,
			address: H160,
		) -> DispatchResultWithPostInfo {
			T::MasterBlueprintServiceManagerUpdateOrigin::ensure_origin(origin)?;

			MasterBlueprintServiceManagerRevisions::<T>::try_append(address)
				.map_err(|_| Error::<T>::MaxMasterBlueprintServiceManagerVersionsExceeded)?;

			let revision = Self::mbsm_latest_revision();
			Self::deposit_event(Event::<T>::MasterBlueprintServiceManagerRevised {
				revision,
				address,
			});

			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
		}
	}
}
