use {
    imlet_engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                distance_functions::{Gyroid, Sphere, ZDomain},
                operations::{boolean::Intersection, interpolation::LinearInterpolation, shape::Thickness},
                Model,
            },
            geometry::{BoundingBox, Mesh, Vec3},
        },
        utils,
    },
    imlet_viewer::{material::Material, viewer},
};

pub fn main() {
    utils::logging::init_info();

    // Inputs
    let size: f32 = 10.0;
    let cell_size = 0.05;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let bounds = model.add_function(Sphere::new(
        Vec3::new(0.5 * size, 0.5 * size, 0.5 * size),
        0.45 * size,
    ));

    let shape = model.add_function(Gyroid::with_equal_spacing(2.5));
    let thick_shape = model.add_operation(Thickness::new(shape, 2.5));
    let slender_shape = model.add_operation(Thickness::new(shape, 1.0));
    let t = model.add_function(ZDomain::remapped(0.0, 10.0));
    let interpolation = model.add_operation(LinearInterpolation::new(thick_shape, slender_shape, t));
    let intersection = model.add_operation(Intersection::new(bounds, interpolation));

    // Discretize
    let mut field = model.evaluate(model_space, cell_size, intersection);
    field.smooth(0.5, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    viewer::run_viewer(&mesh, Material::Normal);
}
