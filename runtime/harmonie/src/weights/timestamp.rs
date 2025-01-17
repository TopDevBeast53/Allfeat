
//! Autogenerated weights for pallet_timestamp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `symphonie-node-3`, CPU: `Intel Xeon Processor (Skylake, IBRS)`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_timestamp
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/timestamp.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_timestamp.
pub trait WeightInfo {
	fn set() -> Weight;
	fn on_finalize() -> Weight;
}

/// Weights for pallet_timestamp using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for AllfeatWeight<T> {
	/// Storage: `Timestamp::Now` (r:1 w:1)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Babe::CurrentSlot` (r:1 w:0)
	/// Proof: `Babe::CurrentSlot` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `209`
		//  Estimated: `1493`
		// Minimum execution time: 22_315_000 picoseconds.
		Weight::from_parts(51_258_000, 1493)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `57`
		//  Estimated: `0`
		// Minimum execution time: 7_865_000 picoseconds.
		Weight::from_parts(16_062_000, 0)
	}
}
