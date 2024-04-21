use std::time::Instant;

use crate::{engine::{algorithms::marching_cubes::generate_iso_surface, types::{functions::*, DenseFieldF32, Mesh, XYZ}}, viewer::{material::Material, window::run}};

pub fn run_bitmask_gyroid(num_pts: usize, size: f32, length: f32, parallel: bool) {
    let sphere1 = Sphere {
        source: XYZ {
            x: size / 2.0,
            y: size / 2.0,
            z: size / 2.0,
        },
        radius: 0.4 * size,
    };

    let sphere2 = Sphere {
        source: XYZ {
            x: size / 2.0,
            y: size / 1.9,
            z: size / 2.0,
        },
        radius: 0.4 * size,
    };

    let offset_sphere = Offset {
        f: sphere2,
        distance: 3.0,
    };

    let gyroid = Gyroid {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let gyroid2 = Gyroid {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let offset = Offset {
        f: gyroid2,
        distance: -0.75,
    };

    let thick_gyroid = Difference {
        f: offset,
        g: gyroid,
    };

    let spheres = Difference {
        f: sphere1,
        g: offset_sphere,
    };

    let trimmed_sphere = ClippingPlane{
        function: spheres,
        direction: XYZ::y_axis(),
        distance: size/2.0,
    };

    let final_func = Intersection {
        f: thick_gyroid,
        g: trimmed_sphere,
    };

    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    grid.evaluate(&final_func, parallel);

    let triangles = generate_iso_surface(&grid, 0.0);

    let mesh = Mesh::from_triangles(&triangles);
    pollster::block_on(run(&mesh, Material::Normal));
}
