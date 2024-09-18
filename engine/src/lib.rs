//! # Imlet
//!
//! A lightweight toolkit for implicit geometry generation written in Rust.
//!
//! It offers data structures and algorithms that can be used to build computational models for implicitly defined geometries.
//! The packages [`types::geometry`] and [`types::computation`] provide the core tools to process geometries and build models.
//!
//! ## Example
//!
//! The main struct is the [`types::computation::ImplicitModel`] which can be used to construct a computation graph.

/// Module with algorithms for iso-surface exctraction.
pub mod algorithms {
    /// Functions for generating iso surface using the marching cubes algorithm.
    pub mod marching_cubes;
    mod tables;
}

/// Module containing the various types used to build implicit models.
pub mod types {
    /// Module for types related to data and computation of implicit models.
    pub mod computation;
    /// Module for general geometry types.
    pub mod geometry;
}

/// Module providing some basic utilities, mainly IO and logging.
pub mod utils {
    /// Provides some methods to read and write data to and from files.
    pub mod io;
    pub mod logging;
    pub(crate) mod math_helper;
}
