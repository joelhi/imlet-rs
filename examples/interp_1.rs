use implicit::{
    display::viewer, engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                distance_functions::{Gyroid, OrthoBox, SchwarzP, YDomain},
                operations::{
                    boolean::Intersection,
                    interpolation::LinearInterpolation,
                    shape::Thickness,
                },
                Model,
            },
            geometry::{BoundingBox, Mesh, Vec3f},
        },
        utils,
    }, display::material::Material
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let size = 10.0;
    let cell_size = 0.03;
    let model_space = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let bounds = model.add_function(OrthoBox::from_size(
        Vec3f::new(2.0*cell_size, 2.0*cell_size, 2.0*cell_size),
        size-4.0*cell_size,
    ));
    let shape1 = model.add_function(Gyroid::with_equal_spacing(1.5));
    let shape2 = model.add_function(SchwarzP::with_equal_spacing(3.0));
    let y_param = model.add_function(YDomain::remapped(0.0, size));
    let blend = model.add_operation(LinearInterpolation::new(shape1, shape2, y_param));
    let thick_blend = model.add_operation(Thickness::new(blend, 0.75));
    let intersect = model.add_operation(Intersection::new(bounds, thick_blend));

    // Discretize
    let mut field = model.evaluate(model_space, cell_size, intersect);

    field.smooth(0.75, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    viewer::run_viewer(&mesh, Material::Arctic);
}
