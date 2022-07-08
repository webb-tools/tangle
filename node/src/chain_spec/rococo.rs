// Copyright 2022 Webb Technologies Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::chain_spec::*;
use arkworks_setups::{common::setup_params, Curve};
use cumulus_primitives_core::ParaId;
use egg_rococo_runtime::{
	AccountId, AnchorBn254Config, AnchorVerifierBn254Config, AssetRegistryConfig, AuraId, DKGId,
	HasherBn254Config, MerkleTreeBn254Config, MixerBn254Config, MixerVerifierBn254Config,
	EXISTENTIAL_DEPOSIT, MILLIUNIT, UNIT,
};
use hex_literal::hex;
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

pub fn egg_rococo_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "tEGG".into());
	properties.insert("tokenDecimals".into(), 12u32.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Egg Rococo",
		// ID
		"egg-rococo",
		ChainType::Live,
		move || {
			rococo_genesis(
				// root
				hex!["5ebd99141e19db88cd2c4b778d3cc43e3678d40168aaea56f33d2ea31f67463f"].into(),
				vec![
					(
						//1//account
						hex!["28714d0740d6b321ad67b8e1a4edd0b53376f735bd10e4904a2c49167bcb7841"]
							.into(),
						//1//aura
						hex!["28714d0740d6b321ad67b8e1a4edd0b53376f735bd10e4904a2c49167bcb7841"]
							.unchecked_into(),
						//1//dkg (--scheme Ecdsa)
						hex!["03568538f7152c4ee734ad74983e1d86e2329ec100bb06b1c2af0bada2f72ffa28"]
							.unchecked_into(),
					),
					(
						//1//account
						hex!["f2ca12f1d3e0cba599b9f17f5675a1dd2d5d781d7a97d241312acc39e0b6f112"]
							.into(),
						//1//aura
						hex!["f2ca12f1d3e0cba599b9f17f5675a1dd2d5d781d7a97d241312acc39e0b6f112"]
							.unchecked_into(),
						//1//dkg (--scheme Ecdsa)
						hex!["03e620a6e19d236bdfe40ef95b9601483629d0e0097e9a7cfb97e7c99e63da609d"]
							.unchecked_into(),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					hex!["5ebd99141e19db88cd2c4b778d3cc43e3678d40168aaea56f33d2ea31f67463f"].into(),
					hex!["28714d0740d6b321ad67b8e1a4edd0b53376f735bd10e4904a2c49167bcb7841"].into(),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("egg-rococo"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn rococo_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId, DKGId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> egg_rococo_runtime::GenesisConfig {
	let curve_bn254 = Curve::Bn254;

	log::info!("Bn254 x5 w3 params");
	let bn254_x5_3_params = setup_params::<ark_bn254::Fr>(curve_bn254, 5, 3);

	log::info!("Verifier params for mixer");
	let mixer_verifier_bn254_params = {
		let vk_bytes = include_bytes!("../../../verifying_keys/mixer/bn254/verifying_key.bin");
		vk_bytes.to_vec()
	};

	log::info!("Verifier params for anchor");
	let anchor_verifier_bn254_params = {
		let vk_bytes = include_bytes!("../../../verifying_keys/anchor/bn254/2/verifying_key.bin");
		vk_bytes.to_vec()
	};

	egg_rococo_runtime::GenesisConfig {
		system: egg_rococo_runtime::SystemConfig {
			code: egg_rococo_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		sudo: egg_rococo_runtime::SudoConfig { key: Some(root_key) },
		balances: egg_rococo_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, MILLIUNIT * 4_096_000))
				.collect(),
		},
		indices: Default::default(),
		parachain_info: egg_rococo_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: egg_rococo_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: egg_rococo_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura, dkg)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						dkg_session_keys(aura, dkg), // session keys
					)
				})
				.collect(),
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		dkg: egg_rococo_runtime::DKGConfig {
			authorities: invulnerables.iter().map(|x| x.2.clone()).collect::<_>(),
			keygen_threshold: 2,
			signature_threshold: 1,
			authority_ids: invulnerables.iter().map(|x| x.0.clone()).collect::<_>(),
		},
		dkg_proposals: Default::default(),
		asset_registry: AssetRegistryConfig {
			asset_names: vec![],
			native_asset_name: b"WEBB".to_vec(),
			native_existential_deposit: egg_rococo_runtime::EXISTENTIAL_DEPOSIT,
		},
		hasher_bn_254: HasherBn254Config {
			parameters: Some(bn254_x5_3_params.to_bytes()),
			phantom: Default::default(),
		},
		mixer_verifier_bn_254: MixerVerifierBn254Config {
			parameters: Some(mixer_verifier_bn254_params),
			phantom: Default::default(),
		},
		anchor_verifier_bn_254: AnchorVerifierBn254Config {
			parameters: Some(anchor_verifier_bn254_params),
			phantom: Default::default(),
		},
		merkle_tree_bn_254: MerkleTreeBn254Config {
			phantom: Default::default(),
			default_hashes: None,
		},
		mixer_bn_254: MixerBn254Config {
			mixers: vec![(0, 10 * UNIT), (0, 100 * UNIT), (0, 1000 * UNIT)],
		},
		anchor_bn_254: AnchorBn254Config {
			anchors: vec![(0, 10 * UNIT, 2), (0, 100 * UNIT, 2), (0, 1000 * UNIT, 2)],
		},
		treasury: Default::default(),
		vesting: Default::default()
	}
}
