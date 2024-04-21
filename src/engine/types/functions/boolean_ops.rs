use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct Union<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Union<F, G>{
    pub fn new(f: F, g: G)->Self{
        Union{
            f,
            g
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Union<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z).min(self.g.eval(x, y, z))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SmoothUnion<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> SmoothUnion<F, G>{
    pub fn new(f: F, g: G)->Self{
        SmoothUnion{
            f,
            g
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for SmoothUnion<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let f_val = self.f.eval(x, y, z);
        let g_val = self.g.eval(x, y, z);

        2.0 / 3.0 * (f_val + g_val - (f_val.powi(2) + g_val.powi(2) - f_val * g_val).sqrt())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Intersection<F, G>{
    pub fn new(f: F, g: G)->Self{
        Intersection{
            f,
            g
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Intersection<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z).max(self.g.eval(x, y, z))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Difference<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> Difference<F, G>{
    pub fn new(f: F, g: G)->Self{
        Difference{
            f,
            g
        }
    }
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for Difference<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z).max(-self.g.eval(x, y, z))
    }
}