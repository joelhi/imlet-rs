use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{functions::Sphere, model::Model, operations::arithmetic::Subtract},
            Mesh, XYZ,
        },
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let num_pts = 10;
    let size = 10.0;

    // Function
    let mut model = Model::new();
    let sphere = model.add_function(Sphere::new(
        XYZ::new(size / 2.0, size / 2.0, size / 2.0),
        size * 0.45,
    ));

    let value = model.add_constant(-1.0);

    let offset = model.add_operation(Subtract::new(sphere, value));

    let mut field = model.evaluate(
        XYZ::origin(),
        num_pts,
        num_pts,
        num_pts,
        size / ((num_pts - 1) as f32),
        offset,
    );

    field.smooth(0.5, 1);

    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Normal));
}
