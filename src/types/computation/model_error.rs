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
    MissingInput {
        component: String,
        index: usize,
    },
    TagGenerationFailed(String),
    /// Can't compute as no output specified.
    MissingOutput(),
    /// Can't compute as no model config is defined.
    MissingConfig(),
    /// A required param is missing from a builder.
    MissingRequiredParam(String),
    /// Data size for the dense field does not match the specified size.
    IncorrectDataSize(usize, usize),
    /// Model has no default output assigned.
    NoDefaultOutput,
    /// A generic error with a custom message.
    Custom(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::CyclicDependency(component) => {
                write!(
                    f,
                    "Cyclic dependency detected for {component}. The component is depending on itself."
                )
            }
            ModelError::MissingTag(tag) => {
                write!(f, "Could not find component with tag {tag} in model.")
            }
            ModelError::DuplicateTag(tag) => {
                write!(f, "Component with tag {tag} already present in model.")
            }
            ModelError::InputIndexOutOfRange {
                component,
                num_inputs,
                index,
            } => {
                write!(f, "Input index out of bounds for component {component}. The recieved index ({index}) is larger than the input count for the component ({num_inputs})")
            }
            ModelError::IncorrectInputCount {
                component,
                num_inputs,
                count,
            } => {
                write!(f, "Incorrect inputs for component {component}. The recieved number ({count}) is larger than the input count for the component ({num_inputs})")
            }
            ModelError::MissingInput { component, index } => {
                write!(
                    f,
                    "Component {component} is missing an input at index {index}. The model cannot be computed."
                )
            }
            ModelError::TagGenerationFailed(tag) => {
                write!(f, "Failed to generate increment for tag {tag}.")
            }
            ModelError::MissingOutput() => {
                write!(f, "Failed to generate output as no output node specified.")
            }
            ModelError::MissingConfig() => write!(
                f,
                "Failed to generate output as no config is specified for the model."
            ),
            ModelError::MissingRequiredParam(param) => {
                write!(f, "Required parameter {param} is missing from the builder.")
            }
            ModelError::Custom(message) => write!(f, "{message}"),
            ModelError::IncorrectDataSize(data_size, field_size) => {
                write!(f, "The provided data buffer of size (size={data_size}) is not matching the point count of the field (n={field_size}).")
            }
            ModelError::NoDefaultOutput => {
                write!(f, "The model has no default output set. Please set one or provide a specific component.")
            }
        }
    }
}

impl std::error::Error for ModelError {}
