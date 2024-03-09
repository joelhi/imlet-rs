use crate::marching_cubes::types::core::XYZ;
use super::base::ImplicitFunction;

pub struct GyroidFunction{

}

impl ImplicitFunction for GyroidFunction{
    fn eval(&self, pt:XYZ)->f32{
            0.0
    }
}

pub struct DistanceFunction{
    pub source:XYZ
}

impl ImplicitFunction for DistanceFunction{
    fn eval(&self, pt:XYZ)->f32{
            self.source.distance_to(pt)
    }
}