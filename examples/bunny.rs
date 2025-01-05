use imlet::{
    types::computation::{
        functions::{CustomMesh, Gyroid},
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    utils::io::{parse_obj_file, write_obj_file},
};

pub fn main() {
    let mesh = parse_obj_file("assets/geometry/bunny.obj", false).unwrap();

    let cell_size = 0.5;
    let model_space = mesh.bounds().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::new();

    let mesh_tag = model
        .add_function("Mesh", CustomMesh::build(&mesh))
        .unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(5., false))
        .unwrap();

    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(2.), &[&gyroid_tag])
        .unwrap();

    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&mesh_tag, &offset_gyroid],
        )
        .unwrap();

    let mesh = model
        .generate_iso_surface(&output, &model_space, 0.5)
        .unwrap();

    write_obj_file(&mesh, "bunny_example").unwrap();
}
