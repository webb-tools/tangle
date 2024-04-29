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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, ReservableCurrency},
	PalletId,
};
use frame_system::pallet_prelude::*;
use sp_runtime::{
	traits::{Get, Zero},
	DispatchResult,
};
use sp_std::prelude::*;

mod functions;
mod impls;
mod rpc;
mod types;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod mock_evm;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
use crate::types::BalanceOf;

pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet(dev_mode)]
pub mod module {
	use super::*;
	use scale_info::prelude::fmt::Debug;
	use sp_runtime::Saturating;
	use tangle_primitives::jobs::v2::{
		ApprovalPrefrence, Field, MaxFields, ServiceBlueprint, TypeCheckError,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The origin which may set filter.
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// `PalletId` for the jobs pallet.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The service blueprint was not found.
		BlueprintNotFound,
		/// The caller is already registered as a service provider.
		AlreadyRegistered,
		/// The caller does not have the requirements to be a service provider.
		InvalidRegistrationInput,
		/// The caller is not registered as a service provider.
		NotRegistered,
		/// The service request was not found.
		ServiceRequestNotFound,
		/// An error occurred while type checking the provided input input.
		TypeCheck(TypeCheckError),
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
		/// A new service provider has been registered.
		Registered {
			/// The account that registered as a service provider.
			provider: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The approval preference for the service provider for this specific blueprint.
			approval_preference: ApprovalPrefrence,
			/// The arguments used for registration.
			registration_args: Vec<Field<T::AccountId>>,
		},
		/// A service provider has been deregistered.
		Deregistered {
			/// The account that deregistered as a service provider.
			provider: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},
		/// The approval preference for a service provider has been updated.
		ApprovalPreferenceUpdated {
			/// The account that updated the approval preference.
			provider: T::AccountId,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The new approval preference.
			approval_preference: ApprovalPrefrence,
		},

		/// A new service has been requested.
		ServiceRequested {
			/// The account that requested the service.
			owner: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The list of service providers that need to approve the service.
			required_approvals: Vec<T::AccountId>,
			/// The list of service providers that atomaticaly approved the service.
			approved: Vec<T::AccountId>,
		},
		/// A service request has been approved.
		ServiceRequestApproved {
			/// The account that approved the service.
			provider: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The list of service providers that need to approve the service.
			required_approvals: Vec<T::AccountId>,
			/// The list of service providers that atomaticaly approved the service.
			approved: Vec<T::AccountId>,
		},
		/// A service request has been rejected.
		ServiceRequestRejected {
			/// The account that rejected the service.
			provider: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
		},

		/// A service request has been updated or modified.
		ServiceRequestUpdated {
			/// The account that requested the service.
			owner: T::AccountId,
			/// The ID of the service request.
			request_id: u64,
			/// The ID of the service blueprint.
			blueprint_id: u64,
			/// The list of service providers that need to approve the service.
			required_approvals: Vec<T::AccountId>,
			/// The list of service providers that atomaticaly approved the service.
			approved: Vec<T::AccountId>,
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
			/// The ID of the job.
			job: u8,
			/// The arguments of the job.
			args: Vec<Field<T::AccountId>>,
		},
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Counters

	/// The next free ID for a service blueprint.
	#[pallet::storage]
	#[pallet::getter(fn next_blueprint_id)]
	pub type NextBlueprintId<T> = StorageValue<_, u64, ValueQuery>;

	/// The next free ID for a service request.
	#[pallet::storage]
	#[pallet::getter(fn next_service_request_id)]
	pub type NextServiceRequestId<T> = StorageValue<_, u64, ValueQuery>;

	/// The next free ID for a service.
	#[pallet::storage]
	#[pallet::getter(fn next_service_id)]
	pub type NextServiceId<T> = StorageValue<_, u64, ValueQuery>;

	/// The service blueprints along with their owner.
	#[pallet::storage]
	#[pallet::getter(fn blueprints)]
	pub type Blueprints<T: Config> = StorageMap<
		_,
		Identity,
		u64,
		(T::AccountId, ServiceBlueprint),
		ResultQuery<Error<T>::BlueprintNotFound>,
	>;

	/// The service providers for a specific service blueprint.
	#[pallet::storage]
	#[pallet::getter(fn service_providers)]
	pub type ServiceProviders<T: Config> = StorageDoubleMap<
		_,
		Identity,
		u64,
		Identity,
		T::AccountId,
		ApprovalPrefrence,
		ResultQuery<Error<T>::NotRegistered>,
	>;

	/// The service requests along with their owner.
	/// Owner -> Request ID -> Service Request
	#[pallet::storage]
	#[pallet::getter(fn service_requests)]
	pub type ServiceRequests<T: Config> = StorageDoubleMap<
		_,
		Identity,
		T::AccountId,
		Identity,
		u64,
		ServiceRequest<T::AccountId, T::BlockNumber>,
		ResultQuery<Error<T>::ServiceRequestNotFound>,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new service blueprint.
		///
		/// A Service Blueprint is a template for a service that can be instantiated later on by a
		/// user.
		///
		/// # Parameters
		/// - `origin`: The account that is creating the service blueprint.
		/// - `blueprint`: The blueprint of the service.
		pub fn create_blueprint(
			origin: OriginFor<T>,
			blueprint: ServiceBlueprint,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let blueprint_id = NextBlueprintId::<T>::get();
			Blueprints::<T>::insert(blueprint_id, (owner.clone(), blueprint));
			NextBlueprintId::<T>::set(blueprint_id.saturating_add(1));

			Self::deposit_event(Event::BlueprintCreated { owner, blueprint_id });
			Ok(())
		}

		/// Register the caller as a service provider for a specific blueprint.
		///
		/// The caller may require an approval first before they can accept to provide the service
		/// for the users.
		pub fn register(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
			approval_preference: ApprovalPrefrence,
			// NOTE: add role profiles here.
			// profile: Profile,
			registration_args: Vec<Field<T::AccountId>>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let (_, blueprint) = Blueprints::<T>::get(blueprint_id)?;
			let already_registered = ServiceProviders::<T>::contains_key(blueprint_id, &caller);
			ensure!(!already_registered, Error::<T>::AlreadyRegistered);
			// TODO: check if the caller has the valid requirements to be a service provider.
			// TODO: call into EVM here.
			blueprint
				.type_check_registration(&registration_args)
				.map_err(Error::<T>::TypeCheck)?;
			ServiceProviders::<T>::insert(blueprint_id, &caller, approval_preference.clone());

			Self::deposit_event(Event::Registered {
				provider: caller.clone(),
				blueprint_id,
				approval_preference,
				registration_args,
			});

			Ok(())
		}

		/// Deregister the caller from being a service provider for the service blueprint
		/// so that, no more services will assigned to the caller for this specific blueprint.
		/// Note that, the caller needs to keep providing service for other active service
		/// that uses this blueprint, until the end of service time, otherwise they may get reported
		/// and slashed.
		pub fn deregister(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			ensure!(Blueprints::<T>::contains_key(blueprint_id), Error::<T>::BlueprintNotFound);
			let registered = ServiceProviders::<T>::contains_key(blueprint_id, &caller);
			ensure!(registered, Error::<T>::NotRegistered);
			// TODO: check if the caller is not providing any service for the blueprint.
			ServiceProviders::<T>::remove(blueprint_id, &caller);

			Self::deposit_event(Event::Deregistered { provider: caller.clone(), blueprint_id });
			Ok(())
		}

		/// Update the approval preference for the caller for a specific service blueprint.
		///
		/// See [`Self::register`] for more information.
		pub fn update_approval_preference(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
			approval_preference: ApprovalPrefrence,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			ensure!(Blueprints::<T>::contains_key(blueprint_id), Error::<T>::BlueprintNotFound);
			let updated_approval_preference = approval_preference.clone();
			ServiceProviders::<T>::try_mutate_exists(
				blueprint_id,
				&caller,
				|current_approval_preference| {
					current_approval_preference
						.as_mut()
						.map(|v| *v = updated_approval_preference)
						.ok_or(Error::<T>::NotRegistered)
				},
			)?;

			Self::deposit_event(Event::ApprovalPreferenceUpdated {
				provider: caller.clone(),
				blueprint_id,
				approval_preference,
			});
			Ok(())
		}
		/// Request a new service to be initiated using the provided blueprint with a list of
		/// service providers that will run your service. Optionally, you can specifiy who is permitted caller
		/// of this service, by default anyone could use this service.
		///
		/// Note that, if anyone of the participants set their [`ApprovalPreference`] to `ApprovalPreference::RequireApproval`
		/// you will need to wait until they are approve your request, otherwise (if none), the service is initiated immediately.
		pub fn request(
			origin: OriginFor<T>,
			#[pallet::compact] blueprint_id: u64,
			permitted_callers: Vec<T::AccountId>,
			service_providers: Vec<T::AccountId>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let (_, blueprint) = Blueprints::<T>::get(blueprint_id)?;
			// TODO: check if all the service providers are registered.
			// TODO: check if any of the service providers are required approval.
			let mut required_approvals = Vec::new();
			for provider in &service_providers {
				let approval_preference = ServiceProviders::<T>::get(blueprint_id, provider)?;
				if approval_preference == ApprovalPrefrence::Required {
					required_approvals.push(provider.clone());
				}
			}
			// TODO: create a new service, and store it.
			// TODO: notify the service providers, by storing the service id in their storage.
			// TODO: emit an event.
			Self::deposit_event(Event::ServiceRequested {
				owner: caller.clone(),
				request_id: 0,
				blueprint_id,
				required_approvals: vec![],
				approved: vec![],
			});
			todo!()
		}

		/// Approve a service request, so that the service can be initiated.
		pub fn approve(origin: OriginFor<T>, #[pallet::compact] request_id: u64) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// TODO: check if the service exists.
			// TODO: check if the caller is a service provider.
			// TODO: check if the caller is required to approve the service.
			// TODO: approve the service.
			Self::deposit_event(Event::ServiceRequestApproved {
				provider: caller.clone(),
				request_id,
				blueprint_id: 0,
				required_approvals: vec![],
				approved: vec![],
			});
			// TODO: check if all the required approvals are done.
			// TODO: initiate the service.
			Self::deposit_event(Event::ServiceInitiated {
				owner: todo!(),
				request_id,
				service_id: 0,
				blueprint_id: 0,
			});
			todo!()
		}

		/// Reject a service request.
		/// The service will not be initiated, and the requester will need to update the service request.
		pub fn reject(origin: OriginFor<T>, #[pallet::compact] request_id: u64) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// TODO: check if the service request exists.
			// TODO: check if the caller is a service provider.
			// TODO: reject the service.
			// TODO: emit an event.
			Self::deposit_event(Event::ServiceRequestRejected {
				provider: caller.clone(),
				request_id,
				blueprint_id: 0,
			});
			todo!()
		}

		/// Update the service request.
		/// The owner of the service request can update the service request, and the service providers will need to approve it again.
		pub fn update_request(
			origin: OriginFor<T>,
			#[pallet::compact] request_id: u64,
			permitted_callers: Vec<T::AccountId>,
			service_providers: Vec<T::AccountId>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// TODO: check if the service request exists.
			// TODO: check if the caller is the owner of the service request.
			// TODO: check if all the service providers are registered.
			// TODO: check if any of the service providers are required approval.
			// TODO: update the service request.
			// TODO: notify the service providers, by storing the service id in their storage.
			// TODO: emit an event.
			Self::deposit_event(Event::ServiceRequestUpdated {
				owner: caller.clone(),
				request_id,
				blueprint_id: 0,
				required_approvals: vec![],
				approved: vec![],
			});
			todo!()
		}

		/// Terminates the service by the owner of the service.
		pub fn terminate(
			origin: OriginFor<T>,
			#[pallet::compact] service_id: u64,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// TODO: check if the service exists.
			// TODO: check if the caller is the owner of the service.
			// TODO: terminate the service.
			Self::deposit_event(Event::ServiceTerminated {
				owner: caller.clone(),
				service_id,
				blueprint_id: 0,
			});
			todo!()
		}

		/// Call a Job in the service.
		/// The caller needs to be the owner of the service, or a permitted caller.
		pub fn job_call(
			origin: OriginFor<T>,
			#[pallet::compact] service_id: u64,
			#[pallet::compact] job: u8,
			args: BoundedVec<Field<T::AccountId>, MaxFields>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// TODO: check if the service exists.
			// TODO: check if the caller is the owner of the service, or a permitted caller.
			// TODO: check if the job exists.
			// TODO: check if the job input is correct.
			// TODO: call the job.
			// TODO: emit an event.
			Self::deposit_event(Event::JobCalled {
				caller: caller.clone(),
				service_id,
				call_id: 0,
				job,
				args: args.into(),
			});
			todo!()
		}

		/// Submit the job result by using the call id.
		/// The caller needs to be one of the service providers for this specific service.
		pub fn job_submit(
			origin: OriginFor<T>,
			#[pallet::compact] call_id: u64,
			result: BoundedVec<Field<T::AccountId>, MaxFields>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// TODO: check if the call exists.
			// TODO: check if the service exists, from the call_id.
			// TODO: check if the caller is a service provider.
			// TODO: check if the caller is a service provider for this specific service.
			// TODO: check if the job result is correct.
			// TODO: verify the job result.
			// TODO: store the job result.
			// TODO: emit an event.
			todo!()
		}
	}
}
