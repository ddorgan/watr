// Copyright 2023 Watr Foundation
// This file is part of Watr.

// Watr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Watr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Watr.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-10-2-102-127`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=mainnet-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_identity
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./runtime/mainnet/src/weights/pallet_identity.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		Weight::from_ref_time(37_500_000 as u64)
			// Standard Error: 4_307
			.saturating_add(Weight::from_ref_time(478_104 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(57_887_000 as u64)
			// Standard Error: 13_172
			.saturating_add(Weight::from_ref_time(945_507 as u64).saturating_mul(r as u64))
			// Standard Error: 2_630
			.saturating_add(Weight::from_ref_time(861_535 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		Weight::from_ref_time(63_589_000 as u64)
			// Standard Error: 4_295
			.saturating_add(Weight::from_ref_time(5_573_767 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		Weight::from_ref_time(64_270_000 as u64)
			// Standard Error: 41_349
			.saturating_add(Weight::from_ref_time(4_248_792 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `x` is `[1, 100]`.
	fn clear_identity(_r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(120_570_000 as u64)
			// Standard Error: 31_109
			.saturating_add(Weight::from_ref_time(2_662_082 as u64).saturating_mul(s as u64))
			// Standard Error: 31_109
			.saturating_add(Weight::from_ref_time(660_861 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(82_554_000 as u64)
			// Standard Error: 9_170
			.saturating_add(Weight::from_ref_time(261_088 as u64).saturating_mul(r as u64))
			// Standard Error: 1_831
			.saturating_add(Weight::from_ref_time(763_233 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(78_193_000 as u64)
			// Standard Error: 10_353
			.saturating_add(Weight::from_ref_time(177_580 as u64).saturating_mul(r as u64))
			// Standard Error: 2_067
			.saturating_add(Weight::from_ref_time(731_909 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		Weight::from_ref_time(20_132_000 as u64)
			// Standard Error: 2_688
			.saturating_add(Weight::from_ref_time(399_823 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		Weight::from_ref_time(20_948_000 as u64)
			// Standard Error: 2_497
			.saturating_add(Weight::from_ref_time(369_610 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		Weight::from_ref_time(20_784_000 as u64)
			// Standard Error: 2_392
			.saturating_add(Weight::from_ref_time(361_366 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[1, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(64_593_000 as u64)
			// Standard Error: 8_437
			.saturating_add(Weight::from_ref_time(133_035 as u64).saturating_mul(r as u64))
			// Standard Error: 1_600
			.saturating_add(Weight::from_ref_time(1_217_643 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `x` is `[1, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(133_008_000 as u64)
			// Standard Error: 175_653
			.saturating_add(Weight::from_ref_time(445_003 as u64).saturating_mul(r as u64))
			// Standard Error: 34_976
			.saturating_add(Weight::from_ref_time(2_049_783 as u64).saturating_mul(s as u64))
			// Standard Error: 34_976
			.saturating_add(Weight::from_ref_time(361_386 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(78_840_000 as u64)
			// Standard Error: 2_190
			.saturating_add(Weight::from_ref_time(292_126 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(31_400_000 as u64)
			// Standard Error: 1_574
			.saturating_add(Weight::from_ref_time(187_815 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(79_024_000 as u64)
			// Standard Error: 2_023
			.saturating_add(Weight::from_ref_time(306_325 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(57_532_000 as u64)
			// Standard Error: 1_688
			.saturating_add(Weight::from_ref_time(256_028 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
