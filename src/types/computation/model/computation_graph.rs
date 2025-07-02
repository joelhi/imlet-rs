use std::cell::RefCell;

use num_traits::Float;
use serde::Serialize;
use smallvec::SmallVec;

use crate::{types::computation::traits::ModelFloat, utils::math_helper::Pi};

use super::{ComponentId, ComponentValues, ModelComponent};

/// Number of inputs that are stack-allocated, when collecting the inputs for each component.
/// If a components has more inputs, they will be on the heap. It's allowed but will probably slow things down a bit.
const INPUT_STACK_BUFFER_SIZE: usize = 8;

/// Defines a set of components which should be computed to generate an output.
///
/// The components are extracted from the model based on the target output.
pub(crate) struct ComputationGraph<'a, T: ModelFloat + 'static> {
    components: Vec<&'a ModelComponent<T>>,
    inputs: Vec<Vec<ComponentId>>,
}

impl<'a, T: ModelFloat + 'static > ComputationGraph<'a, T> {
    /// Create a new, empty, computation graph.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            inputs: Vec::new(),
        }
    }

    /// Add the reference to a [`ModelComponent`] from the main model, which should be computed.
    pub fn add_component(&mut self, component: &'a ModelComponent<T>, inputs: Vec<ComponentId>) {
        self.components.push(component);
        self.inputs.push(inputs);
    }
}

impl<T: ModelFloat + 'static> ComputationGraph<'_, T> {
    thread_local! {
        static COMPONENT_VALUES: RefCell<ComponentValues> = RefCell::new(ComponentValues::new());
    }

    /// Evaluate the computation graph at a specific coordinate.
    pub fn evaluate_at_coord(&self, x: T, y: T, z: T) -> T {
        Self::COMPONENT_VALUES.with(|values| {
            let mut values = values.borrow_mut();
            values.resize(self.components.len());

            for (index, &component) in self.components.iter().enumerate() {
                let inputs = self.inputs(index, &values);
                let val = component.compute(x, y, z, &inputs);
                values.set(index, val);
            }
            values.last()
        })
    }

    /// Retrieve the values for the inputs of a component.
    #[inline(always)]
    fn inputs(
        &self,
        component_id: usize,
        values: &ComponentValues,
    ) -> SmallVec<[T; INPUT_STACK_BUFFER_SIZE]> {
        let mut inputs = SmallVec::<[T; INPUT_STACK_BUFFER_SIZE]>::new();
        for &id in self.inputs[component_id].iter() {
            inputs.push(values.get(id));
        }
        inputs
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::{
            data::DenseField,
            operations::{math::Add, shape::BooleanDifference},
        },
        geometry::{BoundingBox, Sphere, Vec3},
    };

    use super::*;

    #[test]
    fn test_evaluate_model_function() {
        let size = 10.0;
        let cell_size = 2.5;
        let mut graph = ComputationGraph::new();
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(size, size, size));

        // Function
        let binding = ModelComponent::Function(Box::new(Sphere::new(
            Vec3::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.45,
        )));

        graph.add_component(&binding, vec![]);

        // Discretize
        let mut field = DenseField::new(
            Vec3::origin(),
            cell_size,
            DenseField::point_count(&bounds, cell_size),
        );

        field.sample_from_graph(&graph);

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
        let sphere = ModelComponent::Function(Box::new(Sphere::new(
            Vec3::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.50,
        )));

        model.add_component(&sphere, vec![]);

        // Discretize
        let mut field = DenseField::new(
            Vec3::origin(),
            cell_size,
            DenseField::point_count(&bounds, cell_size),
        );
        field.sample_from_graph(&model);
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
        let sphere_component = ModelComponent::Function(Box::new(Sphere::new(Vec3::origin(), 1.0)));
        let sphere_component2 =
            ModelComponent::Function(Box::new(Sphere::new(Vec3::origin(), 0.5)));

        let difference_component = ModelComponent::Operation(Box::new(BooleanDifference::new()));

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

        model.add_component(&ModelComponent::Constant(1.0), vec![]);
        let addition_component = ModelComponent::Operation(Box::new(Add::new()));
        model.add_component(&addition_component, vec![0.into(), 0.into()]);

        let result = model.evaluate_at_coord(0.0, 0.0, 0.0);
        assert!((2.0 - result).abs() < 0.0001);
    }
}
