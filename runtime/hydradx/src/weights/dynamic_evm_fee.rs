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

//! Autogenerated weights for `pallet_dynamic_evm_fee`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-06, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dmoka-msi-pc`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-dynamic-evm-fee
// --output=dynamic-evm-fee.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_dynamic_evm_fee`.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_dynamic_evm_fee::WeightInfo for HydraWeight<T> {
    /// Storage: `DynamicEvmFee::BaseFeePerGas` (r:1 w:1)
    /// Proof: `DynamicEvmFee::BaseFeePerGas` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `AssetRegistry::NextAssetId` (r:1 w:0)
    /// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `AssetRegistry::LocationAssets` (r:1 w:0)
    /// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
    /// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
    /// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
    /// Storage: `Router::Routes` (r:1 w:0)
    /// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
    /// Storage: `EmaOracle::Oracles` (r:4 w:0)
    /// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    fn on_initialize() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2709`
        //  Estimated: `11598`
        // Minimum execution time: 68_199_000 picoseconds.
        Weight::from_parts(71_750_000, 11598)
            .saturating_add(T::DbWeight::get().reads(10))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}