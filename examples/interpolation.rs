use imlet::{
    types::computation::{
        data::sampler::{DenseSampler, Sampler},
        functions::{Gyroid, MeshFile, XYZValue},
        model::ImplicitModel,
        operations::{
            math::{Remap, VariableLinearInterpolation},
            shape::{BooleanIntersection, Thickness},
        },
    },
    utils,
};

/// Example of shape interpolation and parameter use for a custom mesh and a mathematical primitive.
pub fn main() {
    utils::logging::init_info();

    let cell_size = 0.5;
    let mesh_file = MeshFile::from_path("assets/geometry/bunny.obj").unwrap();
    let bounds = mesh_file.bounds().unwrap().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::new();

    let mesh_tag = model.add_function("Mesh", mesh_file).unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(3.0, true))
        .unwrap();

    let offset_gyroid = model
        .add_operation(
            "OffsetGyroid",
            Thickness::new(1.5),
            Some(&[&gyroid_tag]))
        .unwrap();

    let union_tag = model
        .add_operation(
            "Union",
            BooleanIntersection::new(),
            Some(&[&mesh_tag, &offset_gyroid]),
        )
        .unwrap();

    let z_coord_tag = model.add_function("z_coord", XYZValue::z()).unwrap();

    let interp_factor_tag = model
        .add_operation(
            "factor",
            Remap::with_source_domain(bounds.min.z, bounds.max.z),
            Some(&[&z_coord_tag]),
        )
        .unwrap();

    let _ = model
        .add_operation(
            "ShapeInterpolation",
            VariableLinearInterpolation::new(),
            Some(&[&mesh_tag, &union_tag, &interp_factor_tag]),
        )
        .unwrap();

    let mut sampler = DenseSampler::builder()
        .with_bounds(bounds)
        .with_cell_size(cell_size)
        .with_smoothing_factor(0.75)
        .with_smoothing_iter(3)
        .build()
        .unwrap();

    sampler.sample_field(&model).expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    utils::io::write_obj_file(&mesh, "interpolation_example").unwrap();
    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(bounds));
    }
}
