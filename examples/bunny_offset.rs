use imlet::{
    types::computation::{
        data::{
            sampler::{Sampler, SparseSampler},
            SparseFieldConfig,
        },
        functions::{Gyroid, MeshFile},
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    utils::{self, io::write_obj_file},
};

pub fn main() {
    utils::logging::init_info();

    let cell_size = 0.25;
    let offset_dist = 3.;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();
    let bounds = mesh_file.bounds().unwrap().offset(offset_dist + cell_size);

    // Build model
    let mut model = ImplicitModel::new();
    let _ = model.add_function("Mesh", mesh_file).unwrap();

    let mut sampler = SparseSampler::builder()
        .with_bounds(bounds)
        .with_min_val(offset_dist - 0.1)
        .with_max_val(offset_dist + 0.1)
        .with_tolerance(1E-3)
        .with_config(SparseFieldConfig::default().set_cell_size(cell_size))
        .build()
        .expect("Should be able to build the sampler.");

    sampler.sample_field(&model).expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(offset_dist)
        .expect("Extracting iso-surface should work.");

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
