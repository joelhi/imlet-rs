use imlet::utils;
use imlet::{
    types::{
        computation::{
            functions::Gyroid,
            model::ImplicitModel,
            operations::shape::{BooleanIntersection, Thickness},
        },
        geometry::{BoundingBox, Sphere, Vec3},
    },
    viewer,
};

pub fn main() {
    utils::logging::init_info();

    let size: f32 = 100.0;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = ImplicitModel::with_bounds(model_space);

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

    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&sphere_tag, &offset_gyroid],
        )
        .unwrap();

    let mut mesh = model.generate_iso_surface(&output, 0.5).unwrap();

    utils::io::write_obj_file(&mesh, "gyroid_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        mesh.compute_vertex_normals_par();
        viewer::show_mesh(&mesh, mesh.bounds());
    }
}
