use crate::types::computation::model::{Data, Parameter};
use std::any::type_name;

/// Trait to defin a distance function in 3d space.
///
/// This trait provides the framework for evaluating distance functions as part of an implicit model.
pub trait ImplicitFunction<T>: ImplicitComponent<T> {
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
pub trait ImplicitOperation<T>: ImplicitComponent<T> {
    /// Perform the operation based on the input values.
    /// # Arguments
    ///
    /// * `inputs` - Inputs for operation, passed from components in implicit model.
    fn eval(&self, inputs: &[T]) -> T;

    /// Communicates to the model the number of inputs required for this operation.
    fn inputs(&self) -> &[&str];
}

/// Trait for general functionality of an implicit component.
/// 
/// The trait offers the ability to expose parameters, which can be manipulated at runtime.
/// By default nothing is exposed.
pub trait ImplicitComponent<T>: Sync + Send + erased_serde::Serialize  {
    /// Declare variable parameters for the component.
    ///
    /// If no parameters are applicable, this can just return an empty array.
    fn parameters(&self) -> &[Parameter]{
        &[]
    }

    /// Process the input from one of the declared public parameters.
    ///
    /// The provided value should be assigned where intended, using the mutable reference to self.
    ///
    /// If there are no parameters exposed, this shoudn't do anything.
    fn set_parameter(&mut self, _parameter_name: &str, _data: Data<T>){}

    /// Get the value of a parameter.
    fn read_parameter(&self, _parameter_name: &str) -> Option<Data<T>>{
        None
    }

    /// Name of the component.
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}
