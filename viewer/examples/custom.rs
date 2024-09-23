use {
    imlet_engine::{
        types::{
            computation::{distance_functions::MeshSDF, ImplicitModel},
            geometry::Mesh,
        },
        utils::{self, io::parse_obj_file},
    },
    imlet_viewer::viewer::Viewer,
    std::env,
};

pub fn main() {
    utils::logging::init_info();

    // Parse inputs
    let (file_path, cell_size) = parse_input_args();

    // Build model
    let mesh: Mesh<f64> = parse_obj_file(&file_path, false).unwrap();
    let model_space = mesh.bounds().offset(2.0 * cell_size);

    let mut model = ImplicitModel::new();
    let mesh_sdf_tag = model
        .add_function("MeshSDF", MeshSDF::build(&mesh, 10, 12))
        .unwrap();

    Viewer::run(model, model_space, cell_size, &mesh_sdf_tag);
}

fn parse_input_args() -> (String, f64) {
    // Read inputs
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "For this example, two arguments has to be specified. First a path to the obj file, followed by a cell size.");

    let cell_size = args[2]
        .parse()
        .expect("Failed to parse cell size from second argument");

    (args[1].clone(), cell_size)
}
