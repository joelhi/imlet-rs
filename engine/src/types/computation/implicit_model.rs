use crate::types::computation::component::{Component, ComponentId};
use crate::types::computation::traits::implicit_functions::{ImplicitFunction, ImplicitOperation};
use crate::types::computation::ComputationGraph;
use num_traits::Float;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::time::Instant;

pub struct ImplicitModel<T: Float + Debug> {
    components: HashMap<String, Component<T>>,
    inputs: HashMap<String, Vec<Option<String>>>,
}

impl<T: Float + Debug + Send + Sync> ImplicitModel<T> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            inputs: HashMap::new(),
        }
    }

    pub fn add_function<F: ImplicitFunction<T> + 'static>(&mut self, tag: &str, function: F) {
        self.components
            .insert(tag.to_string(), Component::Function(Box::new(function)));
    }

    pub fn add_operation<F: ImplicitOperation<T> + 'static>(&mut self, tag: &str, operation: F) {
        self.inputs
            .insert(tag.to_string(), vec![None; operation.num_inputs()]);
        self.components
            .insert(tag.to_string(), Component::Operation(Box::new(operation)));
    }

    pub fn add_operation_with_inputs<F: ImplicitOperation<T> + 'static>(
        &mut self,
        tag: &str,
        operation: F,
        inputs: &[&str],
    ) {
        self.inputs.insert(
            tag.to_string(),
            inputs.iter().map(|s| Some(s.to_string())).collect(),
        );
        self.components
            .insert(tag.to_string(), Component::Operation(Box::new(operation)));
    }

    pub fn add_constant(&mut self, tag: String, value: T) {
        self.components
            .insert(tag.clone(), Component::Constant(value));
    }

    pub fn add_input(&mut self, target: &String, source: &String, index: usize) {
        let target_component_inputs = self
            .inputs
            .get_mut(target)
            .expect("Target component not found in model.");
        assert!(
            index < target_component_inputs.len(),
            "Input index out of bounds for target component. "
        );
        assert!(
            self.components.contains_key(source),
            "Source component not found in model. "
        );
        target_component_inputs[index] = Some(source.clone());
    }

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

    pub fn compile(&self, target: &str) -> ComputationGraph<T> {
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
            for component in self.get_valid_inputs(&front) {
                assert!(
                    !sources.contains_key(&component),
                    "Cyclical dependency detected for {}",
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
            ordered_inputs[*index].extend(
                self.get_valid_inputs(component).iter().map(|tag| {
                    ComponentId(*sources.get(tag).expect("Failed to retrieve component."))
                }),
            );
        }

        for (index, component) in ordered_components.iter().enumerate() {
            graph.add_component(
                self.components
                    .get(component)
                    .expect("Failed to retrieve component."),
                ordered_inputs
                    .get(index)
                    .expect("Failed to retrieve inputs")
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

    fn get_valid_inputs(&self, component: &String) -> Vec<String> {
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
