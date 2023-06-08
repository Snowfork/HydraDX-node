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

//! Autogenerated weights for pallet_ema_oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-ema-oracle
// --extra
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// ema_oracle.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_ema_oracle::weights::WeightInfo;

/// Weights for pallet_ema_oracle using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: EmaOracle Accumulator (r:1 w:0)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	fn on_finalize_no_entry() -> Weight {
		// Minimum execution time: 2_014 nanoseconds.
		Weight::from_ref_time(2_141_000 as u64).saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:57 w:57)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 19]`.
	fn on_finalize_multiple_tokens(b: u32) -> Weight {
		// Minimum execution time: 19_873 nanoseconds.
		Weight::from_ref_time(8_565_703 as u64) // Standard Error: 36_032
			.saturating_add(Weight::from_ref_time(11_850_507 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(b as u64)))
	}
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 19]`.
	fn on_trade_multiple_tokens(b: u32) -> Weight {
		// Minimum execution time: 6_793 nanoseconds.
		Weight::from_ref_time(6_821_282 as u64) // Standard Error: 2_178
			.saturating_add(Weight::from_ref_time(279_620 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 19]`.
	fn on_liquidity_changed_multiple_tokens(b: u32) -> Weight {
		// Minimum execution time: 6_767 nanoseconds.
		Weight::from_ref_time(6_898_261 as u64) // Standard Error: 3_262
			.saturating_add(Weight::from_ref_time(269_399 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn get_entry() -> Weight {
		// Minimum execution time: 9_540 nanoseconds.
		Weight::from_ref_time(9_891_000 as u64).saturating_add(T::DbWeight::get().reads(2 as u64))
	}
}
