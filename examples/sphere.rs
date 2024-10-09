use {
    imlet::viewer::show_mesh,
    imlet::{
        types::{
            computation::ImplicitModel,
            geometry::{BoundingBox, Sphere, Vec3},
        },
        utils,
    },
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

    let sphere_tag = model
        .add_function(
            "Sphere",
            Sphere::new(
                Vec3::new(
                    offset + size / 2.0,
                    offset + size / 2.0,
                    offset + size / 2.0,
                ),
                size * 0.45,
            ),
        )
        .unwrap();

    // Generate mesh
    let mesh = model
        .generate_iso_surface("Sphere", &bounds, cell_size)
        .unwrap();

    show_mesh(&mesh);
}
