use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, Sphere, ZDomain},
                operations::{
                    boolean::Intersection, interpolation::LinearInterpolation, shape::Thickness,
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

    model.add_function(
        "Sphere",
        Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
    ).unwrap();

    model.add_function("Gyroid", Gyroid::with_equal_spacing(1.0, true)).unwrap();
    model.add_operation_with_inputs("ThickGyroid", Thickness::new(1.0), &vec!["Gyroid"]).unwrap();
    model.add_operation_with_inputs("ThinGyroid", Thickness::new(0.15), &vec!["Gyroid"]).unwrap();
    model.add_function("ZParam", ZDomain::remapped(1.0, 9.0)).unwrap();
    model.add_operation_with_inputs(
        "Interpolation",
        LinearInterpolation::new(),
        &vec!["ThickGyroid", "ThinGyroid", "ZParam"],
    ).unwrap();
    model.add_operation_with_inputs(
        "Output",
        Intersection::new(),
        &vec!["Sphere", "Interpolation"],
    ).unwrap();

    Viewer::run(model, model_space, cell_size, "Output");
}
