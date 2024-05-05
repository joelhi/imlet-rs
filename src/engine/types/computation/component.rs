#[derive(Debug, Copy, Clone)]
pub struct ComponentId(usize);

impl ComponentId {
    pub fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for ComponentId {
    fn from(value: usize) -> Self {
        ComponentId(value)
    }
}

pub enum Component {
    Constant(f32),
    Function(Box<dyn ImplicitFunction>),
    Operation(Box<dyn ImplicitOperation>),
}

impl Component {
    pub fn compute(&self, x: f32, y: f32, z: f32, values: &Vec<f32>) -> f32 {
        match self {
            Component::Constant(value) => *value,
            Component::Function(function) => function.eval(x, y, z),
            Component::Operation(operation) => {
                operation.eval(&Self::get_input_data(&operation.get_inputs(), values))
            }
        }
    }

    pub fn get_input_data(inputs: &[ComponentId], values: &[f32]) -> Vec<f32> {
        inputs.iter().map(|&i| values[i.0]).collect()
    }
}

pub trait ImplicitFunction: Sync + Send {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32;
}

pub trait ImplicitOperation: Sync + Send {
    fn eval(&self, inputs: &Vec<f32>) -> f32;

    fn get_inputs(&self) -> &Vec<ComponentId>;
}
