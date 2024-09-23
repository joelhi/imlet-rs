#[derive(Debug)]
pub enum ModelError {
    CyclicDependency(String),
    MissingTag(String),
    DuplicateTag(String),
    InputIndexOutOfRange {
        component: String,
        num_inputs: usize,
        index: usize,
    },
    IncorrectInputCount {
        component: String,
        num_inputs: usize,
        count: usize,
    },
    MissingInput {
        component: String,
        index: usize,
    },
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::CyclicDependency(component) => {
                write!(
                    f,
                    "Cyclic dependency detected for {}. The component is depending on itself.",
                    component
                )
            }
            ModelError::MissingTag(tag) => {
                write!(f, "Could not find component with tag {} in model.", tag)
            }
            ModelError::DuplicateTag(tag) => {
                write!(f, "Component with tag {} already present in model.", tag)
            }
            ModelError::InputIndexOutOfRange {
                component,
                num_inputs,
                index,
            } => {
                write!(f, "Input index out of bounds for component {}. The recieved index ({}) is larget than the input count for the component ({})", component, index, num_inputs)
            }
            ModelError::IncorrectInputCount {
                component,
                num_inputs,
                count,
            } => {
                write!(f, "Incorrect inputs for component {}. The recieved number ({}) is larget than the input count for the component ({})", component, count, num_inputs)
            }
            ModelError::MissingInput { component, index } => {
                write!(
                    f,
                    "Component {} is missing an input at index {}. The model cannot be computed.",
                    component, index
                )
            }
        }
    }
}

impl std::error::Error for ModelError {}
