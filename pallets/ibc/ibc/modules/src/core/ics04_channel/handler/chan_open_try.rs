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

//! Protocol logic specific to ICS4 messages of type `MsgChannelOpenTry`.

use crate::{
	core::{
		ics03_connection::connection::State as ConnectionState,
		ics04_channel::{
			channel::{ChannelEnd, Counterparty, State},
			error::Error,
			events::Attributes,
			handler::{verify::verify_channel_proofs, ChannelIdState, ChannelResult},
			msgs::chan_open_try::MsgChannelOpenTry,
		},
		ics24_host::identifier::ChannelId,
		ics26_routing::context::ReaderContext,
	},
	events::IbcEvent,
	handler::{HandlerOutput, HandlerResult},
	prelude::*,
};

pub(crate) fn process<Ctx>(
	ctx: &Ctx,
	msg: &MsgChannelOpenTry,
) -> HandlerResult<ChannelResult, Error>
where
	Ctx: ReaderContext,
{
	let mut output = HandlerOutput::builder();

	// Unwrap the old channel end (if any) and validate it against the message.
	let (mut new_channel_end, channel_id) = {
		let channel_end = ChannelEnd::new(
			State::Init,
			*msg.channel.ordering(),
			msg.channel.counterparty().clone(),
			msg.channel.connection_hops().clone(),
			msg.counterparty_version.clone(),
		);

		// Channel identifier construction.
		let id_counter = ctx.channel_counter()?;
		let chan_id = ChannelId::new(id_counter);

		output.log(format!("success: generated new channel identifier: {}", chan_id));

		(channel_end, chan_id)
	};

	// An IBC connection running on the local (host) chain should exist.
	if msg.channel.connection_hops().len() != 1 {
		return Err(Error::invalid_connection_hops_length(1, msg.channel.connection_hops().len()));
	}

	let conn = ctx
		.connection_end(&msg.channel.connection_hops()[0])
		.map_err(Error::ics03_connection)?;
	if !conn.state_matches(&ConnectionState::Open) {
		return Err(Error::connection_not_open(msg.channel.connection_hops()[0].clone()));
	}

	let get_versions = conn.versions();
	let version = match get_versions {
		[version] => version,
		_ => return Err(Error::invalid_version_length_connection()),
	};

	let channel_feature = msg.channel.ordering().to_string();
	if !version.is_supported_feature(channel_feature) {
		return Err(Error::channel_feature_not_suported_by_connection());
	}

	// Proof verification in two steps:
	// 1. Setup: build the Channel as we expect to find it on the other party. the port should be
	//    identical with the port we're using; the channel id should not be set since the
	//    counterparty cannot know yet which ID did we choose.
	let expected_counterparty = Counterparty::new(msg.port_id.clone(), None);
	let counterparty = conn.counterparty();
	let ccid = counterparty.connection_id().ok_or_else(|| {
		Error::undefined_connection_counterparty(msg.channel.connection_hops()[0].clone())
	})?;
	let expected_connection_hops = vec![ccid.clone()];

	// The other party should be storing a channel end in this configuration.
	let expected_channel_end = ChannelEnd::new(
		State::Init,
		*msg.channel.ordering(),
		expected_counterparty,
		expected_connection_hops,
		msg.counterparty_version.clone(),
	);

	// 2. Actual proofs are verified now.
	verify_channel_proofs::<Ctx>(
		ctx,
		msg.proofs.height(),
		&new_channel_end,
		&conn,
		&expected_channel_end,
		&msg.proofs.object_proof(),
	)?;

	output.log("success: channel open try ");

	// Transition the channel end to the new state & pick a version.
	new_channel_end.set_state(State::TryOpen);

	let event_attributes = Attributes {
		channel_id: Some(channel_id),
		height: ctx.host_height(),
		port_id: msg.port_id.clone(),
		connection_id: new_channel_end.connection_hops[0].clone(),
		counterparty_port_id: new_channel_end.counterparty().port_id.clone(),
		counterparty_channel_id: new_channel_end.counterparty().channel_id.clone(),
	};

	let result = ChannelResult {
		port_id: msg.port_id.clone(),
		channel_id_state: ChannelIdState::Generated,
		channel_id,
		channel_end: new_channel_end,
	};

	output.emit(IbcEvent::OpenTryChannel(
		event_attributes.try_into().map_err(|_| Error::missing_channel_id())?,
	));

	Ok(output.with_result(result))
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	use test_log::test;

	use crate::{
		core::{
			ics02_client::{context::ClientReader, error as ics02_error},
			ics03_connection::{
				connection::{
					ConnectionEnd, Counterparty as ConnectionCounterparty, State as ConnectionState,
				},
				error as ics03_error,
				msgs::test_util::get_dummy_raw_counterparty,
				version::get_compatible_versions,
			},
			ics04_channel::{
				channel::{ChannelEnd, State},
				error,
				handler::channel_dispatch,
				msgs::{
					chan_open_try::{
						test_util::get_dummy_raw_msg_chan_open_try, MsgChannelOpenTry,
					},
					ChannelMsg,
				},
			},
			ics24_host::identifier::{ChannelId, ClientId, ConnectionId},
		},
		events::IbcEvent,
		mock::{
			client_state::MockClientState,
			context::{MockClientTypes, MockContext},
		},
		timestamp::ZERO_DURATION,
		Height,
	};

	#[test]
	fn chan_open_try_msg_processing() {
		struct Test {
			name: String,
			ctx: MockContext<MockClientTypes>,
			msg: ChannelMsg,
			want_pass: bool,
			match_error: Box<dyn FnOnce(error::ErrorDetail)>,
		}

		// Some general-purpose variable to parametrize the messages and the context.
		let proof_height = 10;
		let conn_id = ConnectionId::new(2);
		let client_id = ClientId::new(&MockClientState::client_type(), 45).unwrap();

		// The context. We'll reuse this same one across all tests.
		let context = MockContext::default();

		// This is the connection underlying the channel we're trying to open.
		let conn_end = ConnectionEnd::new(
			ConnectionState::Open,
			client_id.clone(),
			ConnectionCounterparty::try_from(get_dummy_raw_counterparty()).unwrap(),
			get_compatible_versions(),
			ZERO_DURATION,
		);

		// We're going to test message processing against this message.
		let mut msg =
			MsgChannelOpenTry::try_from(get_dummy_raw_msg_chan_open_try(proof_height)).unwrap();

		// Assumption: an already existing `Init` channel should exist in the context for `msg`, and
		// this channel should depend on connection `conn_id`.
		let chan_id = ChannelId::new(24);
		let hops = vec![conn_id.clone()];
		msg.channel.connection_hops = hops;

		// This message does not assume a channel should already be initialized.
		let msg_vanilla = msg.clone();

		// A preloaded channel end that resides in the context. This is constructed so as to be
		// consistent with the incoming ChanOpenTry message `msg`.
		let correct_chan_end = ChannelEnd::new(
			State::Init,
			*msg.channel.ordering(),
			msg.channel.counterparty().clone(),
			msg.channel.connection_hops().clone(),
			msg.channel.version().clone(),
		);

		let tests: Vec<Test> = vec![
			Test {
				name: "Processing fails because no connection exists in the context".to_string(),
				ctx: context.clone(),
				msg: ChannelMsg::ChannelOpenTry(msg_vanilla.clone()),
				want_pass: false,
				match_error: {
					let connection_id = msg.channel.connection_hops()[0].clone();
					Box::new(move |e| match e {
						error::ErrorDetail::Ics03Connection(e) => {
							assert_eq!(
								e.source,
								ics03_error::ErrorDetail::ConnectionNotFound(
									ics03_error::ConnectionNotFoundSubdetail { connection_id }
								)
							);
						},
						_ => {
							panic!("Expected MissingConnection, instead got {}", e)
						},
					})
				},
			},
			Test {
				name: "Processing fails b/c the context has no client state".to_string(),
				ctx: context
					.clone()
					.with_connection(conn_id.clone(), conn_end.clone())
					.with_channel(msg.port_id.clone(), chan_id, correct_chan_end.clone()),
				msg: ChannelMsg::ChannelOpenTry(msg.clone()),
				want_pass: false,
				match_error: Box::new(|e| match e {
					error::ErrorDetail::Ics02Client(e) => {
						assert_eq!(
							e.source,
							ics02_error::ErrorDetail::ClientNotFound(
								ics02_error::ClientNotFoundSubdetail {
									client_id: ClientId::new(&MockClientState::client_type(), 45)
										.unwrap()
								}
							)
						);
					},
					_ => {
						panic!("Expected MissingClientState, instead got {}", e)
					},
				}),
			},
			Test {
				name: "Processing is successful".to_string(),
				ctx: context
					.clone()
					.with_client(&client_id, Height::new(0, proof_height))
					.with_connection(conn_id.clone(), conn_end.clone())
					.with_channel(msg.port_id.clone(), chan_id, correct_chan_end),
				msg: ChannelMsg::ChannelOpenTry(msg),
				want_pass: true,
				match_error: Box::new(|_| {}),
			},
			Test {
				name: "Processing is successful against an empty context (no preexisting channel)"
					.to_string(),
				ctx: context
					.with_client(&client_id, Height::new(0, proof_height))
					.with_connection(conn_id, conn_end),
				msg: ChannelMsg::ChannelOpenTry(msg_vanilla),
				want_pass: true,
				match_error: Box::new(|_| {}),
			},
		]
		.into_iter()
		.collect();

		for test in tests {
			let res = channel_dispatch(&test.ctx, &test.msg);
			// Additionally check the events and the output objects in the result.
			match res {
				Ok((proto_output, res)) => {
					assert!(
                        test.want_pass,
                        "chan_open_ack: test passed but was supposed to fail for test: {}, \nparams {:?} {:?}",
                        test.name,
                        test.msg,
                        test.ctx.clone()
                    );

					let proto_output = proto_output.with_result(());
					assert!(!proto_output.events.is_empty()); // Some events must exist.

					// The object in the output is a channel end, should have TryOpen state.
					assert_eq!(res.channel_end.state().clone(), State::TryOpen);

					for e in proto_output.events.iter() {
						assert!(matches!(e, &IbcEvent::OpenTryChannel(_)));
						assert_eq!(e.height(), test.ctx.host_height());
					}
				},
				Err(e) => {
					assert!(
                        !test.want_pass,
                        "chan_open_try: did not pass test: {}, \nparams:\n\tmsg={:?}\n\tcontext={:?}\nerror: {:?}",
                        test.name,
                        test.msg,
                        test.ctx.clone(),
                        e,
                    );

					(test.match_error)(e.0);
				},
			}
		}
	}
}
