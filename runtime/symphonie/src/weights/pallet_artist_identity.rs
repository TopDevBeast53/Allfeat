
//! Autogenerated weights for pallet_artist_identity
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
// --pallet=pallet_artist_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/pallet_artist_identity.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_artist_identity.
pub trait WeightInfo {
	fn update_alias(n: u32, ) -> Weight;
	fn update_bio(n: u32, ) -> Weight;
	fn update_profile_picture(n: u32, ) -> Weight;
	fn update_twitter(n: u32, ) -> Weight;
	fn update_facebook(n: u32, ) -> Weight;
	fn update_instagram(n: u32, ) -> Weight;
	fn update_spotify(n: u32, ) -> Weight;
	fn update_apple_music(n: u32, ) -> Weight;
	fn update_music_styles(n: u32, x: u32, ) -> Weight;
}

/// Weights for pallet_artist_identity using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AllfeatWeight<T> {
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_alias(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_947_000 picoseconds.
		Weight::from_parts(138_632_299, 5103)
			// Standard Error: 14_402
			.saturating_add(Weight::from_parts(101_268, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 512]`.
	fn update_bio(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_916_000 picoseconds.
		Weight::from_parts(140_339_141, 5103)
			// Standard Error: 3_683
			.saturating_add(Weight::from_parts(24_670, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_profile_picture(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 56_133_000 picoseconds.
		Weight::from_parts(139_116_507, 5103)
			// Standard Error: 14_463
			.saturating_add(Weight::from_parts(97_943, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_twitter(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_862_000 picoseconds.
		Weight::from_parts(139_012_615, 5103)
			// Standard Error: 14_494
			.saturating_add(Weight::from_parts(100_007, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_facebook(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_995_000 picoseconds.
		Weight::from_parts(140_202_188, 5103)
			// Standard Error: 14_659
			.saturating_add(Weight::from_parts(94_324, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_instagram(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_991_000 picoseconds.
		Weight::from_parts(139_512_038, 5103)
			// Standard Error: 14_610
			.saturating_add(Weight::from_parts(96_111, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_spotify(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_913_000 picoseconds.
		Weight::from_parts(138_740_104, 5103)
			// Standard Error: 14_403
			.saturating_add(Weight::from_parts(111_427, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_apple_music(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_276_000 picoseconds.
		Weight::from_parts(138_063_454, 5103)
			// Standard Error: 15_225
			.saturating_add(Weight::from_parts(108_209, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// Storage: MusicStyles Styles (r:1 w:0)
	/// Proof: MusicStyles Styles (max_values: Some(1), max_size: Some(34881), added: 35376, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `x` is `[1, 32]`.
	fn update_music_styles(n: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87 + n * (34 ±0) + x * (5 ±0)`
		//  Estimated: `36366`
		// Minimum execution time: 162_300_000 picoseconds.
		Weight::from_parts(141_662_742, 36366)
			// Standard Error: 67_070
			.saturating_add(Weight::from_parts(18_928_175, 0).saturating_mul(n.into()))
			// Standard Error: 9_729
			.saturating_add(Weight::from_parts(19_081, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_alias(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_947_000 picoseconds.
		Weight::from_parts(138_632_299, 5103)
			// Standard Error: 14_402
			.saturating_add(Weight::from_parts(101_268, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 512]`.
	fn update_bio(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_916_000 picoseconds.
		Weight::from_parts(140_339_141, 5103)
			// Standard Error: 3_683
			.saturating_add(Weight::from_parts(24_670, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_profile_picture(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 56_133_000 picoseconds.
		Weight::from_parts(139_116_507, 5103)
			// Standard Error: 14_463
			.saturating_add(Weight::from_parts(97_943, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_twitter(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_862_000 picoseconds.
		Weight::from_parts(139_012_615, 5103)
			// Standard Error: 14_494
			.saturating_add(Weight::from_parts(100_007, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_facebook(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_995_000 picoseconds.
		Weight::from_parts(140_202_188, 5103)
			// Standard Error: 14_659
			.saturating_add(Weight::from_parts(94_324, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_instagram(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_991_000 picoseconds.
		Weight::from_parts(139_512_038, 5103)
			// Standard Error: 14_610
			.saturating_add(Weight::from_parts(96_111, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_spotify(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_913_000 picoseconds.
		Weight::from_parts(138_740_104, 5103)
			// Standard Error: 14_403
			.saturating_add(Weight::from_parts(111_427, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 128]`.
	fn update_apple_music(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `5103`
		// Minimum execution time: 55_276_000 picoseconds.
		Weight::from_parts(138_063_454, 5103)
			// Standard Error: 15_225
			.saturating_add(Weight::from_parts(108_209, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
	/// Proof: ArtistIdentity ArtistMetadata (max_values: None, max_size: Some(1638), added: 4113, mode: MaxEncodedLen)
	/// Storage: MusicStyles Styles (r:1 w:0)
	/// Proof: MusicStyles Styles (max_values: Some(1), max_size: Some(34881), added: 35376, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `x` is `[1, 32]`.
	fn update_music_styles(n: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87 + n * (34 ±0) + x * (5 ±0)`
		//  Estimated: `36366`
		// Minimum execution time: 162_300_000 picoseconds.
		Weight::from_parts(141_662_742, 36366)
			// Standard Error: 67_070
			.saturating_add(Weight::from_parts(18_928_175, 0).saturating_mul(n.into()))
			// Standard Error: 9_729
			.saturating_add(Weight::from_parts(19_081, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
