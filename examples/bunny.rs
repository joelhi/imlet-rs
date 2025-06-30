use imlet::{
    types::computation::{
        data::sampler::{DenseSampler, Sampler},
        functions::{Gyroid, MeshFile},
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    utils::{self, io::write_obj_file},
};

pub fn main() {
    utils::logging::init_info();

    let cell_size = 0.5;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();
    let bounds = mesh_file.bounds().unwrap().offset(cell_size);
    // Build model
    let mut model = ImplicitModel::new();

    let mesh_tag = model.add_function("Mesh", mesh_file).unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(7.5, false))
        .unwrap();

    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(3.5), &[&gyroid_tag])
        .unwrap();

    let _ = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&mesh_tag, &offset_gyroid],
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

    write_obj_file(&mesh, "bunny_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh_with_settings(
            &mesh,
            Some(bounds),
            &imlet::viewer::DisplaySettings::new(),
        );
    }
}
