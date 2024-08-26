use {
    imlet_engine::{
        types::{
            computation::{distance_functions::Sphere, implicit_model::ImplicitModel},
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
    let offset = 5.0;
    let cell_size = 0.25;
    let bounds = BoundingBox::new(
        Vec3::new(offset, offset, offset),
        Vec3::new(offset + size, offset + size, offset + size),
    );

    // Function
    let mut model = ImplicitModel::new();
    let _ = model.add_function(
        "Sphere",
        Sphere::new(
            Vec3::new(
                offset + size / 2.0,
                offset + size / 2.0,
                offset + size / 2.0,
            ),
            size * 0.45,
        ),
    );

    Viewer::run(model, bounds, cell_size, "Sphere");
}
