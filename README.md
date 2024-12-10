### Implicit Modeling Lightweight Exploration Toolkit (Imlet)

![Build & Test](https://github.com/joelhi/implicit-rs/actions/workflows/rust.yml/badge.svg)

![Periodic Surface Example](media/examples.png)

 # Imlet

 `Imlet` is a lightweight toolkit for implicit modeling and geometry generation, written in Rust. It provides tools for creating 3D geometries using an implicit logic in a modular way.

 ## Overview

 **Features in short:**
 * Implicit functions, such as various primitives and periodic surfaces; and operations, such as boolean methods.
 * Interface to build complex implicit models combining various functions with custom processing.
 * Set of tools to create and process geometric objects such as `Points`, `Lines` and `Meshes`.
 * Algorithms to evaluate and extract iso surfaces (as triangle meshes) from implicit models at arbitrary resolutions.
 * Import OBJ files and compute signed distance fields from arbitrary meshes.
 * Export OBJ of generated iso surfaces.
 * Viewer to show generated geometries with some basic post processing tools (WIP)

 The primary modules of the crate are [`types::geometry`] and [`types::computation`], which supply the tools needed to define geometric types and build implicit models.

 At the heart of Imlet is the [`types::computation::ImplicitModel`] struct, which serves as the foundation for creating and evaluating compound functions in 3d space.
 This struct exposes the main methods used to combine functions and operations into a computation graph, which can then be evaluated and used to generate isosurfaces.

 For detailed information on how these components work and interact, refer to the [`types`] module documentation.

 ## Examples

 Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.

 The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.

 ```rust
 use imlet_engine::utils::io::write_obj_file;

 use imlet_engine::types::geometry::{Vec3, BoundingBox};
 use imlet_engine::types::computation::{
     distance_functions::{Gyroid, Sphere},
     operations::shape::BooleanIntersection,
 };

 use imlet_engine::types::computation::ImplicitModel;

 fn main() {

     // Define some model parameters
     let size: f32 = 10.0;
     let cell_size = 0.1;
     let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

     // Create an empty model
     let mut model = ImplicitModel::new();

     // Adda a sphere distance function to the model.
     let sphere = model
         .add_function(
             "Sphere",
             Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
         )
         .unwrap();
     
     // Add a gyroid distance function to the model.
     let gyroid = model
         .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
         .unwrap();

     // Add a difference operation to the model, and feed it the output of the sphere and gyroid distance functions.
     let intersection = model
         .add_operation_with_inputs(
             "Intersection",
             BooleanIntersection::new(),
             &[&sphere, &gyroid],
         )
         .unwrap();

     // Generate an isosurface at the 0 distance.
     let mesh = model.generate_iso_surface(&intersection, &model_space, cell_size)
         .unwrap();

     // Write the mesh to an obj file.
     write_obj_file(&mesh, "output.obj").unwrap();
 }
 ```

## Roadmap

### Base Features
- [x] Update README with detailed examples and usage instructions.
- [x] Improve SDF (Signed Distance Function) computation, addressing issues with leaking and pseudonormals.
- [x] Clean up trait bounds across all structs, so they are only specified where needed.
- [x] More measured and intentional error handling using Result<> in various parts of the code, to make use easier and remove panics
- [x] Refactor so implicit geometries can be added directly to the models, based on the SignedDistance trait. Same for collections of geometries.
- [x] Set up basic viewer with bevy.
- [x] Add variable parameters to model components.
- [x] Implement better topological sort for computation graph / model connectivity.
- [ ] Enhance the viewer with interactive buttons and runtime model generation. (Oct - Dec)

### Future Enhancements (2025)
- [ ] Make model serialization compatible with external impls.
- [ ] Enable GPU-based computation for faster processing. (For example using [CubeCL](https://github.com/tracel-ai/cubecl))
- [ ] Develop a node editor for visual programming.

## License

This project is licensed under either of the following:

- [MIT License](LICENSE-MIT) 
- [Apache License, Version 2.0](LICENSE-APACHE)

Choose the one that best suits your needs.
