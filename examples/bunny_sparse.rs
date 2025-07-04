use imlet::{
    types::computation::{
        data::{
            sampler::{Sampler, SparseSampler},
            SamplingMode, SparseFieldConfig,
        },
        functions::{Gyroid, MeshFile},
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    utils::{self, io::write_obj_file},
};

pub fn main() {
    utils::logging::init_debug();

    let cell_size = 0.25;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();
    let bounds = mesh_file.bounds().unwrap().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::new();

    let mesh_tag = model.add_function("Mesh", mesh_file).unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(15.0, false))
        .unwrap();

    let offset_gyroid = model
        .add_operation("OffsetGyroid", Thickness::new(7.5), Some(&[&gyroid_tag]))
        .unwrap();

    let _ = model
        .add_operation(
            "Output",
            BooleanIntersection::new(),
            Some(&[&mesh_tag, &offset_gyroid]),
        )
        .unwrap();

    let mut sampler = SparseSampler::builder()
        .with_bounds(bounds)
        .with_tolerance(1E-3)
        .with_config(
            SparseFieldConfig::default()
                .set_cell_size(cell_size)
                .set_leaf_size(imlet::types::computation::data::BlockSize::Size8)
                .set_sampling_mode(SamplingMode::CORNERS),
        )
        .build()
        .expect("Should be able to build the sampler.");

    sampler.sample_field(&model).expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    //write_field_csv(&field, "bunny_sparse").unwrap();
    write_obj_file(&mesh, "sparse_bunny").unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh_with_settings(
            &mesh,
            Some(bounds),
            &imlet::viewer::DisplaySettings::new(),
        );
    }
}
