#[derive(Debug)]
pub enum ModelError {
    InvalidInput { component: String, dependency: String },
    RemoveError(String),
    // Add more variants as needed
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::InvalidInput { component, dependency } => {
                write!(f, "Invalid input component {} specified for {}. The component will depend on itself.", component, dependency)
            }
            ModelError::RemoveError(msg) => write!(f, "Remove error: {}", msg),
        }
    }
}

impl std::error::Error for ModelError {}