// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! This crate serves as the implementation of the standard gas meter and algebra used in the Aptos VM.
//! It also defines traits that enable composability of gas meters and algebra.

mod algebra;
mod meter;
mod traits;
#[cfg(feature = "benchmark")]
mod unmetered;

pub use algebra::StandardGasAlgebra;
pub use meter::StandardGasMeter;
pub use traits::{AptosGasMeter, GasAlgebra};
#[cfg(feature = "benchmark")]
pub use unmetered::{enable_gas_metering, is_gas_metering_enabled, AptosUnmeteredGasMeter};
