
//! Autogenerated weights for pallet_office
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-06, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `kacper-HP-ProBook-445-G7`, CPU: `AMD Ryzen 7 4700U with Radeon Graphics`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/substrate
// benchmark
// pallet
// --pallet=pallet_office
// --execution=wasm
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=frame/office/src/weights.rs
// --extrinsic=*
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_office.
pub trait WeightInfo {
	fn set_admin() -> Weight;
	fn set_clerk() -> Weight;
	fn remove_clerk() -> Weight;
	fn execute() -> Weight;
}

/// Weights for pallet_office using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Office Admin (r:1 w:1)
	fn set_admin() -> Weight {
		// Minimum execution time: 25_628 nanoseconds.
		Weight::from_ref_time(26_210_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Office Admin (r:1 w:0)
	// Storage: Office Clerks (r:0 w:1)
	fn set_clerk() -> Weight {
		// Minimum execution time: 27_241 nanoseconds.
		Weight::from_ref_time(27_863_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Office Admin (r:1 w:0)
	// Storage: Office Clerks (r:1 w:1)
	fn remove_clerk() -> Weight {
		// Minimum execution time: 30_267 nanoseconds.
		Weight::from_ref_time(30_627_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Office Clerks (r:1 w:0)
	fn execute() -> Weight {
		// Minimum execution time: 29_666 nanoseconds.
		Weight::from_ref_time(30_006_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Office Admin (r:1 w:1)
	fn set_admin() -> Weight {
		// Minimum execution time: 25_628 nanoseconds.
		Weight::from_ref_time(26_210_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Office Admin (r:1 w:0)
	// Storage: Office Clerks (r:0 w:1)
	fn set_clerk() -> Weight {
		// Minimum execution time: 27_241 nanoseconds.
		Weight::from_ref_time(27_863_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Office Admin (r:1 w:0)
	// Storage: Office Clerks (r:1 w:1)
	fn remove_clerk() -> Weight {
		// Minimum execution time: 30_267 nanoseconds.
		Weight::from_ref_time(30_627_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Office Clerks (r:1 w:0)
	fn execute() -> Weight {
		// Minimum execution time: 29_666 nanoseconds.
		Weight::from_ref_time(30_006_000)
			.saturating_add(RocksDbWeight::get().reads(1))
	}
}
