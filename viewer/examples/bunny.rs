use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, MeshSDF},
                operations::{boolean::Intersection, shape::Thickness},
                ImplicitModel,
            },
            geometry::Mesh,
        },
        utils::{self, io::parse_obj_file},
    },
    imlet_viewer::viewer::Viewer,
};

pub fn main() {
    utils::logging::init_info();

    let mesh: Mesh<f64> = parse_obj_file("assets/geometry/bunny.obj", false).unwrap();

    let cell_size = 0.5;
    let model_space = mesh.bounds().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::new();
    model.add_function("BunnyMesh", MeshSDF::new(&mesh, 10, 12));
    model.add_function("GyroidInfill", Gyroid::with_equal_spacing(7.5, true));
    model.add_operation_with_inputs("ThickGyroid", Thickness::new(5.0), &vec!["GyroidInfill"]);
    model.add_operation_with_inputs(
        "Output",
        Intersection::new(),
        &vec!["BunnyMesh", "ThickGyroid"],
    );

    Viewer::run(model, model_space, cell_size, "Output");
}
