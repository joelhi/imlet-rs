use std::{cell::RefCell, time::Instant};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::engine::types::geometry::{BoundingBox, Vec3i};

use super::{
    component::{Component, ComponentId, ImplicitFunction, ImplicitOperation},
    DenseFieldF32,
};

const MAX_TOTAL_COMPONENTS: usize = 512;

pub struct Model {
    components: Vec<Component>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            components: Vec::new(),
        }
    }

    pub fn add_function<T: ImplicitFunction + 'static>(&mut self, function: T) -> ComponentId {
        self.components
            .push(Component::Function(Box::new(function)));
        (self.components.len() - 1).into()
    }

    pub fn add_operation<T: ImplicitOperation + 'static>(&mut self, operation: T) -> ComponentId {
        self.components
            .push(Component::Operation(Box::new(operation)));
        (self.components.len() - 1).into()
    }

    pub fn add_constant(&mut self, value: f32) -> ComponentId {
        self.components.push(Component::Constant(value));
        (self.components.len() - 1).into()
    }

    thread_local! {
        static COMPONENT_VALUES: RefCell<[f32; MAX_TOTAL_COMPONENTS]> = RefCell::new([0.0; MAX_TOTAL_COMPONENTS]);
    }

    fn evaluate_at_coord(&self, x: f32, y: f32, z: f32, output: ComponentId) -> f32 {
        Self::COMPONENT_VALUES.with(|values| {
            let mut values = values.borrow_mut();
            for (index, component) in self.components.iter().enumerate() {
                values[index] = component.compute(x, y, z, &values.as_slice());
                if index == output.value() {
                    break;
                }
            }
            values[output.value()]
        })
    }

    pub fn evaluate(
        &self,
        bounds: BoundingBox,
        cell_size: f32,
        output: ComponentId,
    ) -> DenseFieldF32 {
        let before = Instant::now();
        let n = Self::get_point_count(&bounds, cell_size);
        let mut data: Vec<f32> = vec![0.0; n.x * n.y * n.z];
        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let (i, j, k) = DenseFieldF32::index3d_from_index1d(index, n.x, n.y, n.z);
            *value = self.evaluate_at_coord(
                cell_size * i as f32,
                cell_size * j as f32,
                cell_size * k as f32,
                output,
            );
        });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            n.x * n.y * n.z,
            before.elapsed()
        );

        DenseFieldF32::new(bounds.min, cell_size, n, data)
    }

    fn get_point_count(bounds: &BoundingBox, cell_size: f32) -> Vec3i {
        let (x_dim, y_dim, z_dim) = bounds.get_dimensions();
        Vec3i::new(
            (x_dim / cell_size).floor() as usize + 1,
            (y_dim / cell_size).floor() as usize + 1,
            (z_dim / cell_size).floor() as usize + 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::types::{
        computation::{
            distance_functions::Sphere,
            operations::{arithmetic::Add, boolean::Difference},
        },
        geometry::Vec3f,
    };

    use super::*;

    #[test]
    fn test_evaluate_model_function() {
        let size = 10.0;
        let cell_size = 2.5;
        let mut model = Model::new();
        let bounds = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

        // Function
        let sphere = model.add_function(Sphere::new(
            Vec3f::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.45,
        ));

        // Discretize
        let field = model.evaluate(bounds, cell_size, sphere);

        assert_eq!(64, field.get_num_cells());
        assert_eq!(125, field.get_num_points());

        let data = field.copy_data();
        for val in data {
            println!("{val},");
        }
    }

    #[test]
    fn test_evaluate_model_function_non_uniform() {
        let size = 10.0;
        let cell_size = 2.5;
        let mut model = Model::new();
        let bounds = BoundingBox::new(Vec3f::origin(), Vec3f::new(2.0 * size, 1.5 * size, size));

        // Function
        let sphere = model.add_function(Sphere::new(
            Vec3f::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.50,
        ));

        // Discretize
        let field = model.evaluate(bounds, cell_size, sphere);

        assert_eq!(8 * 6 * 4, field.get_num_cells());
        assert_eq!(9 * 7 * 5, field.get_num_points());

        // Assert values
        let data = field.copy_data();
        for val in data {
            println!("{val},");
        }
    }

    #[test]
    fn test_create_and_evaluate_model_with_function_operation() {
        let mut model = Model::new();

        // Function
        let sphere_component = model.add_function(Sphere::new(Vec3f::origin(), 1.0));

        let sphere_component_2: ComponentId = model.add_function(Sphere::new(Vec3f::origin(), 0.5));

        let difference_component =
            model.add_operation(Difference::new(sphere_component, sphere_component_2));

        assert!(0.5 - model.evaluate_at_coord(0.0, 0.0, 0.0, difference_component) < 0.001);
        assert!(model.evaluate_at_coord(0.5, 0.0, 0.0, difference_component) < 0.001);
        assert!(model.evaluate_at_coord(1.0, 0.0, 0.0, difference_component) < 0.001);
        assert!(model.evaluate_at_coord(0.0, 0.5, 0.0, difference_component) < 0.001);
        assert!(model.evaluate_at_coord(0.0, 1.0, 0.0, difference_component) < 0.001);
        assert!(
            (-0.25 - model.evaluate_at_coord(0.75, 0.0, 0.0, difference_component)).abs() < 0.001
        );
        assert!(
            (-0.25 - model.evaluate_at_coord(0.0, 0.75, 0.0, difference_component)).abs() < 0.001
        );
    }

    #[test]
    fn test_evaluate_model_constant_operation() {
        let mut model = Model::new();

        let value_component = model.add_constant(1.0);
        let addition_component = model.add_operation(Add::new(value_component, value_component));

        let result = model.evaluate_at_coord(0.0, 0.0, 0.0, addition_component);
        assert!((2.0 - result).abs() < 0.0001);
    }
}
