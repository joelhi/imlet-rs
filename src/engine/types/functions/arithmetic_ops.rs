use super::{constant::Constant, ImplicitFunction};

#[derive(Debug, Clone, Copy)]
pub struct Multiply<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Multiply<F, G>{
    pub fn new(f: F, g: G)->Self{
        Multiply{
            f,
            g
        }

    }
}

impl<F: ImplicitFunction> Multiply<F, Constant> {
    pub fn with_constant(f: F, value: f32) -> Self {
        Multiply {
            f,
            g: Constant::new(value),
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Multiply<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) * self.g.eval(x, y, z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Subtract<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Subtract<F, G>{
    pub fn new(f: F, g: G)->Self{
        Subtract{
            f,
            g
        }

    }
}

impl<F: ImplicitFunction> Subtract<F, Constant> {
    pub fn with_constant(f: F, value: f32) -> Self {
        Subtract {
            f,
            g: Constant::new(value),
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Subtract<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) - self.g.eval(x, y, z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Add<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Add<F, G>{
    pub fn new(f: F, g: G)->Self{
        Add{
            f,
            g
        }

    }
}

impl<F: ImplicitFunction> Add<F, Constant> {
    pub fn with_constant(f: F, value: f32) -> Self {
        Add {
            f,
            g: Constant::new(value),
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Add<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) + self.g.eval(x, y, z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Divide<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Divide<F, G>{
    pub fn new(f: F, g: G)->Self{
        Divide{
            f,
            g
        }

    }
}

impl<F: ImplicitFunction> Divide<F, Constant> {
    pub fn with_constant(f: F, value: f32) -> Self {
        Divide {
            f,
            g: Constant::new(value),
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Divide<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) / self.g.eval(x, y, z)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Max<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Max<F, G>{
    pub fn new(f: F, g: G)->Self{
        Max{
            f,
            g
        }

    }
}

impl<F: ImplicitFunction> Max<F, Constant> {
    pub fn with_constant(f: F, value: f32) -> Self {
        Max {
            f,
            g: Constant::new(value),
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Max<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) / self.g.eval(x, y, z)
    }
}
