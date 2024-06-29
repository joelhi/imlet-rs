use {
    imlet_engine::{
        types::{
            computation::{distance_functions::Sphere, Model},
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
    let cell_size = 0.25;
    let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Function
    let mut model = Model::new();
    let _ = model.add_function(Sphere::new(
        Vec3::new(size / 2.0, size / 2.0, size / 2.0),
        size * 0.45,
    ));

    Viewer::run(model, bounds, cell_size);
}
