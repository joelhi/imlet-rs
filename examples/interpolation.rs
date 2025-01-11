use imlet::{
    types::{
        computation::{
            functions::{Gyroid, SchwarzP},
            model::ImplicitModel,
            operations::{
                math::LinearInterpolation,
                shape::{BooleanIntersection, Thickness},
            },
        },
        geometry::{BoundingBox, Sphere, Vec3},
    },
    utils,
};

pub fn main() {
    utils::logging::init_info();

    let size: f64 = 100.0;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = ImplicitModel::with_bounds(model_space);

    let sphere_tag = model
        .add_function(
            "Sphere",
            Sphere::at_coord(0.5 * size, 0.5 * size, 0.5 * size, 0.45 * size),
        )
        .unwrap();

    let box_tag = model
        .add_function(
            "Box",
            BoundingBox::new(
                Vec3::new(0.1 * size, 0.1 * size, 0.1 * size),
                Vec3::new(0.9 * size, 0.9 * size, 0.9 * size),
            ),
        )
        .unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(15., false))
        .unwrap();

    let schwarz_tag = model
        .add_function("Schwarz", SchwarzP::with_equal_spacing(10., false))
        .unwrap();

    let shape_interpolation = model
        .add_operation_with_inputs(
            "ShapeInterpolation",
            LinearInterpolation::new(),
            &[&box_tag, &sphere_tag],
        )
        .unwrap();

    let infill_interpolation = model
        .add_operation_with_inputs(
            "InfillInterpolation",
            LinearInterpolation::new(),
            &[&gyroid_tag, &schwarz_tag],
        )
        .unwrap();

    let offset_infill = model
        .add_operation_with_inputs("OffsetInfill", Thickness::new(5.), &[&infill_interpolation])
        .unwrap();

    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&shape_interpolation, &offset_infill],
        )
        .unwrap();

    let mesh = model.generate_iso_surface(&output, 0.5).unwrap();

    utils::io::write_obj_file(&mesh, "interpolation_example").unwrap();
}
