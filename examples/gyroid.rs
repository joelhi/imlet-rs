use imlet::types::{
    computation::{
        functions::Gyroid,
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    geometry::{BoundingBox, Sphere, Vec3},
};

pub fn main() {
    //utils::logging::init_info();

    let size: f32 = 100.0;
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
        .add_function("Gyroid", Gyroid::with_equal_spacing(15., false))
        .unwrap();

    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(10.), &[&gyroid_tag])
        .unwrap();

    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&sphere_tag, &offset_gyroid],
        )
        .unwrap();

    let mesh = model
        .generate_iso_surface(&output, &model_space, 0.5)
        .unwrap();

    write_obj_file(&mesh, "output").unwrap();
}
