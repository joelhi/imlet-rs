use std::time::Instant;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::engine::types::geometry::{BoundingBox, Vec3i};

use super::{
    component::{Component, ComponentId, ImplicitFunction, ImplicitOperation},
    DenseFieldF32,
};

pub struct Model {
    components: Vec<Component>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            components: Vec::new(),
        }
    }

    pub fn add_function<T: ImplicitFunction + 'static>(&mut self, function: T) -> ComponentId {
        self.components
            .push(Component::Function(Box::new(function)));
        (self.components.len() - 1).into()
    }

    pub fn add_operation<T: ImplicitOperation + 'static>(&mut self, operation: T) -> ComponentId {
        self.components
            .push(Component::Operation(Box::new(operation)));
        (self.components.len() - 1).into()
    }

    pub fn add_constant(&mut self, value: f32) -> ComponentId {
        self.components.push(Component::Constant(value));
        (self.components.len() - 1).into()
    }
    
    fn evaluate_at_coord(&self, x: f32, y: f32, z: f32, output: ComponentId) -> f32 {
        let mut values = vec![0.0; self.components.len()];
            for (index, component) in self.components.iter().enumerate() {
                values[index] = component.compute(x, y, z, &values);
                if index == output.value(){
                    break;
                }
            }
            values[output.value()]
        }

    pub fn evaluate(
        &self,
        bounds: BoundingBox,
        cell_size: f32,
        output: ComponentId,
    ) -> DenseFieldF32 {
        let before = Instant::now();
        let n = Self::get_cell_count(&bounds, cell_size);
        let mut data: Vec<f32> = vec![0.0; n.x * n.y * n.z];
        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let (i, j, k) = DenseFieldF32::index3d_from_index1d(index, n.x, n.y, n.z);
            *value = self.evaluate_at_coord(
                cell_size * i as f32,
                cell_size * j as f32,
                cell_size * k as f32,
                output,
            );
        });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            n.x * n.y * n.z,
            before.elapsed()
        );

        DenseFieldF32::new(bounds.min, cell_size, n, data)
    }

    fn get_cell_count(bounds: &BoundingBox, cell_size: f32) -> Vec3i {
        let (x_dim, y_dim, z_dim) = bounds.get_dimensions();
        Vec3i::new(
            (x_dim / cell_size).floor() as usize,
            (y_dim / cell_size).floor() as usize,
            (z_dim / cell_size).floor() as usize,
        )
    }
}
