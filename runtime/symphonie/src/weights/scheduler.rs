
//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `symphonie-01`, CPU: `Intel(R) Xeon(R) Platinum 8168 CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 17_261 nanoseconds.
		Weight::from_ref_time(17_520_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 17_851 nanoseconds.
		Weight::from_ref_time(29_280_838)
			// Standard Error: 6_464
			.saturating_add(Weight::from_ref_time(1_369_664).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 37_245 nanoseconds.
		Weight::from_ref_time(37_797_000)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 93_546 nanoseconds.
		Weight::from_ref_time(93_807_000)
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(2_197).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	fn service_task_named() -> Weight {
		// Minimum execution time: 44_397 nanoseconds.
		Weight::from_ref_time(44_978_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 37_502 nanoseconds.
		Weight::from_ref_time(37_928_000)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 14_903 nanoseconds.
		Weight::from_ref_time(15_060_000)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 14_889 nanoseconds.
		Weight::from_ref_time(15_066_000)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 70_998 nanoseconds.
		Weight::from_ref_time(81_131_576)
			// Standard Error: 6_892
			.saturating_add(Weight::from_ref_time(1_399_208).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 82_229 nanoseconds.
		Weight::from_ref_time(81_720_036)
			// Standard Error: 6_029
			.saturating_add(Weight::from_ref_time(2_231_539).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 83_739 nanoseconds.
		Weight::from_ref_time(96_759_125)
			// Standard Error: 7_867
			.saturating_add(Weight::from_ref_time(1_419_900).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 88_799 nanoseconds.
		Weight::from_ref_time(90_546_074)
			// Standard Error: 5_387
			.saturating_add(Weight::from_ref_time(2_268_034).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}