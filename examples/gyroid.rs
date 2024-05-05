use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                functions::{Gyroid, Sphere},
                model::Model,
                operations::{
                    arithmetic::Subtract,
                    boolean::{Difference, Intersection},
                },
            },
            Mesh, XYZ,
        },
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let num_pts = 300;
    let size = 10.0;

    // Build model
    let mut model = Model::new();

    let sphere = model.add_function(Sphere::new(
        XYZ::new(size / 2.0, size / 2.0, size / 2.0),
        size * 0.45,
    ));
    let gyroid = model.add_function(Gyroid::with_equal_spacing(2.5));
    let constant = model.add_constant(-0.8);
    let offset_gyroid = model.add_operation(Subtract::new(gyroid, constant));
    let subtracted_gyroid = model.add_operation(Difference::new(gyroid, offset_gyroid));
    let union = model.add_operation(Intersection::new(sphere, subtracted_gyroid));

    // Discretize
    let mut field = model.evaluate(
        XYZ::origin(),
        num_pts,
        num_pts,
        num_pts,
        size / ((num_pts - 1) as f32),
        union,
    );

    field.smooth(0.75, 10);


    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Arctic));
}
