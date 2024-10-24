use imlet::{
    types::{
        computation::{
            functions::{Gyroid, SchwarzP},
            operations::{
                math::LinearInterpolation,
                shape::{BooleanIntersection, Thickness},
            },
            ImplicitModel,
        },
        geometry::{BoundingBox, Sphere, Vec3},
    },
    utils,
};

pub fn main() {
    utils::logging::init_info();

    let size: f32 = 100.0;
    let model_space = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

    // Build model
    let mut model = ImplicitModel::new();

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

    let infill_variable = model.add_constant("InfillFactor", 0.5).unwrap();

    let shape_variable = model.add_constant("ShapeFactor", 0.5).unwrap();

    let shape_interpolation = model
        .add_operation_with_inputs(
            "ShapeInterpolation",
            LinearInterpolation::new(),
            &[&box_tag, &sphere_tag, &shape_variable],
        )
        .unwrap();

    let infill_interpolation = model
        .add_operation_with_inputs(
            "InfillInterpolation",
            LinearInterpolation::new(),
            &[&gyroid_tag, &schwarz_tag, &infill_variable],
        )
        .unwrap();

    let _ = model
        .add_operation_with_inputs("OffsetInfill", Thickness::new(5.), &[&infill_interpolation])
        .unwrap();

    let _ = model
        .add_operation_with_inputs(
            "Union",
            BooleanIntersection::new(),
            &[&infill_interpolation, &shape_interpolation],
        )
        .unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::run_explorer(model, &model_space);
    }
    #[cfg(not(feature = "viewer"))]
    {
        let _ = model
            .generate_iso_surface(&output, &model_space, cell_size)
            .unwrap();

        println!("Enable the viewer feature by using (--features viewer) to show the result");
    }
}
