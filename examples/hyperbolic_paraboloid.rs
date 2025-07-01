use imlet::types::{
    computation::{
        data::sampler::{DenseSampler, Sampler},
        functions::XYZValue,
        model::ImplicitModel,
        operations::math::{Divide, Multiply, Subtract},
        traits::{ImplicitComponent, ImplicitFunction},
        ModelError,
    },
    geometry::{BoundingBox, Vec3},
};
use imlet::utils;
use num_traits::Float;
use serde::Serialize;

/// Create a new implicit model representing a hyperbolic paraboloid surface.
fn create_hyperbolic_paraboliod(a: f32, b: f32) -> Result<ImplicitModel<f32>, ModelError> {
    let mut model = ImplicitModel::new();

    // Coordinate values
    let x_coord = model.add_function("x_coord", XYZValue::x()).unwrap();
    let y_coord = model.add_function("y_coord", XYZValue::y()).unwrap();
    let z_coord = model.add_function("z_coord", XYZValue::z()).unwrap();

    // Constants
    let a_sqrt = model.add_constant("a", a.powi(2)).unwrap();
    let b_sqrt = model.add_constant("b", b.powi(2)).unwrap();

    // X and Y terms
    let x_sqrt =
        model.add_operation_with_inputs("x_sqrt", Multiply::new(), &[&x_coord, &x_coord])?;
    let y_sqrt =
        model.add_operation_with_inputs("y_sqrt", Multiply::new(), &[&y_coord, &y_coord])?;

    let x_term = model.add_operation_with_inputs("x_term", Divide::new(), &[&x_sqrt, &a_sqrt])?;
    let y_term = model.add_operation_with_inputs("y_term", Divide::new(), &[&y_sqrt, &b_sqrt])?;

    // Final iso value
    let sub_1 = model.add_operation_with_inputs("sub_1", Subtract::new(), &[&y_term, &x_term])?;

    let _ = model.add_operation_with_inputs("sub_2", Subtract::new(), &[&z_coord, &sub_1])?;

    Ok(model)
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct HyperbolicParaboloid<T: Serialize> {
    a: T,
    b: T,
}

// Default implementation of base trait.
impl<T: Send + Sync + Serialize> ImplicitComponent<T> for HyperbolicParaboloid<T> {}

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for HyperbolicParaboloid<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        (y.powi(2) / self.b.powi(2)) - (x.powi(2) / self.a.powi(2)) - z
    }
}

pub fn main() {
    utils::logging::init_info();

    let size: f32 = 5.0;
    let cell_size = 0.1;
    let bounds = BoundingBox::new(Vec3::new(-size, -size, -size), Vec3::new(size, size, size));

    let a: f32 = 2.0;
    let b: f32 = 3.0;
    let model = create_hyperbolic_paraboliod(a, b).unwrap();

    let mut sampler = DenseSampler::builder()
        .with_bounds(bounds)
        .with_cell_size(cell_size)
        .build()
        .unwrap();

    sampler.sample_field(&model).expect("Sampling should work.");

    let mesh = sampler
        .iso_surface(0.0)
        .expect("Extracting iso-surface should work.");

    utils::io::write_obj_file(&mesh, "gyroid_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(bounds));
    }
}
