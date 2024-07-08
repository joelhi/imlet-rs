use {
    imlet_engine::{
        types::{
            computation::{
                distance_functions::{Gyroid, MeshSDF},
                operations::{boolean::Intersection, shape::Thickness},
                Model,
            },
            geometry::{BoundingBox, Mesh, OctreeNode, Vec3},
        }, utils::{self, io::parse_obj_file}
    },
    imlet_viewer::viewer::Viewer,
};

pub fn main() {
    utils::logging::init_info();

    let mesh: Mesh<f32> = parse_obj_file("assets/geometry/bunny.obj").unwrap();

    let cell_size = 0.30;
    let model_space = BoundingBox::new(Vec3::origin(), mesh.get_bounds().max);

    // Build model
    let mut model = Model::new();
    let mut tree = OctreeNode::new(model_space, mesh.as_triangles());
    tree.build(10, 15);
    let bounds = model.add_function(MeshSDF::new(tree));
    let shape = model.add_function(Gyroid::with_equal_spacing(15.0));
    let thick_shape = model.add_operation(Thickness::new(shape, 2.0));
    let _ = model.add_operation(Intersection::new(bounds, thick_shape));

    Viewer::run(model, model_space, cell_size);
}
