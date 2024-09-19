//! # Imlet
//!
//! `Imlet` provides a lightweight toolkit for implicit modeling and geometry generation, written in Rust. It provides tools for creating 3D models defined by spatial functions, offering a set of core data structures and algorithms that can be used to define and process implicit geometries in a modular way.
//!
//! ## Overview
//! 
//! **Features in short:**
//! * Implicit functions and operations to use out the box
//! * Interface to build complex implicit models combining various functions with custom processing.
//! * Set of tools to create and process geometric objects such as `Points`, `Lines` and `Meshes`.
//! * Import OBJ files and compute signed distance fields from arbitrary meshes.
//! * Algorithms to evaluate and extract iso surfaces (as triangle meshes) from implcict models at arbitrary resolutions.
//! * Export OBJ of generated iso surfaces.
//! * Viewer to show generated geometries with some basic post processing tools (WIP)
//! 
//! The primary modules of the crate are [`types::geometry`] and [`types::computation`], which supply the tools needed to define geometric types and build implicit models.
//!
//! At the heart of Imlet is the [`types::computation::ImplicitModel`] struct, which serves as the foundation for creating and evaluating the computation graphs used to define compound functions.
//! This struct exposes the main functions to combine functions and operations into a computation graph, which can then be evaluated in 3d space.
//!
//! For detailed information on how these components work and interact, refer to the [`types`] module documentation.
//!
//! ## Examples
//!
//! **The Very Basic**
//! 
//! The simplest possible computation would be to define two constants, and add them together.
//! 
//! In this example the value is not depending on the x,y,z coordinates, so we just evaluate it once at the origin.
//!
//! ```rust
//! fn main() {
//!
//!     let mut model = ImplicitModel::new();
//!
//!    // Add a constant with a value 1 to the model.
//!    model.add_constant("FirstValue", 1.0).unwrap();
//!
//!    // Add another constant with a value 1 to the model.
//!    model.add_constant("SecondValue", 1.0).unwrap();
//!
//!    // Add an addition operation that reads the two constants and adds them together.
//!    model
//!        .add_operation_with_inputs("Sum", Add::new(), &vec!["FirstValue", "SecondValue"])
//!        .unwrap();
//!
//!    // Evaluate the model reading the output of the Sum operation.
//!    let value = model.evaluate_at("Sum", 0.0, 0.0, 0.0);
//!    println!("The value is {}", value)
//!}
//!
//! ```
//! 
//! This should print 
//! ```shell
//! The value is 2
//! ```
//! to the terminal.
//! 
//! **An Actual Geometry (!)**
//! 
//! Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.
//! 
//! The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.
//!
//! ```rust
//! fn main() {
//! 
//!     // Define some model parameters
//!     let size: f32 = 10.0;
//!     let cell_size = 0.05;
//!     let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));
//!
//!     // Create an empty model
//!     let mut model = ImplicitModel::new();
//!
//!     // Adda a sphere distance function to the model.
//!     model
//!         .add_function(
//!             "Sphere",
//!             Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
//!         )
//!         .unwrap();
//!     
//!     // Add a gyroid distance function to the model.
//!     model
//!         .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
//!         .unwrap();
//!
//!     // Add a difference operation to the model, and feed it the output of the sphere and gyroid distance functions.
//!     model
//!         .add_operation_with_inputs(
//!             "Output",
//!             Intersection::new(),
//!             &vec!["Sphere", "Gyroid"],
//!         )
//!         .unwrap();
//!
//!     // Generate an isosurface at the 0 distance.
//!     let mesh = model.generate_iso_surface("Output", &model_space, cell_size);
//!
//!     // Write the mesh to an obj file.
//!     write_obj_file(&mesh, "output.obj").unwrap();
//! }
//! ```
//!

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
    /// The models are built using the [`computation::ImplicitModel`] struct.
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
    /// ## Data Operations
    /// Data operations are components that take one or more inputs from other components in the model, perform a computation or transformation, and produce an output.
    /// These operations can modify or combine values to create more complex behavior within the model.
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
    /// Provides some methods to read and write data to and from files.
    pub mod io;
    pub mod logging;
    pub mod math_helper;
}
