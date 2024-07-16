use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, MeshSDF},
                operations::{boolean::Intersection, shape::Thickness},
                Model,
            },
            geometry::Mesh,
        },
        utils::{self, io::parse_obj_file},
    },
    imlet_viewer::viewer::Viewer,
};

pub fn main() {
    utils::logging::init_info();

    let mesh: Mesh<f64> = parse_obj_file("assets/geometry/bunny.obj").unwrap();

    let cell_size = 0.25;
    let model_space = mesh.get_bounds().offset(1.0 * cell_size);

    // Build model
    let mut model = Model::new();
    let bounds = model.add_function(MeshSDF::new(&mesh, 10, 12));
    let shape = model.add_function(Gyroid::with_equal_spacing(15.0));
    let thick_shape = model.add_operation(Thickness::new(shape, 1.0));
    let _ = model.add_operation(Intersection::new(bounds, thick_shape));

    Viewer::run(model, model_space, cell_size);
}
