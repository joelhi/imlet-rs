use std::time::Instant;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::engine::types::{DenseFieldF32, XYZ};

use super::component::{Component, ComponentId, ImplicitFunction, ImplicitOperation};

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

    fn compute(&self, x: f32, y: f32, z: f32, output: ComponentId) -> f32 {
        let mut values: Vec<f32> = vec![0.0; self.components.len()];
        for (index, component) in self.components.iter().enumerate() {
            values[index] = component.compute(x, y, z, &values)
        }

        values[output.value()]
    }

    pub fn evaluate(
        &self,
        origin: XYZ,
        size_x: usize,
        size_y: usize,
        size_z: usize,
        cell_size: f32,
        output: ComponentId,
    ) -> DenseFieldF32 {
        let before = Instant::now();
        let mut data: Vec<f32> = vec![0.0; size_x * size_y * size_z];
        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let (i, j, k) = DenseFieldF32::index3d_from_index1d(index, size_x, size_y, size_z);
            *value = self.compute(
                cell_size * i as f32,
                cell_size * j as f32,
                cell_size * k as f32,
                output,
            );
        });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            size_x * size_y * size_z,
            before.elapsed()
        );

        DenseFieldF32::new(origin, cell_size, size_x, size_y, size_z, data)
    }
}
