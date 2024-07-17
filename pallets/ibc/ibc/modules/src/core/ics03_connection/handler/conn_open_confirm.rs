// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Protocol logic specific to processing ICS3 messages of type `MsgConnectionOpenConfirm`.

use crate::{
	core::{
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State},
			error::Error,
			events::Attributes,
			handler::{verify::verify_connection_proof, ConnectionIdState, ConnectionResult},
			msgs::conn_open_confirm::MsgConnectionOpenConfirm,
		},
		ics26_routing::context::ReaderContext,
	},
	events::IbcEvent,
	handler::{HandlerOutput, HandlerResult},
	prelude::*,
};

pub(crate) fn process<Ctx: ReaderContext>(
	ctx: &Ctx,
	msg: MsgConnectionOpenConfirm,
) -> HandlerResult<ConnectionResult, Error> {
	let mut output = HandlerOutput::builder();

	// Validate the connection end.
	let mut conn_end = ctx.connection_end(&msg.connection_id)?;
	// A connection end must be in TryOpen state; otherwise return error.
	if !conn_end.state_matches(&State::TryOpen) {
		// Old connection end is in incorrect state, propagate the error.
		return Err(Error::connection_mismatch(msg.connection_id));
	}

	// Verify proofs. Assemble the connection end as we expect to find it on the counterparty.
	let expected_conn = ConnectionEnd::new(
		State::Open,
		conn_end.counterparty().client_id().clone(),
		Counterparty::new(
			// The counterparty is the local chain.
			conn_end.client_id().clone(),    // The local client identifier.
			Some(msg.connection_id.clone()), // Local connection id.
			ctx.commitment_prefix(),         // Local commitment prefix.
		),
		conn_end.versions().to_vec(),
		conn_end.delay_period(),
	);

	// 2. Pass the details to the verification function.
	verify_connection_proof::<Ctx>(
		ctx,
		msg.proofs.height(),
		&conn_end,
		&expected_conn,
		msg.proofs.height(),
		msg.proofs.object_proof(),
	)?;

	output.log("success: connection verification passed");

	// Transition our own end of the connection to state OPEN.
	conn_end.set_state(State::Open);

	let event_attributes = Attributes {
		connection_id: Some(msg.connection_id.clone()),
		height: ctx.host_height(),
		client_id: conn_end.client_id().clone(),
		counterparty_connection_id: conn_end.counterparty().connection_id.clone(),
		counterparty_client_id: conn_end.counterparty().client_id().clone(),
	};

	let result = ConnectionResult {
		connection_id: msg.connection_id,
		connection_id_state: ConnectionIdState::Reused,
		connection_end: conn_end,
	};

	output.emit(IbcEvent::OpenConfirmConnection(event_attributes.into()));

	Ok(output.with_result(result))
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	use core::str::FromStr;
	use test_log::test;

	use crate::{
		core::{
			ics02_client::context::ClientReader,
			ics03_connection::{
				connection::{ConnectionEnd, Counterparty, State},
				context::ConnectionReader,
				handler::{dispatch, ConnectionResult},
				msgs::{
					conn_open_confirm::{
						test_util::get_dummy_raw_msg_conn_open_confirm, MsgConnectionOpenConfirm,
					},
					ConnectionMsg,
				},
			},
			ics23_commitment::commitment::CommitmentPrefix,
			ics24_host::identifier::ClientId,
		},
		events::IbcEvent,
		mock::context::{MockClientTypes, MockContext},
		timestamp::ZERO_DURATION,
		Height,
	};

	#[test]
	fn conn_open_confirm_msg_processing() {
		struct Test {
			name: String,
			ctx: MockContext<MockClientTypes>,
			msg: ConnectionMsg<MockContext<MockClientTypes>>,
			want_pass: bool,
		}

		let client_id = ClientId::from_str("mock_clientid").unwrap();
		let msg_confirm =
			MsgConnectionOpenConfirm::try_from(get_dummy_raw_msg_conn_open_confirm()).unwrap();
		let counterparty = Counterparty::new(
			client_id.clone(),
			Some(msg_confirm.connection_id.clone()),
			CommitmentPrefix::try_from(b"ibc".to_vec()).unwrap(),
		);

		let context = MockContext::default();

		let incorrect_conn_end_state = ConnectionEnd::new(
			State::Init,
			client_id.clone(),
			counterparty,
			context.get_compatible_versions(),
			ZERO_DURATION,
		);

		let mut correct_conn_end = incorrect_conn_end_state.clone();
		correct_conn_end.set_state(State::TryOpen);

		let tests: Vec<Test> = vec![
			Test {
				name: "Processing fails due to missing connection in context".to_string(),
				ctx: context.clone(),
				msg: ConnectionMsg::ConnectionOpenConfirm(msg_confirm.clone()),
				want_pass: false,
			},
			Test {
				name: "Processing fails due to connections mismatch (incorrect state)".to_string(),
				ctx: context
					.clone()
					.with_client(&client_id, Height::new(0, 10))
					.with_connection(msg_confirm.connection_id.clone(), incorrect_conn_end_state),
				msg: ConnectionMsg::ConnectionOpenConfirm(msg_confirm.clone()),
				want_pass: false,
			},
			Test {
				name: "Processing successful".to_string(),
				ctx: context
					.with_client(&client_id, Height::new(0, 10))
					.with_connection(msg_confirm.connection_id.clone(), correct_conn_end),
				msg: ConnectionMsg::ConnectionOpenConfirm(msg_confirm),
				want_pass: true,
			},
		]
		.into_iter()
		.collect();

		for test in tests {
			let res = dispatch(&test.ctx, test.msg.clone());
			// Additionally check the events and the output objects in the result.
			match res {
				Ok(proto_output) => {
					assert!(
                        test.want_pass,
                        "conn_open_confirm: test passed but was supposed to fail for: {}, \nparams {:?} {:?}",
                        test.name,
                        test.msg.clone(),
                        test.ctx.clone()
                    );

					assert!(!proto_output.events.is_empty()); // Some events must exist.

					// The object in the output is a ConnectionEnd, should have OPEN state.
					let res: ConnectionResult = proto_output.result;
					assert_eq!(res.connection_end.state().clone(), State::Open);

					for e in proto_output.events.iter() {
						assert!(matches!(e, &IbcEvent::OpenConfirmConnection(_)));
						assert_eq!(e.height(), test.ctx.host_height());
					}
				},
				Err(e) => {
					assert!(
						!test.want_pass,
						"conn_open_confirm: failed for test: {}, \nparams {:?} {:?} error: {:?}",
						test.name,
						test.msg,
						test.ctx.clone(),
						e,
					);
				},
			}
		}
	}
}
