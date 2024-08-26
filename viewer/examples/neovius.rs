use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Neovius, Sphere},
                operations::{boolean::Intersection, shape::Thickness},
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
    let size = 10.0;
    let cell_size = 0.05;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = ImplicitModel::new();

    model.add_function(
        "Sphere",
        Sphere::new(Vec3::new(0.5 * size, 0.5 * size, 0.5 * size), 0.45 * size),
    );
    model.add_function("Neovius", Neovius::with_equal_spacing(1.5, true));
    model.add_operation_with_inputs("ThickNeovius", Thickness::new(0.1), &vec!["Neovius"]);
    model.add_operation_with_inputs(
        "Output",
        Intersection::new(),
        &vec!["Sphere", "ThickNeovius"],
    );

    Viewer::run(model, model_space, cell_size, "Output");
}
