
//! Autogenerated weights for pallet_babe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_babe
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/pallet_babe.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_babe.
pub trait WeightInfo {
	fn check_equivocation_proof(x: u32, ) -> Weight;
}

/// Weights for pallet_babe using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AllfeatWeight<T> {
	/// The range of component `x` is `[0, 1]`.
	fn check_equivocation_proof(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 209_351_000 picoseconds.
		Weight::from_parts(214_291_593, 0)
			// Standard Error: 67_063
			.saturating_add(Weight::from_parts(236_906, 0).saturating_mul(x.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `x` is `[0, 1]`.
	fn check_equivocation_proof(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 209_351_000 picoseconds.
		Weight::from_parts(214_291_593, 0)
			// Standard Error: 67_063
			.saturating_add(Weight::from_parts(236_906, 0).saturating_mul(x.into()))
	}
}
