use std::{cell::RefCell, fmt::Debug, time::Instant};

use num_traits::Float;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    types::geometry::{BoundingBox, Vec3i},
    utils::math_helper::index3d_from_index1d,
};

use super::{
    component::{Component, ComponentId, ComponentValues},
    DenseField,
};

pub struct ComputationGraph<'a, T: Float + Debug + Send + Sync> {
    components: Vec<&'a Component<T>>,
    inputs: Vec<Vec<ComponentId>>,
}

impl<'a, T: Float + Debug + Send + Sync> ComputationGraph<'a, T> {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
            inputs: Vec::new(),
        }
    }

    pub(crate) fn add_component(&mut self, component: &'a Component<T>, inputs: Vec<ComponentId>) {
        self.components.push(&component);
        self.inputs.push(inputs);
    }

    thread_local! {
        static COMPONENT_VALUES: RefCell<ComponentValues> = RefCell::new(ComponentValues::new());
    }

    pub fn evaluate_at_coord(&self, x: T, y: T, z: T) -> T {
        Self::COMPONENT_VALUES.with(|values| {
            let mut values = values.borrow_mut();
            values.resize(self.components.len());

            let mut inputs = Vec::with_capacity(4);
            for (index, component) in self.components.iter().enumerate() {
                self.inputs(index, &values, &mut inputs);
                let val = component.compute(x, y, z, &inputs);
                values.set(index, val);
            }
            values.last()
        })
    }

    pub fn evaluate(&self, bounds: &BoundingBox<T>, cell_size: T) -> DenseField<T> {
        let before = Instant::now();
        let n = Self::point_count(&bounds, cell_size);

        log::info!("Evaluating model with {}x{}x{} points", n.x, n.y, n.z);

        let mut data: Vec<T> = vec![T::zero(); n.x * n.y * n.z];
        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let (i, j, k) = index3d_from_index1d(index, n.x, n.y, n.z);
            *value = self.evaluate_at_coord(
                bounds.min.x + cell_size * T::from(i).expect("Failed to convert number to T"),
                bounds.min.y + cell_size * T::from(j).expect("Failed to convert number to T"),
                bounds.min.z + cell_size * T::from(k).expect("Failed to convert number to T"),
            );
        });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            n.x * n.y * n.z,
            before.elapsed()
        );

        DenseField::with_data(bounds.min, cell_size, n, data)
    }

    fn point_count(bounds: &BoundingBox<T>, cell_size: T) -> Vec3i {
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

    #[inline]
    pub fn inputs(&self, component_id: usize, values: &ComponentValues, inputs: &mut Vec<T>) {
        inputs.clear();
        inputs.resize(self.inputs[component_id].len(), T::zero());
        for (i, &id) in self.inputs[component_id].iter().enumerate() {
            inputs[i] = values.get(id);
        }
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
        let mut graph = ComputationGraph::new();
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

        // Function
        let binding = Component::Function(Box::new(Sphere::new(
            Vec3::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.45,
        )));

        graph.add_component(&binding, vec![]);

        // Discretize
        let field = graph.evaluate(&bounds, cell_size);

        assert_eq!(64, field.num_cells());
        assert_eq!(125, field.num_points());

        let data = field.copy_data();
        for val in data {
            println!("{val},");
        }
    }

    #[test]
    fn test_evaluate_model_function_non_uniform() {
        let size = 10.0;
        let cell_size = 2.5;
        let mut model = ComputationGraph::new();
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(2.0 * size, 1.5 * size, size));

        // Function
        let sphere = Component::Function(Box::new(Sphere::new(
            Vec3::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.50,
        )));

        model.add_component(&sphere, vec![]);

        // Discretize
        let field = model.evaluate(&bounds, cell_size);

        assert_eq!(8 * 6 * 4, field.num_cells());
        assert_eq!(9 * 7 * 5, field.num_points());

        // Assert values
        let data = field.copy_data();
        for val in data {
            println!("{val},");
        }
    }

    #[test]
    fn test_create_and_evaluate_model_with_function_operation() {
        let mut model = ComputationGraph::new();

        // Function
        let sphere_component = Component::Function(Box::new(Sphere::new(Vec3::origin(), 1.0)));
        let sphere_component2 = Component::Function(Box::new(Sphere::new(Vec3::origin(), 0.5)));

        let difference_component = Component::Operation(Box::new(Difference::new()));

        model.add_component(&sphere_component, vec![]);
        model.add_component(&sphere_component2, vec![]);
        model.add_component(&difference_component, vec![0.into(), 1.into()]);

        assert!(0.5 - model.evaluate_at_coord(0.0, 0.0, 0.0) < 0.001);
        assert!(model.evaluate_at_coord(0.5, 0.0, 0.0) < 0.001);
        assert!(model.evaluate_at_coord(1.0, 0.0, 0.0) < 0.001);
        assert!(model.evaluate_at_coord(0.0, 0.5, 0.0) < 0.001);
        assert!(model.evaluate_at_coord(0.0, 1.0, 0.0) < 0.001);
        assert!((-0.25 - model.evaluate_at_coord(0.75, 0.0, 0.0)).abs() < 0.001);
        assert!((-0.25 - model.evaluate_at_coord(0.0, 0.75, 0.0)).abs() < 0.001);
    }

    #[test]
    fn test_evaluate_model_constant_operation() {
        let mut model = ComputationGraph::new();

        model.add_component(&Component::Constant(1.0), vec![]);
        let addition_component = Component::Operation(Box::new(Add::new()));
        model.add_component(&addition_component, vec![0.into(), 0.into()]);

        let result = model.evaluate_at_coord(0.0, 0.0, 0.0);
        assert!((2.0 - result).abs() < 0.0001);
    }
}
