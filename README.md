 # Imlet

![Build & Test](https://github.com/joelhi/implicit-rs/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/imlet.svg)](https://crates.io/crates/imlet)
[![docs.rs](https://img.shields.io/docsrs/imlet)](https://docs.rs/imlet)

![Periodic Surface Example](media/examples.png)

 `Imlet` (Implicit Modeling Lightweight Exploration Toolkit) is a lightweight and flexible engine for creating 3D geometries through implicit modeling, written in Rust.
 It enables the construction of compound spatial functions that can be evaluated and polygonized to generate geometries.

 ## Overview

 `Imlet` provides tools for defining and combining distance functions, extracting isosurfaces, and exporting the results. At its core, it offers a high-level interface for implicit modeling, including:

 ### Key Features

 - **Functional Modeling**: Create geometries by combining distance functions (e.g., spheres, toruses) and operations (e.g., intersections, unions).
 - **Geometric Types**: Provides core geometric types, like `Vec3`, `Plane`, `Mesh`, and more.
 - **Custom Distance Functions**: Define distance functions mathematically or derive them from external triangle meshes.
 - **Model Serialization**: Save and load implicit models using the `.json` format for sharing and reuse.
 - **Mesh Export/Import**: Export results to `.obj` files or import external `.obj` files to create custom distance functions.
 - **Iso-surfacing**: Efficient iso-surface extraction from discretized scalar fields.
 - **Built-in Viewer** *(optional)*: Visualize mesh outputs quickly using the `viewer` feature built on top of `wgpu`.

 For a more in-depth explanation of the library, see the [docs](https://docs.rs/imlet)

## How to use

### Examples

To run a basic example via the terminal and show the result, run the following.

```cmd
cargo run --release --features viewer --example $example
```

where `$example` is any of the examples in the example dir, like `gyroid`, `interpolation` or `bunny`.

### Build a model

Add via cargo.

```
cargo add imlet
```
 
Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.

The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.

 ```rust
 // Define the model parameters
 let size = 10.0;
 let cell_size = 0.1;
 let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

 // Create an implicit model
 let mut model = ImplicitModel::new();

 // Add a sphere to the model
 let sphere = model
     .add_function(
         "Sphere",
         Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size))
     .unwrap();

 // Add a gyroid function to the model
 let gyroid = model
     .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
     .unwrap();

 // Combine the sphere and gyroid using a Boolean intersection
 let intersection = model
     .add_operation(
         "Intersection",
         BooleanIntersection::new(),
         Some(&[&sphere, &gyroid]))
     .unwrap();

 // Sample a sparse field and generate an iso-surface.
 let config = SparseFieldConfig {
     internal_size: BlockSize::Size64,       // Internal node subdivision.
     leaf_size: BlockSize::Size4,            // Leaf node subdivision.
     sampling_mode: SamplingMode::CORNERS,   // Sampling logic for Leaf node exclusion.
     cell_size,                              // Sampling resolution
 };

 let mut sampler = SparseSampler::builder()
     .with_bounds(bounds)            // Set the bounds for the sampling.
     .with_sparse_config(config)     // Set the sparse field parameters.
     .build()
     .expect("Should be able to build the sampler.");

 sampler
     .sample_field(model)
     .expect("Sampling should work.");

 let mesh = sampler
     .iso_surface(0.0)
     .expect("Extracting iso-surface should work.");

 utils::io::write_obj_file(&mesh, "interpolation_example").unwrap();

 ```

## Roadmap

The project is still in the early phase of development, so expect breaking API changes as the library keeps developing. 
Below is a non-exhaustive list of improvements that are on my radar for the next steps.

### Future Enhancements (2025)
- [ ] Make model serialization compatible with external impls.
- [ ] Python binding or other scripting interface to build and compute models. (For example using [PyO3](https://github.com/PyO3/pyo3))
- [ ] GPU computation of models for faster processing. (For example using [CubeCL](https://github.com/tracel-ai/cubecl))
- [ ] Develop a node editor for visual programming. (For example using [snarl](https://github.com/zakarumych/egui-snarl))

## License

This project is licensed under either of the following:

- [MIT License](LICENSE-MIT) 
- [Apache License, Version 2.0](LICENSE-APACHE)

Choose the one that best suits your needs.