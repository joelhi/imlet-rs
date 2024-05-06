use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{computation::{functions::Sphere, Model}, geometry::{BoundingBox, Mesh, Vec3f}},
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let size = 10.0;
    let cell_size = 0.25;
    let bounds = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

    // Function
    let mut model = Model::new();
    let sphere = model.add_function(Sphere::new(
        Vec3f::new(size / 2.0, size / 2.0, size / 2.0),
        size * 0.45,
    ));

    // Discretize
    let mut field = model.evaluate(bounds, cell_size, sphere);

    field.smooth(0.75, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Normal));
}
