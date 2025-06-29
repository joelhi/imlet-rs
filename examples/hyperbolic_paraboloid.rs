use imlet::types::{
    computation::{
        data::sampler::{DenseSampler, Sampler},
        functions::{CoordinateValue, XYZValue},
        model::ImplicitModel,
        operations::math::{Divide, Multiply, Subtract},
    },
    geometry::{BoundingBox, Vec3},
};
use imlet::utils;

pub fn main() {
    utils::logging::init_info();

    let size: f32 = 5.0;
    let cell_size = 0.05;
    let bounds = BoundingBox::new(Vec3::new(-size, -size, -size), Vec3::new(size, size, size));

    let a: f32 = 2.0;
    let b: f32 = 3.0;

    // Build model
    let mut model = ImplicitModel::new();

    // Coordinate values
    let x_coord = model
        .add_function("x_coord", XYZValue::new(CoordinateValue::X))
        .unwrap();
    let y_coord = model
        .add_function("y_coord", XYZValue::new(CoordinateValue::Y))
        .unwrap();
    let z_coord = model
        .add_function("z_coord", XYZValue::new(CoordinateValue::Z))
        .unwrap();

    // Constants
    let a_sqrt = model.add_constant("a", a.powi(2)).unwrap();
    let b_sqrt = model.add_constant("b", b.powi(2)).unwrap();

    // X and Y terms
    let x_sqrt = model
        .add_operation_with_inputs("x_sqrt", Multiply::new(), &[&x_coord, &x_coord])
        .unwrap();
    let y_sqrt = model
        .add_operation_with_inputs("y_sqrt", Multiply::new(), &[&y_coord, &y_coord])
        .unwrap();

    let x_term = model
        .add_operation_with_inputs("x_term", Divide::new(), &[&x_sqrt, &a_sqrt])
        .unwrap();
    let y_term = model
        .add_operation_with_inputs("y_term", Divide::new(), &[&y_sqrt, &b_sqrt])
        .unwrap();

    // Final iso value
    let sub_1 = model
        .add_operation_with_inputs("sub_1", Subtract::new(), &[&y_term, &x_term])
        .unwrap();
    let output = model
        .add_operation_with_inputs("sub_2", Subtract::new(), &[&z_coord, &sub_1])
        .unwrap();

    let mut sampler = DenseSampler::builder()
        .with_bounds(bounds)
        .with_model(model)
        .build()
        .unwrap();

    sampler
        .sample_field(cell_size, &output)
        .expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    utils::io::write_obj_file(&mesh, "gyroid_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(bounds));
    }
}
