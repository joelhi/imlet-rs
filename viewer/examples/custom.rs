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

    // Read inputs
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "For this example, two arguments has to be specified. First a path to the obj file, followed by a cell size.");

    let cell_size = args[2]
        .parse()
        .expect("Failed to parse cell size from second argument");

    // Build model
    let mesh: Mesh<f64> = parse_obj_file(&args[1], false).unwrap();
    let model_space = mesh.bounds().offset(cell_size);

    let mut model = ImplicitModel::new();
    model
        .add_function("MeshSDF", MeshSDF::new(&mesh, 10, 12))
        .unwrap();

    Viewer::run(model, model_space, cell_size, "MeshSDF");
}
