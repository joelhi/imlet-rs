use crate::algorithms::marching_cubes::generate_iso_surface;
use crate::types::computation::component::{Component, ComponentId};
use crate::types::computation::traits::implicit_functions::{ImplicitFunction, ImplicitOperation};
use crate::types::computation::ComputationGraph;
use crate::types::geometry::{BoundingBox, Mesh};
use num_traits::Float;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::time::Instant;

use super::ScalarField;

/// Defines an implicit model composed of distance functions and operations.
pub struct ImplicitModel<T: Float + Debug> {
    components: HashMap<String, Component<T>>,
    inputs: HashMap<String, Vec<Option<String>>>,
}

impl<T: Float + Debug + Send + Sync> ImplicitModel<T> {
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
    /// * `Result<(), String>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.
    pub fn add_function<F: ImplicitFunction<T> + 'static>(
        &mut self,
        tag: &str,
        function: F,
    ) -> Result<(), String> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        self.components
            .insert(tag_string, Component::Function(Box::new(function)));

        Ok(())
    }

    /// Add a distance function component to the model.
    /// # Arguments
    ///
    /// * `tag` - The tag of the operation component added. This is used to reference the component for input and output assignments.
    /// * `operation` - The operation to add.
    /// # Returns
    ///      
    /// * `Result<(), String>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.
    pub fn add_operation<F: ImplicitOperation<T> + 'static>(
        &mut self,
        tag: &str,
        operation: F,
    ) -> Result<(), String> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        self.inputs
            .insert(tag_string.clone(), vec![None; operation.num_inputs()]);
        self.components
            .insert(tag_string, Component::Operation(Box::new(operation)));

        Ok(())
    }

    /// Add a operation component to the model, and populate it with inputs.
    /// # Arguments
    ///
    /// * `tag` - The tag of the operation component added. This is used to reference the component for input and output assignments.
    /// * `function` - The operation to add.
    /// * `inputs` - The tags of the components which provide the inputs. The number of inputs must match the operation added.
    /// # Returns
    ///      
    /// * `Result<(), String>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.
    pub fn add_operation_with_inputs<F: ImplicitOperation<T> + 'static>(
        &mut self,
        tag: &str,
        operation: F,
        inputs: &[&str],
    ) -> Result<(), String> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        if operation.num_inputs() != inputs.len() {
            return Err(format!(
                "Number of inputs for component '{}' does not match the inputs for '{}'. Expected {}, got {}.",
                tag,
                std::any::type_name::<F>(),
                operation.num_inputs(),
                inputs.len()
            ));
        }

        self.inputs.insert(
            tag_string.clone(),
            inputs.iter().map(|s| Some(s.to_string())).collect(),
        );

        self.components
            .insert(tag.to_string(), Component::Operation(Box::new(operation)));

        Ok(())
    }

    /// Add a tagged constant value to the model, which can be processed in other components.
    /// # Arguments
    ///
    /// * `tag` - The tag of the value component added. This is used to reference the component for input assignments.
    /// * `value` - The constant value.
    /// # Returns
    ///      
    /// * `Result<(), String>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is already in use.

    pub fn add_constant(&mut self, tag: &str, value: T) -> Result<(), String> {
        let tag_string = tag.to_string();
        self.verify_tag_is_free(&tag_string)?;

        self.components
            .insert(tag_string, Component::Constant(value));

        Ok(())
    }

    /// Assign an input to a component.
    /// # Arguments
    ///
    /// * `target` - The tag of the operation which recieves the input.
    /// * `source` - The tag of the output source to feed as input.
    /// * `index` - The input index of the targer to which the output source is assigned.
    /// # Returns
    ///      
    /// * `Result<(), String>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the source or target tags are not found in the model.
    pub fn add_input(&mut self, target: &str, source: &str, index: usize) -> Result<(), String> {
        let target_string = target.to_string();
        self.verify_tag_is_present(&target_string)?;
        let source_string = source.to_string();
        self.verify_tag_is_present(&source_string)?;

        let target_component_inputs = self
            .inputs
            .get_mut(target)
            .expect("Target component not found in model.");

        if index > target_component_inputs.len() {
            return Err(format!(
                "Input '{}' is larger than the number of inputs for '{}', which has {} inputs.",
                index,
                target_string,
                target_component_inputs.len()
            ));
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
    /// * `Result<(), String>` - Returns `Ok(())` if the function is added successfully, or `Err(String)` if something goes wrong, such as when the tag is not found in the model.

    pub fn remove_input(&mut self, component: &String, index: usize) {
        let component_inputs = self
            .inputs
            .get_mut(component)
            .expect("Target component not found in model.");
        assert!(
            index < component_inputs.len(),
            "Input index out of bounds for target component. "
        );

        component_inputs[index] = None;
    }

    fn verify_tag_is_free(&self, tag: &String) -> Result<(), String> {
        if self.components.contains_key(tag) {
            return Err(format!(
                "A component with tag '{}' is already present in the model.",
                tag
            ));
        }

        Ok(())
    }

    fn verify_tag_is_present(&self, tag: &String) -> Result<(), String> {
        if !self.components.contains_key(tag) {
            return Err(format!(
                "A component with tag '{}' is not present in the model",
                tag
            ));
        }

        Ok(())
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
    ) -> ScalarField<T> {
        let computation_graph = self.compile(output);
        computation_graph.evaluate(&bounds, cell_size)
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
    ) -> Mesh<T> {
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
    ) -> Mesh<T> {
        let field = self.generate_field(output, &bounds, cell_size);

        let triangles = generate_iso_surface(&field, iso_value);
        Mesh::from_triangles(&triangles)
    }

    fn compile(&self, target: &str) -> ComputationGraph<T> {
        let before = Instant::now();
        let target_output = target.to_string();

        // Traverse model from target to resolve all dependents
        let mut graph = ComputationGraph::new();
        let mut ordered_components = Vec::new();
        let mut ordered_inputs = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(target_output.clone());

        // Find all sources for the target
        let mut sources = HashMap::new();
        while let Some(front) = queue.pop_front() {
            if sources.contains_key(&front) {
                continue;
            }
            sources.insert(front.clone(), sources.len());
            for component in self.valid_inputs(&front) {
                assert!(
                    !sources.contains_key(&component),
                    "Cyclical dependency detected for {}. Make sure it's inputs are not dependent on it's own output.",
                    component
                );
                queue.push_back(component);
            }
        }
        let num_sources = sources.len() - 1;
        sources
            .iter_mut()
            .for_each(|(_, index)| *index = num_sources - *index);

        ordered_components.resize(sources.len(), String::new());
        ordered_inputs.resize(sources.len(), vec![]);

        // Iterate all the sources into an ordered list and assign indexed input
        for (component, index) in sources.iter() {
            ordered_components[*index].insert_str(0, component.as_str());
            ordered_inputs[*index].extend(self.valid_inputs(component).iter().map(|tag| {
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

        graph
    }

    fn valid_inputs(&self, component: &String) -> Vec<String> {
        let default = Vec::new();
        let option_inputs = self.inputs.get(component).unwrap_or(&default);

        option_inputs.into_iter().fold(Vec::new(), |mut acc, item| {
            item.clone()
                .map(|s| {
                    acc.push(s);
                    acc
                })
                .expect(&format!("Component {} is missing an input.", component))
        })
    }
}
