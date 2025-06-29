//! # Imlet
//!
//! `Imlet` (Implicit Modeling Lightweight Exploration Toolkit) is a lightweight and flexible engine for creating 3D geometries through implicit modeling, written in Rust.
//! It enables the construction of compound spatial functions that can be evaluated and polygonized to generate geometries.
//!
//! ## Overview
//!
//! `Imlet` provides tools for defining and combining distance functions, extracting isosurfaces, and exporting the results. At its core, it offers a high-level interface for implicit modeling, including:
//!
//! ### Key Features
//!
//! - **Functional Modeling**: Create geometries by combining distance functions (e.g., spheres, toruses) and operations (e.g., intersections, unions).
//! - **Geometric Types**: The engine includes the core geometric types, like [Vec3](crate::types::geometry::Vec3), [Plane](crate::types::geometry::Plane), [Mesh](crate::types::geometry::Mesh), and more.
//! - **Custom Distance Functions**: Define distance functions mathematically or derive them from external triangle meshes.
//! - **Model Serialization**: Save and load implicit models using the `.json` format for easy sharing and reuse.
//! - **Mesh Export/Import**: Export results to `.obj` files or import external `.obj` files to create custom distance functions.
//! - **Iso-surfacing**: Efficient iso-surface extraction from discretized scalar fields.
//! - **CLI Interface**: Run saved models and show `.obj` files directly from the command line.
//! - **Built-in Viewer** *(optional)*: Visualize mesh outputs quickly using the `viewer` feature built on top of `wgpu`.
//!
//! The main modules of the crate are [`types::geometry`] and [`types::computation`], which together form the foundation for creating and manipulating implicit models. At the heart of `Imlet` is the [`ImplicitModel`](types::computation::model::ImplicitModel) struct, which represents the computation graph used for modeling.
//!
//! ## Example: Creating a Simple Geometry
//!
//! Here's a basic example demonstrating how to use `Imlet` to combine a sphere and a gyroid using an intersection operation. The result is polygonized and saved as an `.obj` file:
//!
//! ```rust
//! # use imlet::utils::io::write_obj_file;
//! # use imlet::types::geometry::{Vec3, BoundingBox, Sphere};
//! # use imlet::types::computation::{
//! #    functions::Gyroid,
//! #    operations::shape::BooleanIntersection,
//! # };
//! # use imlet::types::computation::model::ImplicitModel;
//! # use imlet::types::computation::data::{SparseField, SparseFieldConfig, SamplingMode, BlockSize};
//! # use imlet::types::computation::data::sampler::{SparseSampler, Sampler};
//!
//! // Define the model parameters
//! let size = 10.0;
//! let cell_size = 0.1;
//! let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));
//!
//! // Create an implicit model
//! let mut model = ImplicitModel::new();
//!
//! // Add a sphere to the model
//! let sphere = model
//!     .add_function(
//!         "Sphere",
//!         Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size))
//!     .unwrap();
//!
//! // Add a gyroid function to the model
//! let gyroid = model
//!     .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
//!     .unwrap();
//!
//! // Combine the sphere and gyroid using a Boolean intersection
//! let intersection = model
//!     .add_operation_with_inputs(
//!         "Intersection",
//!         BooleanIntersection::new(),
//!         &[&sphere, &gyroid])
//!     .unwrap();
//!
//! // Sample a sparse field and generate an iso-surface.
//! let config = SparseFieldConfig {
//!     internal_size: BlockSize::Size64,       // Internal node subdivision.
//!     leaf_size: BlockSize::Size4,            // Leaf node subdivision.
//!     sampling_mode: SamplingMode::CENTRE,    // Sampling logic for Leaf node exclusion.
//! };
//!
//! let mut sampler = SparseSampler::builder()
//!     .with_bounds(bounds)            // Set the bounds for the sampling.
//!     .with_model(model)              // Set the model to sample.
//!     .with_sparse_config(config)     // Set the sparse field parameters.
//!     .build()
//!     .expect("Should be able to build the sampler.");
//!
//! sampler
//!     .sample_field(cell_size, &intersection)
//!     .expect("Sampling should work.");
//!
//! let mesh = sampler
//!     .iso_surface(0.0)
//!     .expect("Extracting iso-surface should work.");
//!
//! ```
//!
//! For detailed usage and API references, explore the module documentation.
//!
//!

/// The current version of the `Imlet` library.
pub const IMLET_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Algorithms for iso-surface extraction.
pub mod algorithms {
    /// Marching cubes algorithm for polygonizing implicit models.
    pub mod marching_cubes;
    mod tables;
}

/// Types for building and processing implicit models.
pub mod types {
    /// Data and computation types for implicit models.
    pub mod computation;
    /// General geometry types for spatial operations and representations.
    pub mod geometry;
}

/// Utility modules for file I/O, logging, and math operations.
pub mod utils {
    /// Read and write data to and from files (e.g., OBJ, CSV).
    pub mod io;
    /// Logging utilities for debugging and tracing.
    pub mod logging;
    /// Mathematical helper functions.
    pub mod math_helper;
}

/// Optional viewer for visualizing generated geometries.
#[cfg(feature = "viewer")]
pub mod viewer;
