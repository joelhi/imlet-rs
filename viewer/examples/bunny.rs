use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, MeshSDF},
                operations::{boolean::Intersection, shape::Thickness},
                Model,
            },
            geometry::{BoundingBox, Mesh, OctreeNode, Vec3},
        }, utils::{self, io::{parse_obj_file, write_field_csv}}
    },
    imlet_viewer::viewer::Viewer,
};

pub fn main() {
    utils::logging::init_info();

    let mesh: Mesh<f64> = parse_obj_file("assets/geometry/bunny.obj").unwrap();

    let cell_size = 0.5;
    let model_space = mesh.get_bounds();

    // Build model
    let mut model = Model::new();
    let mut tree = OctreeNode::new(model_space, mesh.as_triangles());
    tree.build(10, 9);
    let bounds = model.add_function(MeshSDF::new(tree));
    //let shape = model.add_function(Gyroid::with_equal_spacing(10.0));
    //let thick_shape = model.add_operation(Thickness::new(shape, 1.75));
    //let _ = model.add_operation(Intersection::new(bounds, thick_shape));

    //let field = model.evaluate(&model_space, cell_size, None);

   //write_field_csv(&field, "bunny_field");

    Viewer::run(model, model_space, cell_size);
}
