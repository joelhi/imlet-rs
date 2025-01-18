### Implicit Modeling Lightweight Exploration Toolkit (Imlet)

![Build & Test](https://github.com/joelhi/implicit-rs/actions/workflows/rust.yml/badge.svg)

![Periodic Surface Example](media/examples.png)

 # Imlet

 `Imlet` (Implicit Modeling Lightweight Exploration Toolkit) is a lightweight and flexible engine for creating 3D geometries through implicit modeling, written in Rust. 
 It enables the construction of compound spatial functions that can be evaluated and polygonized to generate complex geometries.

 ## Overview

 `Imlet` provides tools for defining and combining distance functions, transforming these into geometric representations, and exporting the results. At its core, it offers a high-level interface for implicit modeling, including:

 ### Key Features

 - **Compositional Modeling**: Create complex functions by combining primitives (e.g., spheres, toruses) and operations (e.g., intersections, unions).
 - **Versatile Geometric Types**: Work with [planes](crate::types::geometry::Plane), [lines](crate::types::geometry::Line), [triangle meshes](crate::types::geometry::Mesh), and more.
 - **Custom Distance Functions**: Define distance functions mathematically or derive them from external triangle meshes.
 - **Model Serialization**: Save and load models using the `.json` format for easy sharing and reuse.
 - **Mesh Export/Import**: Export results to `.obj` files or import external `.obj` files for custom modeling.
 - **Marching Cubes Algorithm**: Efficient iso-surface extraction for polygonization.
 - **CLI Interface**: Run models and manage `.obj` files directly from the command line.
 - **Built-in Viewer** *(optional)*: Visualize results quickly using the `viewer` feature built on top of `wgpu`.

 ## Examples
 
 Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.

 The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.

 ```rust
 use imlet::utils::io::write_obj_file;

 use imlet::types::geometry::{Vec3, BoundingBox};
 use imlet::types::computation::{
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
- [x] Handle normals in obj import/export.
- [x] Improve normal interpolation to reduce leaking.
- [x] Simple mesh viewer with wgpu.
- [x] Some cli tools to run and show models.
- [ ] Finish docs.

### Future Enhancements (2025)
- [ ] Make model serialization compatible with external impls.
- [ ] Python binding or other scripting interface to build and compute models. 
- [ ] GPU computation of models for faster processing. (For example using [CubeCL](https://github.com/tracel-ai/cubecl))
- [ ] Develop a node editor for visual programming. (For example using [snarl](https://github.com/zakarumych/egui-snarl))

## License

This project is licensed under either of the following:

- [MIT License](LICENSE-MIT) 
- [Apache License, Version 2.0](LICENSE-APACHE)

Choose the one that best suits your needs.