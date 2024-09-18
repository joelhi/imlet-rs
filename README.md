### Implicit Modeling Lightweight Exploration Toolkit (Imlet)

![Build & Test](https://github.com/joelhi/implicit-rs/actions/workflows/rust.yml/badge.svg)

![Periodic Surface Example](media/examples.png)

## Overview

**Imlet** is a Rust library for implicit modeling and geometry generation, It provides tools for creating 3D models defined by mathematical functions, where models are represented as computation graphs. The models enable the combination of various distance fields (SDFs) with custom operations to generate complex shapes and functions.

The library uses the marching cubes algorithm, following the approach by [Paul Bourke](https://paulbourke.net/geometry/polygonise/), to convert these implicit representations into polygonal meshes.

## Features

- **Implicit Functions**: Define geometries using distance functions, from equations or triangle meshes.
- **Modular Design**: Easily combine and manipulate implicit functions by building computation graphs.
- **Marching Cubes Algorithm**: Convert implicit functions into polygonal meshes.

## Example Usage

Here’s a simple example demonstrating how to use Imlet to create a model combining a sphere and a gyroid:

```rust

fn main() {
    let size: f32 = 10.0;
    let cell_size = 0.05;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = ImplicitModel::new();

    model
        .add_function(
            "Sphere",
            Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
        )
        .unwrap();

    model
        .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
        .unwrap();

    model
        .add_operation_with_inputs(
            "Output",
            Intersection::new(),
            &vec!["Sphere", "Gyroid"],
        )
        .unwrap();

    let mesh = model.generate_iso_surface("Output", &model_space, cell_size);

    write_obj_file(&mesh, "output.obj").unwrap();
}
```

## Roadmap

### Base Features
- [x] Update README with detailed examples and usage instructions.
- [x] Improve SDF (Signed Distance Function) computation, addressing issues with leaking and pseudonormals.
- [ ] More measured and intentional error handling using Result<> in various parts of the code, to make use easier and remove panics
- [ ] Clean up trait bounds across all structs, so they are only specified where needed.
- [ ] Implement serialization for models, potentially using sealed traits (?).
- [ ] Integrate [Rhai](https://rhai.rs/), or other scripting language for Rust, for enhanced customization.
- [ ] Enhance the viewer with interactive buttons and runtime script loading (Use bevy?).

### Future Enhancements
- [ ] Enable GPU-based computation for faster processing. (For example using [CubeCL](https://github.com/tracel-ai/cubecl))
- [ ] Develop a node editor for visual programming.

## License

This project is licensed under either of the following:

- [MIT License](LICENSE-MIT) 
- [Apache License, Version 2.0](LICENSE-APACHE)

Choose the one that best suits your needs.
