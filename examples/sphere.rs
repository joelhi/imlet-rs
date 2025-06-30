use imlet::{
    types::{
        computation::{
            data::sampler::{DenseSampler, Sampler},
            model::ImplicitModel,
        },
        geometry::{BoundingBox, Sphere, Vec3},
    },
    utils,
};

pub fn main() {
    utils::logging::init_info();

    // Inputs
    let cell_size = 5.0;
    let size = 100.0;
    let offset = 5.0;
    let bounds = BoundingBox::new(
        Vec3::new(offset, offset, offset),
        Vec3::new(offset + size, offset + size, offset + size),
    );

    // Function
    let mut model = ImplicitModel::new();

    let _ = model
        .add_function(
            "Sphere",
            Sphere::new(
                Vec3::new(
                    offset + size / 2.0,
                    offset + size / 2.0,
                    offset + size / 2.0,
                ),
                size * 0.45,
            ),
        )
        .unwrap();

    let mut sampler = DenseSampler::builder()
        .with_bounds(bounds)
        .with_cell_size(cell_size)
        .build()
        .unwrap();

    sampler.sample_field(&model).expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    utils::io::write_obj_file(&mesh, "sphere_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        use imlet::viewer::{DisplaySettings, Material};

        imlet::viewer::show_mesh_with_settings(
            &mesh,
            Some(bounds),
            &DisplaySettings {
                show_bounds: true,
                show_mesh_edges: true,
                mesh_material: Material::Normal,
            },
        );
    }
}
