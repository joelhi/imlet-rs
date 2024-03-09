use crate::XYZ;

pub trait ImplicitFunction {
    fn eval(&self, pt:XYZ)->f32;
}