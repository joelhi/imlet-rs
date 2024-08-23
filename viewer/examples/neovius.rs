use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Neovius, Sphere},
                operations::{boolean::Intersection, shape::Thickness},
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
    let size = 10.0;
    let cell_size = 0.025;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let bounds = model.add_function(Sphere::new(
        Vec3::new(0.5 * size, 0.5 * size, 0.5 * size),
        0.45 * size,
    ));
    let shape = model.add_function(Neovius::with_equal_spacing(2.0));
    let thick_shape = model.add_operation(Thickness::new(2.0), vec![shape]);
    let _ = model.add_operation(Intersection::new(), vec![bounds, thick_shape]);

    Viewer::run(model, model_space, cell_size);
}
