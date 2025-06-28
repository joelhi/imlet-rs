use imlet::{
    types::computation::{
        data::{
            sampler::{Sampler, SparseSampler},
            BlockSize, SamplingMode, SparseFieldConfig,
        },
        functions::{Gyroid, MeshFile},
        model::ImplicitModel,
        operations::shape::{BooleanIntersection, Thickness},
    },
    utils::{self, io::write_obj_file},
};

pub fn main() {
    utils::logging::init_debug();

    let cell_size = 0.5;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();
    let bounds = mesh_file.bounds().unwrap().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::new();

    let mesh_tag = model.add_function("Mesh", mesh_file).unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(7.5, true))
        .unwrap();

    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(3.5), &[&gyroid_tag])
        .unwrap();

    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&mesh_tag, &offset_gyroid],
        )
        .unwrap();

    let config = SparseFieldConfig {
        internal_size: BlockSize::Size64,
        leaf_size: BlockSize::Size4,
        sampling_mode: SamplingMode::CENTRE,
    };

    let mut sampler = SparseSampler::builder()
        .with_bounds(bounds.offset(5.0))
        .with_model(model)
        .with_sparse_config(config)
        .build()
        .expect("Should be able to build the sampler.");

    sampler
        .sample_field(cell_size, &output)
        .expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    //write_field_csv(&field, "bunny_sparse").unwrap();
    write_obj_file(&mesh, "sparse_bunny").unwrap();
}
