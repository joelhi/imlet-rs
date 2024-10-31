use crate::types::computation::{Data, Parameter};
use std::any::type_name;

/// Trait to define a distance function in 3d space.
///
/// This trait provides the framework for evaluating distance functions as part of an implicit model.
pub trait ImplicitFunction<T>: Sync + Send {
    /// Evaluate a function in 3 dimensional space. *f(x,y,z)->value*
    ///
    /// This function will be evaluated at each sample point in an implicit model.
    /// # Arguments
    ///
    /// * `x` - X coordinate to evaluate.
    /// * `y` - Y coordinate to evaluate.
    /// * `z` - Z coordinate to evaluate.
    fn eval(&self, x: T, y: T, z: T) -> T;

    /// Declare variable parameters for the function.
    ///
    /// If no parameters are applicable, this can just return an empty array.
    fn parameters(&self) -> &[Parameter];

    /// Process the input from one of the declared public parameters.
    ///
    /// The provided value should be assigned where intended, using the mutable reference to self.
    ///
    /// If there are no parameters exposed, this shoudn't do anything.
    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>);

    /// Get the value of a parameter.
    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>>;

    /// Name of the function
    fn function_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// Trait to define an operation to be performed as part of an implicit model computation.
///
/// This is used to define custom operations on data in an implicit model, independent of global coordinates.
///
/// For example simple arithmetic or boolean operations.
pub trait ImplicitOperation<T>: Sync + Send {
    /// Perform the operation based on the input values.
    /// # Arguments
    ///
    /// * `inputs` - Inputs for operation, passed from components in implicit model.
    fn eval(&self, inputs: &[T]) -> T;

    /// Communicates to the model the number of inputs required for this operation.
    fn inputs(&self) -> &[&str];

    /// Declare variable parameters for the function.
    ///
    /// If no parameters are applicable, this can just return an empty array.
    fn parameters(&self) -> &[Parameter];

    /// Process the input from one of the declared public parameters.
    ///
    /// The provided value should be assigned where intended, using the mutable reference to self.
    ///
    /// If there are no parameters exposed, this shoudn't do anything.
    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>);

    /// Get the value of a parameter.
    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>>;

    /// Name of the operation
    fn operation_name(&self) -> &'static str {
        type_name::<Self>()
    }
}
