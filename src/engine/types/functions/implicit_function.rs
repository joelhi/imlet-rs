pub trait ImplicitFunction {
    fn eval(&self, params: &[f32], x: f32, y: f32, z: f32) -> f32;

    fn num_params(&self)->usize;
}