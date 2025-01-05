//! # Distance Functions
//!
//! This crate provides various mathematical primitives, periodic surfaces, and custom geometry functions.
//!
//! ## Simple
//! - [`XDomain`](functions::XDomain)
//! - [`YDomain`](functions::YDomain)
//! - [`ZDomain`](functions::ZDomain)
//!
//! ## Periodic Surface Functions
//! - [`Gyroid`](functions::Gyroid)
//! - [`Neovius`](functions::Neovius)
//! - [`SchwarzP`](functions::SchwarzP)
//!
//! ## Custom
//! - [`CustomGeometry`](functions::CustomMesh)
//! - [`MeshFile`](functions::MeshFile)

// Modules
mod coordinates;
pub use coordinates::*;

mod gyroid;
pub use gyroid::*;
mod neovius;
pub use neovius::*;
mod schwarz;
pub use schwarz::*;

mod custom_sdf;
pub use custom_sdf::*;
mod mesh_file;
pub use mesh_file::*;

mod function_components;
pub use function_components::*;
