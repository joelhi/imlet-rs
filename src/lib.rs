//! # Imlet
//!
//! `Imlet` is a lightweight toolkit for implicit modeling and geometry generation, written in Rust. It provides tools for creating 3D geometries, offering a set of data structures and algorithms that can be used to model in 3d using an implicit logic in a modular way.
//!
//! ## Overview
//!
//! **Features in short:**
//! * Implicit functions, such as various primitives and periodic surfaces; and operations, such as boolean methods.
//! * Interface to build complex implicit models combining various functions with custom processing.
//! * Set of tools to create and process geometric objects such as `Points`, `Lines` and `Meshes`.
//! * Import OBJ files and compute signed distance fields from arbitrary meshes.
//! * Algorithms to evaluate and extract iso surfaces (as triangle meshes) from implcict models at arbitrary resolutions.
//! * Export OBJ of generated iso surfaces.
//! * Viewer to show generated geometries with some basic post processing tools (WIP)
//!
//! The primary modules of the crate are [`types::geometry`] and [`types::computation`], which supply the tools needed to define geometric types and build implicit models.
//!
//! At the heart of Imlet is the [`ImplicitModel`](types::computation::model::ImplicitModel) struct, which serves as the foundation for creating and evaluating compound functions in 3d space.
//! This struct exposes the main methods used to combine functions and operations into a computation graph, which can then be evaluated and used to generate iso surfaces.
//!
//! For detailed information on how these components work and interact, refer to the [`types`] module documentation.
//!
//! ## Examples
//!
//! **Creating a simple geometry**
//!
//! Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.
//!
//! The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.
//!
//! ```rust
//!
//! use imlet::utils::io::write_obj_file;
//!
//! use imlet::types::geometry::{Vec3, BoundingBox, Sphere};
//! use imlet::types::computation::{
//!     functions::Gyroid,
//!     operations::shape::BooleanIntersection,
//! };
//!
//! use imlet::types::computation::model::ImplicitModel;
//!
//! {
//!
//!     // Define some model parameters
//!     let size: f32 = 10.0;
//!     let cell_size = 0.1;
//!     let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));
//!
//!     // Create an empty model
//!     let mut model = ImplicitModel::new();
//!
//!     // Adda a sphere distance function to the model.
//!     let sphere = model
//!         .add_function(
//!             "Sphere",
//!             Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
//!         )
//!         .unwrap();
//!     
//!     // Add a gyroid distance function to the model.
//!     let gyroid = model
//!         .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
//!         .unwrap();
//!
//!     // Add a difference operation to the model, and feed it the output of the sphere and gyroid distance functions.
//!     let intersection = model
//!         .add_operation_with_inputs(
//!             "Intersection",
//!             BooleanIntersection::new(),
//!             &[&sphere, &gyroid],
//!         )
//!         .unwrap();
//!
//!     // Generate an isosurface at the 0 distance.
//!     let mesh = model.generate_iso_surface(&intersection, &model_space, cell_size)
//!         .unwrap();
//!
//!     // Write the mesh to an obj file.
//!     write_obj_file(&mesh, "output.obj").unwrap();
//! }
//! ```
//!

/// Version of the library
pub const IMLET_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Module with algorithms for iso-surface exctraction.
pub mod algorithms {
    /// Functions for generating iso surface using the marching cubes algorithm.
    pub mod marching_cubes;
    mod tables;
}

/// Module containing the various types used to build implicit models.
pub mod types {
    /// Module for types related to the data and computation of implicit models.
    ///
    /// The models are built using the [`ImplicitModel`](computation::model::ImplicitModel) struct.
    /// This struct represents a computation graph where nodes pass data between each other as inputs and outputs.
    ///
    /// # ImplicitModel
    ///
    /// An `ImplicitModel` represents a continuous function that can be evaluated at any point in space.
    /// It is constructed as a computation graph that defines relationships between different components.
    /// While the model is independent of the spatial discretization, it can be evaluated over a grid of {x, y, z} coordinates to produce a scalar field.
    ///
    /// The graph is made up of various components that process and compute values, resulting in the final scalar value at each sampled point.
    ///
    /// There are three main categories of components in a model:
    /// * Implicit Functions
    /// * Data Operations
    /// * Constants
    ///
    /// ## Implicit Functions
    /// Implicit functions are mathematical functions that take {x, y, z} coordinates as input and return a scalar value.
    /// These functions are typically used to represent distance functions or other field equations, which can be combined to define implicit geometries.
    ///
    /// Implicit functions should implement [`ImplicitFunction`](computation::traits::ImplicitFunction).
    ///
    /// ## Data Operations
    /// Data operations are components that take one or more inputs from other components in the model, perform a computation or transformation, and produce an output.
    /// These operations can modify or combine values to create more complex behavior within the model.
    ///
    /// Operations should implement [`ImplicitOperation`](computation::traits::ImplicitOperation).
    ///
    /// ## Constants
    /// Constants are simple components that represent fixed values. These values remain unchanged and can be passed as inputs to other components.
    ///
    pub mod computation;

    /// Module for general geometry types.
    ///
    /// This module defines the geometric types and structures used to set up and process the results from the implicit models.
    /// These types help manage the spatial aspects of the models and their output, such as meshes, vectors, and other geometric representations.
    pub mod geometry;
}

/// Module providing some basic utilities, mainly IO and logging.
pub mod utils {
    /// Provides some methods to read and write data to and from files such as CSV and OBJ.
    pub mod io;
    /// Small utility methods related to logging.
    pub mod logging;
    /// Additional utilities related to mathematics.
    pub mod math_helper;
}
