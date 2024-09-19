//! # Distance Functions
//!
//! This crate provides various mathematical primitives, periodic surfaces, and custom geometry functions.
//!
//! ## Mathematical
//! - [`XCoord`](distance_functions::XCoord)
//! - [`YCoord`](distance_functions::YCoord)
//! - [`ZCoord`](distance_functions::ZCoord)
//! - [`ClippingPlane`](distance_functions::ClippingPlane)
//!
//! ## Primitives
//! - [`AABB`](distance_functions::AABB)
//! - [`Capsule`](distance_functions::Capsule)
//! - [`Sphere`](distance_functions::Sphere)
//! - [`Torus`](distance_functions::Torus)
//!
//! ## Periodic Surface Functions
//! - [`Gyroid`](distance_functions::Gyroid)
//! - [`Neovius`](distance_functions::Neovius)
//! - [`SchwarzP`](distance_functions::SchwarzP)
//!
//! ## Custom
//! - [`CustomFunction`](distance_functions::CustomFunction)
//! - [`MeshSDF`](distance_functions::MeshSDF)

// Modules
mod coordinates;
pub use coordinates::*;
mod clipping_plane;
pub use clipping_plane::*;

mod aabb;
pub use aabb::*;
mod capsule;
pub use capsule::*;
mod sphere;
pub use sphere::*;
mod torus;
pub use torus::*;

mod gyroid;
pub use gyroid::*;
mod neovius;
pub use neovius::*;
mod schwarz;
pub use schwarz::*;

mod custom;
pub use custom::*;
mod mesh_sdf;
pub use mesh_sdf::*;
