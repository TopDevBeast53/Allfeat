// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

pub mod genesis;
pub mod helpers;

use crate::chain_specs::helpers::{authority_keys_from_seed, chain_properties};
pub use allfeat_primitives::{AccountId, Signature};
use grandpa_primitives::AuthorityId as GrandpaId;
pub use harmonie_runtime::opaque::{Block, SessionKeys};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_runtime::traits::{IdentifyAccount, Verify};
type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

#[allow(unused)]
// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = sc_service::GenericChainSpec<(), Extensions>;

#[cfg(feature = "harmonie-native")]
pub type HarmonieChainSpec =
	sc_service::GenericChainSpec<harmonie_runtime::RuntimeGenesisConfig, Extensions>;
#[cfg(not(feature = "harmonie-native"))]
pub type HarmonieChainSpec = GenericChainSpec<DummyChainSpec, Extensions>;

pub fn harmonie_config() -> Result<HarmonieChainSpec, String> {
	HarmonieChainSpec::from_json_bytes(&include_bytes!("../../genesis/symphonieV2_raw.json")[..])
}

/*
pub fn _harmonie_live_config() -> HarmonieChainSpec {
	HarmonieChainSpec::from_genesis(
		"Harmonie Testnet Live",
		"harmonie_live",
		ChainType::Live,
		genesis::harmonie_genesis,
		vec![],
		None,
		Some("aft"),
		None,
		Some(chain_properties()),
		Default::default(),
	)
}
*/

/// Development config (single validator Alice)
pub fn development_config() -> HarmonieChainSpec {
	HarmonieChainSpec::from_genesis(
		"Harmonie Development",
		"harmonie_dev",
		ChainType::Development,
		genesis::harmonie_dev_genesis,
		vec![],
		None,
		Some("aft"),
		None,
		Some(chain_properties()),
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::{
		chain_specs::genesis::harmonie_dev_genesis,
		service::{new_full_base, NewFullBase},
	};
	use harmonie_runtime::RuntimeGenesisConfig;
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> RuntimeGenesisConfig {
		genesis::testnet_genesis(
			vec![authority_keys_from_seed("Alice")],
			vec![],
			helpers::get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> HarmonieChainSpec {
		HarmonieChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> HarmonieChainSpec {
		HarmonieChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			harmonie_dev_genesis,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, sync, transaction_pool, .. } =
				new_full_base(config, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				sync,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		harmonie_config().unwrap().build_storage().unwrap();
	}
}
