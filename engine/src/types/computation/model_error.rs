/// Error variants returned from model building and computation, in the event that something went wrong.
#[derive(Debug)]
pub enum ModelError {
    /// A cyclic dependency was found. This means that one components inputs are dependent on itself.
    CyclicDependency(String),
    /// A referenced tag was not present in the model.
    MissingTag(String),
    /// A component was added with a tag that is already used.
    DuplicateTag(String),
    /// An input was specified to a component that was larger than the total inputs required.
    InputIndexOutOfRange {
        component: String,
        num_inputs: usize,
        index: usize,
    },
    /// A component was added with a list of inputs that doesn't match the number specified by the operation.
    IncorrectInputCount {
        component: String,
        num_inputs: usize,
        count: usize,
    },
    /// Model cannot be computed as a component as an input with no source.
    MissingInput { component: String, index: usize },
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
