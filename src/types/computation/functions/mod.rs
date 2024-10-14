//! # Distance Functions
//!
//! This crate provides various mathematical primitives, periodic surfaces, and custom geometry functions.
//!
//! ## Simple
//! - [`XCoord`](functions::XCoord)
//! - [`YCoord`](functions::YCoord)
//! - [`ZCoord`](functions::ZCoord)
//!
//! ## Periodic Surface Functions
//! - [`Gyroid`](functions::Gyroid)
//! - [`Neovius`](functions::Neovius)
//! - [`SchwarzP`](functions::SchwarzP)
//!
//! ## Custom
//! - [`CustomFunction`](functions::CustomFunction)
//! - [`CustomGeometry`](functions::CustomGeometry)

// Modules
mod coordinates;
pub use coordinates::*;

mod gyroid;
pub use gyroid::*;
mod neovius;
pub use neovius::*;
mod schwarz;
pub use schwarz::*;

mod custom_function;
pub use custom_function::*;
mod custom_sdf;
pub use custom_sdf::*;
