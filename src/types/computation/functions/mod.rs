//! # Distance Functions
//!
//! This module provide some general functions utils such as coordinates and Triply-Periodic-Surface (TPS) definitions. 
//! 
//! The [`MeshFile`](functions::MeshFile) can be used to load an external [.obj] inside an [`ImplicitModel`](crate::types::computation::model::ImplicitModel).
//!
//! ## Simple
//! - [`XDomain`](functions::XDomain)
//! - [`YDomain`](functions::YDomain)
//! - [`ZDomain`](functions::ZDomain)
//! - [`XYZValue`](functions::XYZValue)
//!
//! ## Periodic Surface Functions
//! - [`Gyroid`](functions::Gyroid)
//! - [`Neovius`](functions::Neovius)
//! - [`SchwarzP`](functions::SchwarzP)
//!
//! ## Custom
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

mod mesh_file;
pub use mesh_file::*;

mod function_components;
pub use function_components::*;
