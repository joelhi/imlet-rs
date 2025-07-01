use imlet::types::{
    computation::{
        data::sampler::{DenseSampler, Sampler},
        model::ImplicitModel,
        traits::{ImplicitComponent, ImplicitFunction},
        ModelError,
    },
    geometry::{BoundingBox, Vec3},
};
use imlet::utils;
use num_traits::Float;
use serde::Serialize;

// Custom implicit function.
#[derive(Debug, Serialize)]
pub struct HyperbolicParaboloid<T> {
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

/// Create a new implicit model representing a hyperbolic paraboloid surface.
fn create_hyperbolic_paraboliod(a: f32, b: f32) -> Result<ImplicitModel<f32>, ModelError> {
    let mut model = ImplicitModel::new();
    let func = HyperbolicParaboloid { a, b };
    model.add_function("func", func)?;
    Ok(model)
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

    utils::io::write_obj_file(&mesh, "custom_function_example").unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(bounds));
    }
}
