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

pub mod algorithms {
    pub mod marching_cubes;
    mod tables;
}
pub mod types {
    pub mod computation;
    pub mod geometry;
}
pub mod utils {
    pub mod io;
    pub mod logging;
    pub mod math_helper;
}
