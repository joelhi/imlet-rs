use std::fmt::Debug;

use num_traits::Float;

/// Trait to define a distance function in 3d space.
///
/// This trait provides the framework for evaluating distance functions as part of an implicit model.
pub trait ImplicitFunction<T: Float + Debug + Send + Sync>: Sync + Send {
    /// Evaluate a function in 3 dimensional space. *f(x,y,z)->value*
    ///
    /// This function will be evaluated at each sample point in an implicit model.
    /// # Arguments
    ///
    /// * `x` - X coordinate to evaluate.
    /// * `y` - Y coordinate to evaluate.
    /// * `z` - Z coordinate to evaluate.
    fn eval(&self, x: T, y: T, z: T) -> T;
}

/// Trait to define an operation to be performed as part of an implicit model computation.
///
/// This is used to define custom operations on data in an implicit model, independent of global coordinates.
///
/// For example simple arithmetic or boolean operations.
pub trait ImplicitOperation<T: Float + Debug + Send + Sync>: Sync + Send {
    /// Perform the operation based on the input values.
    /// # Arguments
    ///
    /// * `inputs` - Inputs for operation, passed from components in implicit model.
    fn eval(&self, inputs: &[T]) -> T;

    /// Communicates to the model the number of inputs required for this operation.
    fn num_inputs(&self) -> usize;
}
