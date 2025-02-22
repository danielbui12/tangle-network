// This file is part of Tangle.
// Copyright (C) 2022-2024 Tangle Foundation.
//
// Tangle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Tangle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Tangle.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2025-02-22, STEPS: `10`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("benchmark")`

// Executed Command:
// target/release/tangle
// benchmark
// --chain=dev
// --steps=10
// --repeat=2
// --pallet=rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn claim_rewards() -> Weight;
	fn update_vault_reward_config() -> Weight;
	fn claim_rewards_other() -> Weight;
	fn manage_asset_reward_vault() -> Weight;
	fn create_reward_vault() -> Weight;
	fn update_decay_config() -> Weight;
	fn update_apy_blocks() -> Weight;
}

/// Weight functions needed for rewards pallet.
/// Weights for `pallet_rewards` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Rewards::RewardVaults` (r:1 w:0)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Rewards::UserRewards` (r:1 w:1)
	/// Proof: `Rewards::UserRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342`
		//  Estimated: `2329`
		// Minimum execution time: 14_171_000 picoseconds.
        Weight::from_parts(125_000_000, 2329)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
	/// Storage: `Rewards::RewardVaults` (r:1 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn update_vault_reward_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `123`
		//  Estimated: `2329`
		// Minimum execution time: 14_171_000 picoseconds.
        Weight::from_parts(95_000_000, 2329)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
	/// Storage: `Rewards::RewardVaults` (r:1 w:0)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Rewards::UserRewards` (r:1 w:1)
	/// Proof: `Rewards::UserRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn claim_rewards_other() -> Weight {
        Weight::from_parts(135_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
	/// Storage: `Rewards::RewardVaults` (r:1 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn manage_asset_reward_vault() -> Weight {
        Weight::from_parts(115_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
	/// Storage: `Rewards::RewardVaults` (r:0 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn create_reward_vault() -> Weight {
        Weight::from_parts(85_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
	/// Storage: `Rewards::DecayConfig` (r:0 w:1)
	/// Proof: `Rewards::DecayConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    fn update_decay_config() -> Weight {
        Weight::from_parts(75_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
	/// Storage: `Rewards::ApyBlocks` (r:0 w:1)
	/// Proof: `Rewards::ApyBlocks` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    fn update_apy_blocks() -> Weight {
        Weight::from_parts(65_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Rewards::RewardVaults` (r:1 w:0)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Rewards::UserRewards` (r:1 w:1)
	/// Proof: `Rewards::UserRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn claim_rewards() -> Weight {
        Weight::from_parts(125_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
	/// Storage: `Rewards::RewardVaults` (r:1 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn update_vault_reward_config() -> Weight {
        Weight::from_parts(95_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
	/// Storage: `Rewards::RewardVaults` (r:1 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Rewards::UserRewards` (r:1 w:1)
	/// Proof: `Rewards::UserRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn claim_rewards_other() -> Weight {
        Weight::from_parts(135_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
	/// Storage: `Rewards::RewardVaults` (r:1 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn manage_asset_reward_vault() -> Weight {
        Weight::from_parts(115_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
	/// Storage: `Rewards::RewardVaults` (r:0 w:1)
	/// Proof: `Rewards::RewardVaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn create_reward_vault() -> Weight {
        Weight::from_parts(85_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
	/// Storage: `Rewards::DecayConfig` (r:0 w:1)
	/// Proof: `Rewards::DecayConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    fn update_decay_config() -> Weight {
        Weight::from_parts(75_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
	/// Storage: `Rewards::ApyBlocks` (r:0 w:1)
	/// Proof: `Rewards::ApyBlocks` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    fn update_apy_blocks() -> Weight {
        Weight::from_parts(65_000_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
}