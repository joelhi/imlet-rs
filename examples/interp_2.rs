use cgmath::InnerSpace;
use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                functions::{Gyroid, Neovius, OrthoBox, Sphere, YDomain, ZDomain},
                operations::{boolean::Intersection, interpolation::LinearInterpolation, shape::{Offset, Thickness}},
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
    let cell_size = 0.03;
    let model_space = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let bounds = model.add_function(Sphere::new(
        Vec3f::new(0.5*size, 0.5*size, 0.5*size),
        0.45*size,
    ));

    let shape1 = model.add_function(Gyroid::with_equal_spacing(1.5));
    let y_param = model.add_function(YDomain::remapped(0.5, 9.5));
    let thick = model.add_operation(Thickness::new(shape1, 1.0));
    let blend = model.add_operation(LinearInterpolation::new(thick, bounds, y_param));
    let intersect = model.add_operation(Intersection::new(bounds, blend));

    // Discretize
    let mut field = model.evaluate(model_space, cell_size, intersect);

    field.smooth(0.75, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Normal));
}
