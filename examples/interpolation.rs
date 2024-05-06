use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                functions::{Gyroid, SchwarzP, Sphere, YDomain, ZDomain},
                operations::{
                    arithmetic::Subtract,
                    boolean::{Difference, Intersection, Union},
                    interpolation::LinearInterpolation,
                    shape::{Offset, Thickness},
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
    let cell_size = 0.03;
    let bounds = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let sphere = model.add_function(Sphere::new(
        Vec3f::new(size / 2.0, size / 2.0, size / 2.0),
        size * 0.45,
    ));
    let gyroid = model.add_function(Gyroid::with_equal_spacing(3.0));
    let schwarz = model.add_function(SchwarzP::with_equal_spacing(2.0));
    let y_param = model.add_function(YDomain::remapped(0.0, size));
    let blend = model.add_operation(LinearInterpolation::new(gyroid, schwarz, y_param));
    let thick_blend = model.add_operation(Thickness::new(blend, 0.75));
    let union = model.add_operation(Intersection::new(sphere, thick_blend));

    // Discretize
    let mut field = model.evaluate(bounds, cell_size, union);

    field.smooth(0.75, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Normal));
}
