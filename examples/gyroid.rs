use imlet::types::{
    computation::{
        data::sampler::{DenseSampler, Sampler},
        functions::Gyroid,
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    geometry::{BoundingBox, Sphere, Vec3},
};
use imlet::utils;

pub fn main() {
    utils::logging::init_info();

    let size: f32 = 100.0;
    let cell_size = 0.5;
    let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = ImplicitModel::new();

    let sphere_tag = model
        .add_function(
            "Sphere",
            Sphere::at_coord(0.5 * size, 0.5 * size, 0.5 * size, 0.45 * size),
        )
        .unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(15., false))
        .unwrap();

    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(10.), &[&gyroid_tag])
        .unwrap();

    let _ = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&sphere_tag, &offset_gyroid],
        )
        .unwrap();

    let mut sampler = DenseSampler::builder()
        .with_bounds(bounds)
        .with_cell_size(cell_size)
        .with_smoothing_iter(5)
        .with_smoothing_factor(0.75)
        .build()
        .unwrap();

    sampler.sample_field(&model).expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");
    utils::io::write_obj_file(&mesh, "gyroid_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(bounds));
    }
}
