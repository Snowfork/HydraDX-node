// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

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

//! Autogenerated weights for pallet_asset_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-23, STEPS: 10, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_asset_registry
// --output=weights.rs
// --extrinsic=*

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn register() -> Weight;
	fn update() -> Weight;
	fn register_external() -> Weight;
	fn ban_asset() -> Weight;
	fn unban_asset() -> Weight;
}
/// Weights for pallet_asset_registry using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: AssetRegistry Assets (r:1 w:1)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(133), added: 2608, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetIds (r:1 w:1)
	// Proof: AssetRegistry AssetIds (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	// Storage: AssetRegistry LocationAssets (r:1 w:1)
	// Proof: AssetRegistry LocationAssets (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetLocations (r:0 w:1)
	// Proof: AssetRegistry AssetLocations (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	fn register() -> Weight {
		// Minimum execution time: 39_013 nanoseconds.
		Weight::from_parts(39_795_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: AssetRegistry Assets (r:1 w:1)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(133), added: 2608, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetIds (r:1 w:2)
	// Proof: AssetRegistry AssetIds (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetLocations (r:1 w:1)
	// Proof: AssetRegistry AssetLocations (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry LocationAssets (r:0 w:1)
	// Proof: AssetRegistry LocationAssets (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	fn update() -> Weight {
		// Minimum execution time: 47_430 nanoseconds.
		Weight::from_parts(47_760_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:1)
	// Proof: AssetRegistry NextAssetId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: AssetRegistry LocationAssets (r:1 w:1)
	// Proof: AssetRegistry LocationAssets (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetLocations (r:0 w:1)
	// Proof: AssetRegistry AssetLocations (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:0 w:1)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(133), added: 2608, mode: MaxEncodedLen)
	fn register_external() -> Weight {
		// Minimum execution time: 63_891 nanoseconds.
		Weight::from_parts(64_922_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}

	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: AssetRegistry BlacklistedAssets (r:1 w:1)
	// Proof: AssetRegistry BlacklistedAssets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	fn ban_asset() -> Weight {
		// Minimum execution time: 22_677 nanoseconds.
		Weight::from_parts(22_950_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AssetRegistry BlacklistedAssets (r:1 w:1)
	// Proof: AssetRegistry BlacklistedAssets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	fn unban_asset() -> Weight {
		// Minimum execution time: 17_460 nanoseconds.
		Weight::from_parts(17_958_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

impl WeightInfo for () {
	// Storage: AssetRegistry Assets (r:1 w:1)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(133), added: 2608, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetIds (r:1 w:1)
	// Proof: AssetRegistry AssetIds (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	// Storage: AssetRegistry LocationAssets (r:1 w:1)
	// Proof: AssetRegistry LocationAssets (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetLocations (r:0 w:1)
	// Proof: AssetRegistry AssetLocations (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	fn register() -> Weight {
		// Minimum execution time: 39_013 nanoseconds.
		Weight::from_parts(39_795_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: AssetRegistry Assets (r:1 w:1)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(133), added: 2608, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetIds (r:1 w:2)
	// Proof: AssetRegistry AssetIds (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetLocations (r:1 w:1)
	// Proof: AssetRegistry AssetLocations (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry LocationAssets (r:0 w:1)
	// Proof: AssetRegistry LocationAssets (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	fn update() -> Weight {
		// Minimum execution time: 47_430 nanoseconds.
		Weight::from_parts(47_760_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:1)
	// Proof: AssetRegistry NextAssetId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: AssetRegistry LocationAssets (r:1 w:1)
	// Proof: AssetRegistry LocationAssets (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetLocations (r:0 w:1)
	// Proof: AssetRegistry AssetLocations (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:0 w:1)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(133), added: 2608, mode: MaxEncodedLen)
	fn register_external() -> Weight {
		// Minimum execution time: 63_891 nanoseconds.
		Weight::from_parts(64_922_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}

	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: AssetRegistry BlacklistedAssets (r:1 w:1)
	// Proof: AssetRegistry BlacklistedAssets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	fn ban_asset() -> Weight {
		// Minimum execution time: 22_677 nanoseconds.
		Weight::from_parts(22_950_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AssetRegistry BlacklistedAssets (r:1 w:1)
	// Proof: AssetRegistry BlacklistedAssets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	fn unban_asset() -> Weight {
		// Minimum execution time: 17_460 nanoseconds.
		Weight::from_parts(17_958_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
