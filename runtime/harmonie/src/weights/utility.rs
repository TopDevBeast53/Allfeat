
//! Autogenerated weights for pallet_utility
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
// --pallet=pallet_utility
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/utility.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
	fn dispatch_as() -> Weight;
	fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for AllfeatWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 27_792_000 picoseconds.
		Weight::from_parts(48_360_000, 0)
			// Standard Error: 161_685
			.saturating_add(Weight::from_parts(21_526_856, 0).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 17_443_000 picoseconds.
		Weight::from_parts(18_394_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 25_791_000 picoseconds.
		Weight::from_parts(27_105_000, 0)
			// Standard Error: 164_913
			.saturating_add(Weight::from_parts(24_738_674, 0).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 20_613_000 picoseconds.
		Weight::from_parts(51_949_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 38_270_000 picoseconds.
		Weight::from_parts(866_931_318, 0)
			// Standard Error: 358_444
			.saturating_add(Weight::from_parts(20_791_921, 0).saturating_mul(c.into()))
	}
}
