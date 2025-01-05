use std::{cell::RefCell, time::Instant};

use num_traits::Float;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::Serialize;
use smallvec::SmallVec;

use crate::{
    types::{
        computation::ScalarField,
        geometry::{BoundingBox, Vec3i},
    },
    utils::{
        self,
        math_helper::{index3d_from_index1d, Pi},
    },
};

use super::{ComponentId, ComponentValues, ModelComponent};

/// Number of inputs that are stack-allocated, when collecting the inputs for each component.
/// If a components has more inputs, they will be on the heap. It's allowed but will probably slow things down a bit.
const INPUT_STACK_BUFFER_SIZE: usize = 8;

/// Defines a set of components which should be computed to generate an output.
///
/// The components are extracted from the model based on the target output.
pub struct ComputationGraph<'a, T: Float + Send + Sync + Serialize + 'static + Pi> {
    components: Vec<&'a ModelComponent<T>>,
    inputs: Vec<Vec<ComponentId>>,
}

impl<'a, T: Float + Send + Sync + Serialize + 'static + Pi> ComputationGraph<'a, T> {
    /// Create a new, empty, computation graph.
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
            inputs: Vec::new(),
        }
    }

    /// Add the reference to a [`ModelComponent`] from the main model, which should be computed.
    pub(crate) fn add_component(
        &mut self,
        component: &'a ModelComponent<T>,
        inputs: Vec<ComponentId>,
    ) {
        self.components.push(component);
        self.inputs.push(inputs);
    }
}

impl<T: Float + Send + Sync + Serialize + 'static + Pi> ComputationGraph<'_, T> {
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

    fn point_count(bounds: &BoundingBox<T>, cell_size: T) -> Vec3i {
        let (x_dim, y_dim, z_dim) = bounds.dimensions();
        Vec3i::new(
            (x_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to get point count from bounds. Make sure the bounds have a positive size in all directions.")
                + 1,
            (y_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to get point count from bounds. Make sure the bounds have a positive size in all directions.")
                + 1,
            (z_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to get point count from bounds. Make sure the bounds have a positive size in all directions.")
                + 1,
        )
    }

    /// Retrieve the values for the inputs of a component.
    #[inline]
    pub fn inputs(
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

impl<T: Float + Send + Sync + Serialize + 'static + Pi> ComputationGraph<'_, T> {
    /// Evaluate the computation graph over a discretized domain.
    pub fn evaluate(&self, bounds: &BoundingBox<T>, cell_size: T) -> ScalarField<T> {
        let before = Instant::now();
        let n = Self::point_count(bounds, cell_size);

        log::info!("Evaluating model with {}x{}x{} points", n.i, n.j, n.k);

        let mut data: Vec<T> = vec![T::zero(); n.product()];
        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let (i, j, k) = index3d_from_index1d(index, n.i, n.j, n.k);
            *value = self.evaluate_at_coord(
                bounds.min.x + cell_size * T::from(i).expect("Failed to convert number to T"),
                bounds.min.y + cell_size * T::from(j).expect("Failed to convert number to T"),
                bounds.min.z + cell_size * T::from(k).expect("Failed to convert number to T"),
            );
        });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            utils::math_helper::format_integer(n.i * n.j * n.k,),
            before.elapsed()
        );

        ScalarField::with_data(bounds.min, cell_size, n, data)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::operations::{math::Add, shape::BooleanDifference},
        geometry::{Sphere, Vec3},
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
        let sphere = ModelComponent::Function(Box::new(Sphere::new(
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
