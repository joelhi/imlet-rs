use std::fmt::Debug;

use imlet_engine::{
    algorithms::marching_cubes::generate_iso_surface,
    types::{
        computation::{ImplicitModel, ScalarField},
        geometry::{BoundingBox, Line, Mesh},
    },
};
use num_traits::Float;

use crate::material::Material;

pub struct Scene<T: Float + Debug + Send + Sync> {
    meshes: Vec<Mesh<T>>,
    lines: Vec<Line<T>>,
    settings: SceneSettings,
}

impl<T: Float + Debug + Send + Sync> Scene<T> {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            lines: Vec::new(),
            settings: SceneSettings::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh<T>) {
        self.meshes.push(mesh);
    }

    pub fn add_lines(&mut self, lines: &[Line<T>]) {
        self.lines.extend(lines);
    }

    pub fn clear(&mut self) {
        self.meshes.clear();
        self.lines.clear();
    }

    pub fn meshes(&self) -> &[Mesh<T>] {
        &self.meshes
    }

    pub fn lines(&self) -> &[Line<T>] {
        &self.lines
    }

    pub fn settings(&self) -> &SceneSettings {
        &self.settings
    }
}

pub struct SceneSettings {
    pub mesh_material: Material,
    pub show_bounds: bool,
    pub show_edges: bool,
}

impl SceneSettings {
    pub fn new() -> Self {
        Self {
            mesh_material: Material::Normal,
            show_bounds: true,
            show_edges: true,
        }
    }
}

pub struct ModelData<T: Float + Debug + Send + Sync> {
    model: ImplicitModel<T>,
    bounds: BoundingBox<T>,
    output: String,
    data: Option<ScalarField<T>>,
}

impl<T: Float + Debug + Send + Sync> ModelData<T> {
    pub fn new(model: ImplicitModel<T>, bounds: BoundingBox<T>, output: &str) -> Self {
        Self {
            model: model,
            bounds: bounds,
            output: output.to_string(),
            data: None,
        }
    }

    pub fn bounds(&self) -> &BoundingBox<T> {
        &self.bounds
    }

    pub fn data(&self) -> &Option<ScalarField<T>> {
        &self.data
    }

    pub fn smooth(&mut self, iterations: u32, factor: T) {
        match &mut self.data {
            Some(values) => values.smooth(factor, iterations),
            None => log::info!("No smoothing computed as field is not computed."),
        }
    }

    pub fn compute(&mut self, cell_size: T) {
        self.data = Some(
            self.model
                .generate_field(&self.output, &self.bounds, cell_size),
        );
    }

    pub fn generate_mesh(&mut self) -> Option<Mesh<T>> {
        match self.data() {
            Some(data) => {
                let triangles = generate_iso_surface(data, T::zero());
                Some(Mesh::from_triangles(&triangles))
            }
            None => {
                log::info!("No mesh generated as field is not computed.");
                None
            }
        }
    }
}
