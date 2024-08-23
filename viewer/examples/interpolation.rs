use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, Sphere, ZDomain},
                operations::{
                    boolean::Intersection, interpolation::LinearInterpolation, shape::Thickness,
                },
                Model,
            },
            geometry::{BoundingBox, Vec3},
        },
        utils,
    },
    imlet_viewer::viewer::Viewer,
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

    let shape = model.add_function(Gyroid::with_equal_spacing(1.5, true));
    let thick_shape = model.add_operation(Thickness::new(1.5), vec![shape]);
    let slender_shape = model.add_operation(Thickness::new(0.25), vec![shape]);
    let t = model.add_function(ZDomain::remapped(0.0, 10.0));
    let interpolation = model.add_operation(
        LinearInterpolation::new(),
        vec![thick_shape, slender_shape, t],
    );
    let _ = model.add_operation(Intersection::new(), vec![bounds, interpolation]);

    Viewer::run(model, model_space, cell_size);
}
