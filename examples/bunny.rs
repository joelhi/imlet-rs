use imlet::{
    types::computation::{
        functions::{Gyroid, MeshFile},
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    utils::{self, io::write_obj_file},
};

pub fn main() {
    utils::logging::init_info();

    let cell_size = 0.5;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();

    // Build model
    let mut model = ImplicitModel::with_bounds(mesh_file.bounds().unwrap().offset(cell_size));

    let mesh_tag = model.add_function("Mesh", mesh_file).unwrap();

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

    let mut mesh = model.generate_iso_surface(&output, 0.5).unwrap();

    mesh.compute_vertex_normals_par();

    write_obj_file(&mesh, "bunny_example").unwrap();
}
