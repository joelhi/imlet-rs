### Implicit Modeling Lightweight Exploration Toolkit (Imlet)

![Build & Test](https://github.com/joelhi/implicit-rs/actions/workflows/rust.yml/badge.svg)

![Periodic Surface Example](media/examples.png)

`Imlet` is a lightweight toolkit for implicit modeling and geometry generation, written in Rust. It provides tools for creating 3D geometries, offering a set of data structures and algorithms that can be used to model in 3d using an implicit logic in a modular way.

 ## Overview
 
 ### Features
 * Implicit functions, such as various primitives and periodic surfaces; and operations, such as boolean methods.
 * Interface to build complex implicit models combining various functions with custom processing.
 * Set of tools to create and process geometric objects such as `Points`, `Lines` and `Meshes`.
 * Import OBJ files and compute signed distance fields from arbitrary meshes.
 * Algorithms to evaluate and extract iso surfaces (as triangle meshes) from implcict models at arbitrary resolutions.
 * Export OBJ of generated iso surfaces.
 * Viewer to show generated geometries with some basic post processing tools (WIP)
 
 The primary modules of the crate are [`types::geometry`] and [`types::computation`], which supply the tools needed to define geometric types and build implicit models.

At the heart of Imlet is the [`ImplicitModel`] struct, which serves as the foundation for creating and evaluating compound functions in 3d space.

This struct exposes the main methods used to combine functions and operations into a computation graph, which can then be evaluated and used to generate iso surfaces.

For detailed information on how these components work and interact, refer to the [`types`] module documentation.

 ## Examples

 ### The Very Basic
 
 The simplest possible computation would be to define two constants, and add them together.
 
 In this example the value is not depending on the x,y,z coordinates, so we just evaluate it once at the origin.

 ```rust
 fn main() {

     let mut model = ImplicitModel::new();

    // Add a constant with a value 1 to the model.
    model.add_constant("FirstValue", 1.0).unwrap();

    // Add another constant with a value 1 to the model.
    model.add_constant("SecondValue", 1.0).unwrap();

    // Add an addition operation that reads the two constants and adds them together.
    model
        .add_operation_with_inputs("Sum", Add::new(), &vec!["FirstValue", "SecondValue"])
        .unwrap();

    // Evaluate the model reading the output of the Sum operation.
    let value = model.evaluate_at("Sum", 0.0, 0.0, 0.0);
    println!("The value is {}", value)
}

 ```
 
 This should print the following to the terminal.
 ```shell
 The value is 2
 ```
 
 ### An Actual Geometry
 
 Below is an example of how to use Imlet to create a 3D model by combining a sphere and a gyroid using an intersection operation.
 
 The model is then evaluated over a 3D space and saved as a mesh in an OBJ file.

 ```rust
 fn main() {
 
     // Define some model parameters
     let size: f32 = 10.0;
     let cell_size = 0.05;
     let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

     // Create an empty model
     let mut model = ImplicitModel::new();

     // Adda a sphere distance function to the model.
     model
         .add_function(
             "Sphere",
             Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
         )
         .unwrap();
     
     // Add a gyroid distance function to the model.
     model
         .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
         .unwrap();

     // Add a difference operation to the model, and feed it the output of the sphere and gyroid distance functions.
     model
         .add_operation_with_inputs(
             "Output",
             Intersection::new(),
             &vec!["Sphere", "Gyroid"],
         )
         .unwrap();

     // Generate an isosurface at the 0 distance.
     let mesh = model.generate_iso_surface("Output", &model_space, cell_size);

     // Write the mesh to an obj file.
     write_obj_file(&mesh, "output.obj").unwrap();
 }
 ```


## Roadmap

### Base Features
- [x] Update README with detailed examples and usage instructions.
- [x] Improve SDF (Signed Distance Function) computation, addressing issues with leaking and pseudonormals.
- [ ] Clean up trait bounds across all structs, so they are only specified where needed.
- [ ] Refactor so implicit geometries can be added directly to the models, based on the SignedDistance trait. Same for collections of geometries.
- [ ] More measured and intentional error handling using Result<> in various parts of the code, to make use easier and remove panics
- [ ] Enhance the viewer with interactive buttons and runtime script loading (Use bevy?).
- [ ] Implement serialization for models, potentially using sealed traits (?).
- [ ] Integrate [Rhai](https://rhai.rs/), or other scripting language for Rust, for enhanced customization.

### Future Enhancements
- [ ] Enable GPU-based computation for faster processing. (For example using [CubeCL](https://github.com/tracel-ai/cubecl))
- [ ] Develop a node editor for visual programming.

## License

This project is licensed under either of the following:

- [MIT License](LICENSE-MIT) 
- [Apache License, Version 2.0](LICENSE-APACHE)

Choose the one that best suits your needs.
