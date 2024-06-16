use {
    imlet_engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                distance_functions::{Gyroid, Sphere},
                operations::{boolean::Intersection, shape::Thickness},
                Model,
            },
            geometry::{BoundingBox, Mesh, Vec3},
        },
        utils,
    },
    imlet_viewer::{material::Material, viewer::{self, ViewerSettings}},
};

pub fn main() {
    utils::logging::init_info();

    // Inputs
    let size: f32 = 10.0;
    let cell_size = 0.05;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let bounds = model.add_function(Sphere::new(
        Vec3::new(0.5 * size, 0.5 * size, 0.5 * size),
        0.45 * size,
    ));

    let shape = model.add_function(Gyroid::with_equal_spacing(2.5));
    let thick_shape = model.add_operation(Thickness::new(shape, 1.75));
    let intersection = model.add_operation(Intersection::new(bounds, thick_shape));

    let settings = ViewerSettings{
        mesh_material: Material::Normal,
        show_bounds: true,
        show_edges: true,
    };

    let mut viewer = viewer::Viewer::with_settings(settings);

    viewer.add_model(model);
    viewer.create_mesh_from_model(model_space, cell_size, 5, 0.75);

    viewer.run();
}
