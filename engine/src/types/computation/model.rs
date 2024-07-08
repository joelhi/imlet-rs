use std::{cell::RefCell, fmt::Debug, time::Instant};

use num_traits::Float;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    types::geometry::{BoundingBox, Vec3i},
    utils::math_helper::index3d_from_index1d,
};

use super::{
    component::{Component, ComponentId, ComponentValues, ImplicitFunction, ImplicitOperation},
    DenseField,
};

pub struct Model<T: Float + Debug + Send + Sync> {
    components: Vec<Component<T>>,
}

impl<T: Float + Debug + Send + Sync> Model<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_function<F: ImplicitFunction<T> + 'static>(&mut self, function: F) -> ComponentId {
        self.components
            .push(Component::Function(Box::new(function)));
        (self.components.len() - 1).into()
    }

    pub fn add_operation<F: ImplicitOperation<T> + 'static>(
        &mut self,
        operation: F,
    ) -> ComponentId {
        self.components
            .push(Component::Operation(Box::new(operation)));
        (self.components.len() - 1).into()
    }

    pub fn add_constant(&mut self, value: T) -> ComponentId {
        self.components.push(Component::Constant(value));
        (self.components.len() - 1).into()
    }

    thread_local! {
        static COMPONENT_VALUES: RefCell<ComponentValues> = RefCell::new(ComponentValues::new());
    }

    fn evaluate_at_coord(&self, x: T, y: T, z: T, output: Option<ComponentId>) -> T {
        Self::COMPONENT_VALUES.with(|values| {
            let mut values = values.borrow_mut();
            let output_index = output.unwrap_or_else(|| ComponentId(self.components.len() - 1));
            for (index, component) in self.components.iter().enumerate() {
                component.compute(x, y, z, &mut values, index);
                if index == output_index.value() {
                    break;
                }
            }
            values.get(output_index)
        })
    }

    pub fn evaluate(
        &self,
        bounds: &BoundingBox<T>,
        cell_size: T,
        output: Option<ComponentId>,
    ) -> DenseField<T> {
        let before = Instant::now();
        let n = Self::get_point_count(&bounds, cell_size);
        let mut data: Vec<T> = vec![T::zero(); n.x * n.y * n.z];
        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let (i, j, k) = index3d_from_index1d(index, n.x, n.y, n.z);
            *value = self.evaluate_at_coord(
                bounds.min.x + cell_size * T::from(i).expect("Failed to convert number to T"),
                bounds.min.x + cell_size * T::from(j).expect("Failed to convert number to T"),
                bounds.min.x + cell_size * T::from(k).expect("Failed to convert number to T"),
                output,
            );
        });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            n.x * n.y * n.z,
            before.elapsed()
        );

        DenseField::with_data(bounds.min, cell_size, n, data)
    }

    fn get_point_count(bounds: &BoundingBox<T>, cell_size: T) -> Vec3i {
        let (x_dim, y_dim, z_dim) = bounds.dimensions();
        Vec3i::new(
            (x_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to convert T to usize")
                + 1,
            (y_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to convert T to usize")
                + 1,
            (z_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to convert T to usize")
                + 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::{
            distance_functions::Sphere,
            operations::{arithmetic::Add, boolean::Difference},
        },
        geometry::Vec3,
    };

    use super::*;

    #[test]
    fn test_evaluate_model_function() {
        let size = 10.0;
        let cell_size = 2.5;
        let mut model = Model::new();
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

        // Function
        let sphere = model.add_function(Sphere::new(
            Vec3::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.45,
        ));

        // Discretize
        let field = model.evaluate(&bounds, cell_size, Some(sphere));

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
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(2.0 * size, 1.5 * size, size));

        // Function
        let sphere = model.add_function(Sphere::new(
            Vec3::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.50,
        ));

        // Discretize
        let field = model.evaluate(&bounds, cell_size, Some(sphere));

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
        let sphere_component = model.add_function(Sphere::new(Vec3::origin(), 1.0));

        let sphere_component_2: ComponentId = model.add_function(Sphere::new(Vec3::origin(), 0.5));

        let difference_component =
            Some(model.add_operation(Difference::new(sphere_component, sphere_component_2)));

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

        let result = model.evaluate_at_coord(0.0, 0.0, 0.0, Some(addition_component));
        assert!((2.0 - result).abs() < 0.0001);
    }
}
