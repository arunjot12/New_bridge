// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_bridge_parachains
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `covid`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_parachains
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/parachains/src/weights.rs
// --template=./.maintain/millau-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge_parachains.
pub trait WeightInfo {
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight;
	fn submit_parachain_heads_with_1kb_proof() -> Weight;
	fn submit_parachain_heads_with_16kb_proof() -> Weight;
}

/// Weights for `pallet_bridge_parachains` that are generated using one of the Bridge testnets.
///
/// Those weights are test only and must never be used in production.
pub struct BridgeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BridgeWeight<T> {
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(14400), max_size:
	/// Some(64), added: 2044, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(14400), max_size:
	/// Some(196), added: 2176, mode: MaxEncodedLen)
	///
	/// The range of component `p` is `[1, 2]`.
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `5143`
		// Minimum execution time: 35_160 nanoseconds.
		Weight::from_parts(36_951_585, 5143)
			// Standard Error: 336_932
			.saturating_add(Weight::from_ref_time(407_557).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(14400), max_size:
	/// Some(64), added: 2044, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(14400), max_size:
	/// Some(196), added: 2176, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `5143`
		// Minimum execution time: 42_276 nanoseconds.
		Weight::from_parts(43_525_000, 5143)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(14400), max_size:
	/// Some(64), added: 2044, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(14400), max_size:
	/// Some(196), added: 2176, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `5143`
		// Minimum execution time: 85_824 nanoseconds.
		Weight::from_parts(87_335_000, 5143)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(14400), max_size:
	/// Some(64), added: 2044, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(14400), max_size:
	/// Some(196), added: 2176, mode: MaxEncodedLen)
	///
	/// The range of component `p` is `[1, 2]`.
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `5143`
		// Minimum execution time: 35_160 nanoseconds.
		Weight::from_parts(36_951_585, 5143)
			// Standard Error: 336_932
			.saturating_add(Weight::from_ref_time(407_557).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(14400), max_size:
	/// Some(64), added: 2044, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(14400), max_size:
	/// Some(196), added: 2176, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `5143`
		// Minimum execution time: 42_276 nanoseconds.
		Weight::from_parts(43_525_000, 5143)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(14400), max_size:
	/// Some(64), added: 2044, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(14400), max_size:
	/// Some(196), added: 2176, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `5143`
		// Minimum execution time: 85_824 nanoseconds.
		Weight::from_parts(87_335_000, 5143)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
