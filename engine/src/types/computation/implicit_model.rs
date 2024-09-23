use crate::algorithms::marching_cubes::generate_iso_surface;
use crate::types::computation::component::{Component, ComponentId};
use crate::types::computation::traits::{ImplicitFunction, ImplicitOperation};
use crate::types::computation::ComputationGraph;
use crate::types::geometry::{BoundingBox, Mesh};
use num_traits::Float;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use super::{ModelError, ScalarField};

/// An implicit model composed of distance functions and operations.
///
/// This acts as the main interface used to build and compute implicit models.
pub struct ImplicitModel<T> {
    components: HashMap<String, Component<T>>,
    inputs: HashMap<String, Vec<Option<String>>>,
}

impl<T> ImplicitModel<T> {
    /// Create a new empty model.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            inputs: HashMap::new(),
        }
    }

    /// Add a distance function component to the model.
    /// # Arguments
    ///
    /// * `tag` - The tag of the function component added. This is used to reference the component for input and output assignments.
    /// * `function` - The function to add.
    /// # Returns
    ///      
    /// * `Result<(), ModelError>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.
    pub fn add_function<F: ImplicitFunction<T> + 'static>(
        &mut self,
        tag: &str,
        function: F,
    ) -> Result<String, ModelError> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        self.components
            .insert(tag_string.clone(), Component::Function(Box::new(function)));

        Ok(tag_string)
    }

    /// Add a distance function component to the model.
    /// # Arguments
    ///
    /// * `tag` - The tag of the operation component added. This is used to reference the component for input and output assignments.
    /// * `operation` - The operation to add.
    /// # Returns
    ///      
    /// * `Result<&str, ModelError>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.
    pub fn add_operation<F: ImplicitOperation<T> + 'static>(
        &mut self,
        tag: &str,
        operation: F,
    ) -> Result<String, ModelError> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        self.inputs
            .insert(tag_string.clone(), vec![None; operation.num_inputs()]);
        self.components.insert(
            tag_string.clone(),
            Component::Operation(Box::new(operation)),
        );

        Ok(tag_string)
    }

    /// Add a operation component to the model, and populate it with inputs.
    /// # Arguments
    ///
    /// * `tag` - The tag of the operation component added. This is used to reference the component for input and output assignments.
    /// * `function` - The operation to add.
    /// * `inputs` - The tags of the components which provide the inputs. The number of inputs must match the operation added.
    /// # Returns
    ///      
    /// * `Result<String, ModelError>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.
    pub fn add_operation_with_inputs<F: ImplicitOperation<T> + 'static>(
        &mut self,
        tag: &str,
        operation: F,
        inputs: &[&str],
    ) -> Result<String, ModelError> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        if operation.num_inputs() != inputs.len() {
            return Err(ModelError::IncorrectInputCount {
                component: tag_string,
                num_inputs: inputs.len(),
                count: operation.num_inputs(),
            });
        }

        self.inputs.insert(
            tag_string.clone(),
            inputs.iter().map(|s| Some(s.to_string())).collect(),
        );

        self.components.insert(
            tag_string.clone(),
            Component::Operation(Box::new(operation)),
        );

        Ok(tag_string)
    }

    /// Add a tagged constant value to the model, which can be processed in other components.
    /// # Arguments
    ///
    /// * `tag` - The tag of the value component added. This is used to reference the component for input assignments.
    /// * `value` - The constant value.
    /// # Returns
    ///      
    /// * `Result<String, ModelError>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.

    pub fn add_constant(&mut self, tag: &str, value: T) -> Result<String, ModelError> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        self.components
            .insert(tag_string.clone(), Component::Constant(value));

        Ok(tag_string)
    }

    /// Assign an input to a component.
    /// # Arguments
    ///
    /// * `target` - The tag of the operation which recieves the input.
    /// * `source` - The tag of the output source to feed as input.
    /// * `index` - The input index of the targer to which the output source is assigned.
    /// # Returns
    ///      
    /// * `Result<(), ModelError>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the source or target tags are not found in the model.
    pub fn add_input(
        &mut self,
        target: &str,
        source: &str,
        index: usize,
    ) -> Result<(), ModelError> {
        let target_string = target.to_string();
        self.verify_tag_is_present(&target_string)?;
        let source_string = source.to_string();
        self.verify_tag_is_present(&source_string)?;
        self.verify_input_validity(&target_string, &source_string, index)?;

        if target_string.eq(&source_string) {
            return Err(ModelError::CyclicDependency(target.to_string()));
        }

        let target_component_inputs = self
            .inputs
            .get_mut(target)
            .expect("Target component not found in model.");

        if index > target_component_inputs.len() {
            return Err(ModelError::InputIndexOutOfRange {
                component: target_string,
                num_inputs: target_component_inputs.len(),
                index,
            });
        }

        target_component_inputs[index] = Some(source_string.clone());

        Ok(())
    }

    /// Remove an input from a component. This will leave the specific input parameter unassigned.
    /// # Arguments
    ///
    /// * `component` - The tag of the operation which recieves the input.
    /// * `index` - The index of the input to unassign.
    ///
    /// # Returns
    ///      
    /// * `Result<(), ModelError>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is not found in the model.
    pub fn remove_input(&mut self, component: &str, index: usize) -> Result<(), ModelError> {
        let component_inputs = self
            .inputs
            .get_mut(component)
            .expect("Target component not found in model.");

        if index > component_inputs.len() {
            return Err(ModelError::InputIndexOutOfRange {
                component: component.to_string(),
                num_inputs: component_inputs.len(),
                index,
            });
        }

        component_inputs[index] = None;
        Ok(())
    }

    fn verify_tag_is_free(&self, tag: &String) -> Result<(), ModelError> {
        if self.components.contains_key(tag) {
            return Err(ModelError::DuplicateTag(tag.clone()));
        }

        Ok(())
    }

    fn verify_tag_is_present(&self, tag: &str) -> Result<(), ModelError> {
        if !self.components.contains_key(tag) {
            return Err(ModelError::MissingTag(tag.to_string()));
        }

        Ok(())
    }

    /// Verify that new input is ok.
    fn verify_input_validity(
        &self,
        target: &str,
        source: &String,
        index: usize,
    ) -> Result<(), ModelError> {
        // Verify that the index is within range
        let inputs = self
            .inputs
            .get(target)
            .ok_or_else(|| ModelError::MissingTag(target.to_string()))?;

        if inputs.len() <= index {
            return Err(ModelError::InputIndexOutOfRange {
                component: target.to_string(),
                num_inputs: inputs.len(),
                index: index,
            });
        }

        // Verify that the source is not dependent on the target.
        let mut queue = VecDeque::new();
        queue.push_back(source.clone());

        // Traverse all sources for the target and verify that source is not dependent on target.
        while let Some(front) = queue.pop_front() {
            if front.eq(target) {
                // Component depends on itself. Return an error.
                return Err(ModelError::CyclicDependency(target.to_string()));
            }
            for component in self.valid_inputs(&front)? {
                queue.push_back(component);
            }
        }

        Ok(())
    }

    /// Return all the sources upon which a component depends.
    ///
    /// Returns a HashMap with all dependends by tag and index if valid. Will return an error if a successful traversal is impossible, for example if a cyclical dependecy is found.
    fn gather_sources_for_component(
        &self,
        tag: &str,
    ) -> Result<HashMap<String, usize>, ModelError> {
        let mut queue = VecDeque::new();
        for component in self.valid_inputs(tag)? {
            queue.push_back(component);
        }

        // Find all sources for the target
        let mut sources = HashMap::new();
        while let Some(front) = queue.pop_front() {
            if front.eq(tag) {
                // Component depends on itself. Return an error.
                return Err(ModelError::CyclicDependency(tag.to_string()));
            }
            if sources.contains_key(&front) {
                continue;
            }
            sources.insert(front.clone(), sources.len() + 1);
            for component in self.valid_inputs(&front)? {
                if !sources.contains_key(&component) {
                    queue.push_back(component);
                }
            }
        }

        sources.insert(tag.to_string(), 0);
        Ok(sources)
    }

    fn compile(&self, target: &str) -> Result<ComputationGraph<T>, ModelError> {
        let before = Instant::now();
        let target_output = target.to_string();

        // Traverse model from target to resolve all dependents
        let mut graph = ComputationGraph::new();
        let mut ordered_components = Vec::new();
        let mut ordered_inputs = Vec::new();

        let mut sources = self.gather_sources_for_component(&target_output)?;
        let num_sources = sources.len() - 1;
        sources
            .iter_mut()
            .for_each(|(_, index)| *index = num_sources - *index);

        ordered_components.resize(sources.len(), String::new());
        ordered_inputs.resize(sources.len(), vec![]);

        // Iterate all the sources into an ordered list and assign indexed input
        for (component, index) in sources.iter() {
            ordered_components[*index].insert_str(0, component.as_str());
            ordered_inputs[*index].extend(self.valid_inputs(component)?.iter().map(|tag| {
                ComponentId(*sources.get(tag).expect(&format!(
                    "No component with tag {} found. The model may be corrupt or an invalid target is requested.",
                    tag
                )))
            }));
        }

        for (index, component) in ordered_components.iter().enumerate() {
            graph.add_component(
                self.components.get(component).expect(&format!(
                    "No component with tag {} found. The model may be corrupt or an invalid target is requested.",
                    component
                )),
                ordered_inputs
                    .get(index)
                    .expect(&format!(
                        "No component with tag {} found. The model may be corrupt or an invalid target is requested.",
                        component
                    ))
                    .to_vec(),
            )
        }

        log::info!(
            "Computation graph with {} components compiled in {:.2?}",
            ordered_components.len(),
            before.elapsed()
        );

        Ok(graph)
    }

    fn valid_inputs(&self, component: &str) -> Result<Vec<String>, ModelError> {
        let default = Vec::new();
        let option_inputs = self.inputs.get(component).unwrap_or(&default);

        option_inputs
            .iter()
            .enumerate()
            .map(|(index, item)| {
                item.clone().ok_or_else(|| ModelError::MissingInput {
                    component: component.to_string(),
                    index: index,
                })
            })
            .collect()
    }
}

impl<T: Float + Send + Sync> ImplicitModel<T> {
    /// Evaluate the model at a coordinate *{x, y, z}*.
    /// # Arguments
    ///
    /// * `output` - The tag of the component for which the output should be returned.
    /// * `x` - X coordinate to evaluate at.
    /// * `y` - Y coordinate to evaluate at.
    /// * `z` - Z coordinate to evaluate at.
    ///
    /// # Returns
    ///      
    /// * `ScalarField<T>` - The scalar field holding the computed data.
    pub fn evaluate_at(&self, output: &str, x: T, y: T, z: T) -> Result<T, ModelError> {
        let computation_graph = self.compile(output)?;
        Ok(computation_graph.evaluate_at_coord(x, y, z))
    }

    /// Compute a discrete scalar field from the model.
    /// # Arguments
    ///
    /// * `output` - The tag of the component for which the output should be stored in the field.
    /// * `bounds` - The domain to compute.
    /// * `cell_size` - The resolution at which the domain is computed.
    ///
    /// # Returns
    ///      
    /// * `ScalarField<T>` - The scalar field holding the computed data.
    pub fn generate_field(
        &self,
        output: &str,
        bounds: &BoundingBox<T>,
        cell_size: T,
    ) -> Result<ScalarField<T>, ModelError> {
        let computation_graph = self.compile(output)?;
        Ok(computation_graph.evaluate(&bounds, cell_size))
    }

    /// Extract the iso surface at the zero-level.
    /// # Arguments
    ///
    /// * `output` - The tag of the component which output should be used for the iso surface extraction.
    /// * `bounds` - The domain to compute.
    /// * `cell_size` - The resolution at which the domain is computed.
    ///
    /// # Returns
    ///      
    /// * `Mesh<T>` - The iso surface represented as an indexed triangle mesh.
    pub fn generate_iso_surface(
        &self,
        output: &str,
        bounds: &BoundingBox<T>,
        cell_size: T,
    ) -> Result<Mesh<T>, ModelError> {
        self.generate_iso_surface_at(output, bounds, cell_size, T::zero())
    }

    /// Extract the iso surface at a specified level.
    /// # Arguments
    ///
    /// * `output` - The tag of the component which output should be used for the iso surface extraction.
    /// * `bounds` - The domain to compute.
    /// * `cell_size` - The resolution at which the domain is computed.
    /// * `iso_value` - Specific value at which the iso surface should be extracted.
    ///
    /// # Returns
    ///      
    /// * `Mesh<T>` - The iso surface represented as an indexed triangle mesh.    
    pub fn generate_iso_surface_at(
        &self,
        output: &str,
        bounds: &BoundingBox<T>,
        cell_size: T,
        iso_value: T,
    ) -> Result<Mesh<T>, ModelError> {
        let field = self.generate_field(output, &bounds, cell_size)?;

        let triangles = generate_iso_surface(&field, iso_value);
        Ok(Mesh::from_triangles(&triangles))
    }
}

#[cfg(test)]
mod tests {

    use crate::types::computation::operations::math::Add;

    use super::*;

    #[test]
    fn test_build_model_with_input_connections() {
        let mut model = ImplicitModel::new();

        model.add_constant("Value", 1.0).unwrap();
        model.add_operation("Add", Add::new()).unwrap();

        model.add_input("Add", "Value", 0).unwrap();
        model.add_input("Add", "Value", 1).unwrap();

        let val = model.evaluate_at("Add", 0.0, 0.0, 0.0).unwrap();

        assert!(
            (val - 2.0).abs() < f64::epsilon(),
            "Incorrect value. Expected 2.0 but was {}",
            val
        );
    }

    #[test]
    fn test_error_with_cyclic_dependecies() {
        let mut model = ImplicitModel::new();

        model.add_constant("Value", 1.0).unwrap();
        model.add_operation("Add", Add::new()).unwrap();

        model.add_input("Add", "Value", 0).unwrap();
        model.add_input("Add", "Value", 1).unwrap();

        model.add_operation("Add2", Add::new()).unwrap();

        model.add_input("Add2", "Add", 0).unwrap();
        model.add_input("Add2", "Value", 1).unwrap();

        let val = model.evaluate_at("Add2", 0.0, 0.0, 0.0).unwrap();

        assert!(
            (val - 3.0).abs() < f64::epsilon(),
            "Incorrect value. Expected 3.0 but was {}",
            val
        );

        model.remove_input("Add", 0).unwrap();

        let error = model.add_input("Add", "Add2", 0).unwrap_err();

        assert!(matches!(error, ModelError::CyclicDependency { .. }));
    }

    #[test]
    fn test_error_incorrect_input_count() {
        let mut model = ImplicitModel::new();

        model.add_constant("Value", 1.0).unwrap();
        model.add_operation("Add", Add::new()).unwrap();

        model.add_input("Add", "Value", 0).unwrap();
        model.add_input("Add", "Value", 1).unwrap();

        // Out of bounds
        let error = model.add_input("Add", "Value", 2).unwrap_err();

        assert!(matches!(error, ModelError::InputIndexOutOfRange { .. }));
    }

    #[test]
    fn test_error_missing_tag() {
        let mut model = ImplicitModel::new();

        model.add_constant("Value", 1.0).unwrap();
        model.add_operation("Add", Add::new()).unwrap();

        model.add_input("Add", "Value", 0).unwrap();

        // Value2 is not in model
        let error = model.add_input("Add", "Value2", 1).unwrap_err();

        assert!(matches!(error, ModelError::MissingTag { .. }));
    }

    #[test]
    fn test_error_duplicate_tag() {
        let mut model = ImplicitModel::new();

        model.add_constant("Value", 1.0).unwrap();

        // Value is already in model
        let error = model.add_operation("Value", Add::new()).unwrap_err();

        assert!(matches!(error, ModelError::DuplicateTag { .. }));
    }

    #[test]
    fn test_error_missing_input() {
        let mut model = ImplicitModel::new();

        model.add_constant("Value", 1.0).unwrap();
        model.add_operation("Add", Add::new()).unwrap();

        model.add_input("Add", "Value", 0).unwrap();

        let error = model.evaluate_at("Add", 0.0, 0.0, 0.0).unwrap_err();

        assert!(matches!(error, ModelError::MissingInput { .. }));
    }
}
