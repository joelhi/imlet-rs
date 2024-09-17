use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, Sphere},
                operations::{boolean::Intersection, shape::Thickness},
                ImplicitModel,
            },
            geometry::{BoundingBox, Vec3},
        },
        utils::{self},
    },
    imlet_viewer::viewer::Viewer,
};

pub fn main() {
    utils::logging::init_info();

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
        .add_operation_with_inputs("ThickGyroid", Thickness::new(1.5), &vec!["Gyroid"])
        .unwrap();
    model
        .add_operation_with_inputs(
            "Output",
            Intersection::new(),
            &vec!["Sphere", "ThickGyroid"],
        )
        .unwrap();

    Viewer::run(model, model_space, cell_size, "Output");
}
