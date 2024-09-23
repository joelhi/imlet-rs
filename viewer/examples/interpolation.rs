use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, Sphere, ZCoord},
                operations::{
                    math::LinearInterpolation, shape::BooleanIntersection, shape::Thickness,
                },
                ImplicitModel,
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
    let mut model = ImplicitModel::new();

    let sphere_tag = model
        .add_function(
            "Sphere",
            Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
        )
        .unwrap();

    let original_gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(1.0, true))
        .unwrap();

    let offset_gyroid_thick_tag = model
        .add_operation_with_inputs("ThickGyroid", Thickness::new(1.0), &[&original_gyroid_tag])
        .unwrap();

    let offset_gyroid_thin_tag = model
        .add_operation_with_inputs("ThinGyroid", Thickness::new(0.15), &[&original_gyroid_tag])
        .unwrap();

    let z_param_tag = model
        .add_function("ZParam", ZCoord::remapped(1.0, 9.0))
        .unwrap();

    let interpolation_tag = model
        .add_operation_with_inputs(
            "Interpolation",
            LinearInterpolation::new(),
            &[
                &offset_gyroid_thick_tag,
                &offset_gyroid_thin_tag,
                &z_param_tag,
            ],
        )
        .unwrap();
    let output_tag = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&sphere_tag, &interpolation_tag],
        )
        .unwrap();

    Viewer::run(model, model_space, cell_size, &output_tag);
}
