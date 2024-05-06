

use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                functions::{Gyroid, Neovius, OrthoBox, Sphere},
                operations::{
                    boolean::{Difference, Intersection}, shape::{Offset, Thickness}
                },
                Model,
            },
            geometry::{BoundingBox, Mesh, Vec3f},
        },
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let size = 10.0;
    let cell_size = 0.025;
    let model_space = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let bounds = model.add_function(Sphere::new(
        Vec3f::new(0.5*size, 0.5*size, 0.5*size),
        0.45*size,
    ));
    let shape = model.add_function(Neovius::with_equal_spacing(2.0));
    let thick_shape = model.add_operation(Thickness::new(shape, 2.0));
    let union = model.add_operation(Intersection::new(bounds, thick_shape));

    // Discretize
    let mut field = model.evaluate(model_space, cell_size, union);
    field.smooth(0.75, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Normal));
}
