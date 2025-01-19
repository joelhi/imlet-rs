### Implicit Modeling Lightweight Exploration Toolkit (Imlet)

![Build & Test](https://github.com/joelhi/implicit-rs/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/imlet.svg)](https://crates.io/crates/imlet)
[![docs.rs](https://img.shields.io/docsrs/imlet)](https://docs.rs/imlet)

![Periodic Surface Example](media/examples.png)

 # Imlet

 `Imlet` (Implicit Modeling Lightweight Exploration Toolkit) is a lightweight and flexible engine for creating 3D geometries through implicit modeling, written in Rust.
 It enables the construction of compound spatial functions that can be evaluated and polygonized to generate geometries.

 ## Overview

 `Imlet` provides tools for defining and combining distance functions, extracting isosurfaces, and exporting the results. At its core, it offers a high-level interface for implicit modeling, including:

 ### Key Features

 - **Functional Modeling**: Create geometries by combining distance functions (e.g., spheres, toruses) and operations (e.g., intersections, unions).
 - **Geometric Types**: Provides core geometric types, like [Vec3](crate::types::geometry::Vec3), [Plane](crate::types::geometry::Plane), [Mesh](crate::types::geometry::Mesh), and more.
 - **Custom Distance Functions**: Define distance functions mathematically or derive them from external triangle meshes.
 - **Model Serialization**: Save and load implicit models using the `.json` format for sharing and reuse.
 - **Mesh Export/Import**: Export results to `.obj` files or import external `.obj` files to create custom distance functions.
 - **Iso-surfacing**: Efficient iso-surface extraction from discretized scalar fields.
 - **CLI Interface**: Run saved models and show `.obj` files directly from the command line.
 - **Built-in Viewer** *(optional)*: Visualize mesh outputs quickly using the `viewer` feature built on top of `wgpu`.

 For a more in-depth explanation of the library, see the [docs](https://docs.rs/imlet)

## How to use

### Examples

To run a basic example via the terminal and show the result, run the following.

```cmd
cargo run --release --features viewer --example $example
```

where `$example` is any of the examples in the example dir, like `gyroid`, `interpolation` or `bunny`.

### CLI

To run a serialized model and show an output you can run the binary, for example

```cmd
cargo run --release --features viewer -- run-model assets/models/gyroid_model.json Output 0.5 --show
```

to run the `Output` node of the serialized model `gyroid_model.json` with a cell size of 0.5.

### Build a model

Add via cargo.

```
cargo add imlet
```
 
Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.

The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.

 ```rust
 use imlet::utils::io::write_obj_file;
 use imlet::types::geometry::{Vec3, BoundingBox, Sphere};
 use imlet::types::computation::{
     functions::Gyroid,
     operations::shape::BooleanIntersection,
 };
 use imlet::types::computation::model::ImplicitModel;

 // Define the model parameters
 let size = 10.0;
 let cell_size = 0.1;
 let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

 // Create an implicit model
 let mut model = ImplicitModel::with_bounds(model_space);

 // Add a sphere to the model
 let sphere = model
     .add_function(
         "Sphere",
         Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
     )
     .unwrap();

 // Add a gyroid function to the model
 let gyroid = model
     .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
     .unwrap();

 // Combine the sphere and gyroid using a Boolean intersection
 let intersection = model
     .add_operation_with_inputs(
         "Intersection",
         BooleanIntersection::new(),
         &[&sphere, &gyroid],
     )
     .unwrap();

 // Generate the iso-surface and save it to an OBJ file
 let mesh = model.generate_iso_surface(&intersection, cell_size).unwrap();
 write_obj_file(&mesh, "output.obj").unwrap();
 ```

## Roadmap

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