
//! Autogenerated weights for pallet_recovery
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `symphonie-01`, CPU: `Intel(R) Xeon(R) Platinum 8168 CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_recovery
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/pallet_recovery.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_recovery.
pub trait WeightInfo {
	fn as_recovered() -> Weight;
	fn set_recovered() -> Weight;
	fn create_recovery(n: u32, ) -> Weight;
	fn initiate_recovery() -> Weight;
	fn vouch_recovery(n: u32, ) -> Weight;
	fn claim_recovery(n: u32, ) -> Weight;
	fn close_recovery(n: u32, ) -> Weight;
	fn remove_recovery(n: u32, ) -> Weight;
	fn cancel_recovered() -> Weight;
}

/// Weights for pallet_recovery using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AllfeatWeight<T> {
	/// Storage: Recovery Proxy (r:1 w:0)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3545`
		// Minimum execution time: 38_337_000 picoseconds.
		Weight::from_parts(38_618_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Recovery Proxy (r:0 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 41_858_000 picoseconds.
		Weight::from_parts(42_355_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3816`
		// Minimum execution time: 136_584_000 picoseconds.
		Weight::from_parts(138_207_124, 3816)
			// Standard Error: 9_680
			.saturating_add(Weight::from_parts(334_958, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3854`
		// Minimum execution time: 144_888_000 picoseconds.
		Weight::from_parts(147_778_000, 3854)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 76_031_000 picoseconds.
		Weight::from_parts(85_897_533, 3854)
			// Standard Error: 13_511
			.saturating_add(Weight::from_parts(539_582, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 103_619_000 picoseconds.
		Weight::from_parts(105_025_166, 3854)
			// Standard Error: 8_490
			.saturating_add(Weight::from_parts(130_373, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 108_856_000 picoseconds.
		Weight::from_parts(169_975_963, 3854)
			// Standard Error: 12_015
			.saturating_add(Weight::from_parts(309_472, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 151_588_000 picoseconds.
		Weight::from_parts(153_411_434, 3854)
			// Standard Error: 12_337
			.saturating_add(Weight::from_parts(185_375, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3545`
		// Minimum execution time: 49_142_000 picoseconds.
		Weight::from_parts(49_527_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Recovery Proxy (r:1 w:0)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3545`
		// Minimum execution time: 38_337_000 picoseconds.
		Weight::from_parts(38_618_000, 3545)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Recovery Proxy (r:0 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 41_858_000 picoseconds.
		Weight::from_parts(42_355_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3816`
		// Minimum execution time: 136_584_000 picoseconds.
		Weight::from_parts(138_207_124, 3816)
			// Standard Error: 9_680
			.saturating_add(Weight::from_parts(334_958, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3854`
		// Minimum execution time: 144_888_000 picoseconds.
		Weight::from_parts(147_778_000, 3854)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 76_031_000 picoseconds.
		Weight::from_parts(85_897_533, 3854)
			// Standard Error: 13_511
			.saturating_add(Weight::from_parts(539_582, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 103_619_000 picoseconds.
		Weight::from_parts(105_025_166, 3854)
			// Standard Error: 8_490
			.saturating_add(Weight::from_parts(130_373, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 108_856_000 picoseconds.
		Weight::from_parts(169_975_963, 3854)
			// Standard Error: 12_015
			.saturating_add(Weight::from_parts(309_472, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 151_588_000 picoseconds.
		Weight::from_parts(153_411_434, 3854)
			// Standard Error: 12_337
			.saturating_add(Weight::from_parts(185_375, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3545`
		// Minimum execution time: 49_142_000 picoseconds.
		Weight::from_parts(49_527_000, 3545)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
