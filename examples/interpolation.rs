use imlet::{
    types::{
        computation::{
            data::sampler::{DenseSampler, Sampler},
            functions::MeshFile,
            model::{Data, ImplicitModel},
            operations::math::LinearInterpolation,
        },
        geometry::Sphere,
    },
    utils,
};

/// Example of shape interpolation and parameter use for a custom mesh and a mathematical primitive.
pub fn main() {
    utils::logging::init_info();

    let factor: f32 = 0.35;
    let cell_size = 0.5;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();
    let bounds = mesh_file.bounds().unwrap().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::with_bounds(bounds);

    let sphere_tag = model
        .add_function(
            "Sphere",
            Sphere::new(bounds.centroid(), bounds.dimensions().0 * 0.3),
        )
        .unwrap();

    let bunny_tag = model.add_function("Bunny", mesh_file).unwrap();

    let shape_interpolation = model
        .add_operation_with_inputs(
            "ShapeInterpolation",
            LinearInterpolation::new(),
            &[&bunny_tag, &sphere_tag],
        )
        .unwrap();

    // Update the interpolation factor
    model
        .get_component_mut(&shape_interpolation)
        .expect("Component should be present")
        .set_parameter("Factor", Data::Value(factor));

    let mut sampler = DenseSampler::builder()
        .with_bounds(bounds)
        .with_model(model)
        .build()
        .unwrap();

        sampler
        .sample_field(cell_size, &shape_interpolation)
        .expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    utils::io::write_obj_file(&mesh, "interpolation_example").unwrap();
    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(mesh.bounds()));
    }
}
