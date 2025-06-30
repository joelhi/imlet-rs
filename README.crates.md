# Imlet

 `Imlet` (Implicit Modeling Lightweight Exploration Toolkit) is a lightweight and flexible engine for creating 3D geometries through implicit modeling, written in Rust.
 It enables the construction of compound spatial functions that can be evaluated and polygonized to generate geometries.

 The project is still in the early phase of development, so expect breaking API changes as the library keeps developing.

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

### Build a model
 
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
     .add_operation_with_inputs(
         "Intersection",
         BooleanIntersection::new(),
         &[&sphere, &gyroid])
     .unwrap();

 // Sample a sparse field and generate an iso-surface.
 let config = SparseFieldConfig {
     internal_size: BlockSize::Size64,       // Internal node subdivision.
     leaf_size: BlockSize::Size4,            // Leaf node subdivision.
     sampling_mode: SamplingMode::CORNERS,    // Sampling logic for Leaf node exclusion.
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

 ```

## License

This project is licensed under either of the following:

- [MIT License](LICENSE-MIT) 
- [Apache License, Version 2.0](LICENSE-APACHE)

Choose the one that best suits your needs.