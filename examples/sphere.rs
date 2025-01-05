use imlet::{
    types::{
        computation::model::ImplicitModel,
        geometry::{BoundingBox, Sphere, Vec3},
    },
    utils,
};

pub fn main() {
    utils::logging::init_info();

    // Inputs
    let size = 100.0;
    let offset = 5.0;
    let model_space = BoundingBox::new(
        Vec3::new(offset, offset, offset),
        Vec3::new(offset + size, offset + size, offset + size),
    );

    // Function
    let mut model = ImplicitModel::new();

    let sphere_node = model
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

    let mesh = model
        .generate_iso_surface(&sphere_node, &model_space, 0.5)
        .unwrap();

    utils::io::write_obj_file(&mesh, "sphere_example").unwrap();
}
