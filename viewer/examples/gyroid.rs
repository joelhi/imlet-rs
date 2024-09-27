use {
    imlet_engine::{
        types::{
            computation::{
                functions::Gyroid,
                operations::shape::{BooleanIntersection, Thickness},
                ImplicitModel,
            },
            geometry::{BoundingBox, Sphere, Vec3},
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

    let sphere_tag = model
        .add_function(
            "Sphere",
            Sphere::at_coord(0.5 * size, 0.5 * size, 0.5 * size, 0.45 * size),
        )
        .unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(2.5, true))
        .unwrap();
    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(1.5), &[&gyroid_tag])
        .unwrap();
    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&sphere_tag, &offset_gyroid],
        )
        .unwrap();

    Viewer::run(model, model_space, cell_size, &output);
}
