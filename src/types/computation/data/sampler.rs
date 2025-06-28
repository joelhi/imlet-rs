use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::{
    algorithms::{self, marching_cubes},
    types::{
        computation::{
            data::{DenseField, SparseField, SparseFieldConfig},
            model::ImplicitModel,
            ModelError,
        },
        geometry::{BoundingBox, Mesh},
    },
    utils::math_helper::Pi,
};

pub trait Sampler<T, F> {
    /// Sample a field at a certain iso-value.
    fn sample_field(&mut self, cell_size: T, component_tag: &str) -> Result<&F, ModelError>;

    /// Access the field data if present.
    fn field(&self) -> Option<&F>;

    /// Extract an iso-surface for a certain iso value.
    /// Requires the field to be computed.
    fn iso_surface(&self, iso_val: T) -> Result<Mesh<T>, ModelError>;
}

#[derive(Serialize, Deserialize)]
pub struct SparseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    min_val: T,
    max_val: T,
    model: ImplicitModel<T>,
    bounds: BoundingBox<T>,
    sparse_config: SparseFieldConfig,
    field: Option<SparseField<T>>,
}

impl<T> SparseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    fn new(
        min_val: T,
        max_val: T,
        model: ImplicitModel<T>,
        bounds: BoundingBox<T>,
        sparse_config: SparseFieldConfig,
    ) -> Self {
        Self {
            min_val,
            max_val,
            model,
            bounds,
            sparse_config,
            field: None,
        }
    }

    fn default_max() -> T {
        T::from(0.1).unwrap()
    }

    fn default_min() -> T {
        T::from(-0.1).unwrap()
    }
}

pub struct SparseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    max_val: Option<T>,
    min_val: Option<T>,
    model: Option<ImplicitModel<T>>,
    bounds: Option<BoundingBox<T>>,
    sparse_config: Option<SparseFieldConfig>,
}

impl<T> SparseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    pub fn new() -> Self {
        Self {
            max_val: Some(SparseSampler::default_max()),
            min_val: Some(SparseSampler::default_min()),
            model: None,
            bounds: None,
            sparse_config: None,
        }
    }

    pub fn with_max_val(mut self, max_val: T) -> Self {
        self.max_val = Some(max_val);
        self
    }

    pub fn with_delta_neg(mut self, min_val: T) -> Self {
        self.min_val = Some(min_val);
        self
    }

    pub fn with_model(mut self, model: ImplicitModel<T>) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_bounds(mut self, bounds: BoundingBox<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    pub fn with_sparse_config(mut self, config: SparseFieldConfig) -> Self {
        self.sparse_config = Some(config);
        self
    }

    pub fn build(self) -> Result<SparseSampler<T>, &'static str> {
        let model = self.model.ok_or("model is required")?;
        let bounds = self.bounds.ok_or("bounds is required")?;
        let sparse_config = self.sparse_config.ok_or("sparse_config is required")?;

        let min_val = self.min_val.unwrap_or(SparseSampler::default_min());
        let max_val = self.max_val.unwrap_or(SparseSampler::default_max());
        let field = SparseSampler::new(min_val, max_val, model, bounds, sparse_config);
        Ok(field)
    }
}

impl<T> SparseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    pub fn builder() -> SparseSamplerBuilder<T> {
        SparseSamplerBuilder::new()
    }
}

impl<T: Float + Send + Sync + Serialize + 'static + Pi + Serialize + Default>
    Sampler<T, SparseField<T>> for SparseSampler<T>
{
    fn sample_field(
        &mut self,
        cell_size: T,
        component_tag: &str,
    ) -> Result<&SparseField<T>, ModelError> {
        let mut sparse_field = SparseField::new(self.sparse_config);
        sparse_field.init_bounds(&self.bounds, cell_size);
        let comp_graph = self.model.compile(component_tag)?;
        sparse_field.sample_from_graph(
            &comp_graph,
            self.min_val,
            self.max_val,
            self.sparse_config.sampling_mode,
        )?;
        self.field = Some(sparse_field);
        Ok(self.field.as_ref().unwrap())
    }

    fn iso_surface(&self, iso_val: T) -> Result<Mesh<T>, ModelError> {
        if iso_val > self.max_val || iso_val < self.min_val {
            return Err(ModelError::Custom(
                format!(
                    "Iso value {} is out of bounds for the current field. Min: {}, Max: {}",
                    iso_val.to_f32().unwrap(),
                    self.min_val.to_f32().unwrap(),
                    self.max_val.to_f32().unwrap()
                )
                .to_owned(),
            ));
        }
        if let Some(field_ref) = &self.field {
            let tris = marching_cubes::generate_iso_surface(field_ref, iso_val);
            return Ok(Mesh::from_triangles(&tris, true));
        }

        Err(ModelError::Custom("Field not computed".to_owned()))
    }

    fn field(&self) -> Option<&SparseField<T>> {
        self.field.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DenseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    model: ImplicitModel<T>,
    bounds: BoundingBox<T>,
    smoothing_iter: usize,
    smoothing_factor: T,
    padding: bool,
    dense_field: Option<DenseField<T>>,
}

pub struct DenseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    model: Option<ImplicitModel<T>>,
    bounds: Option<BoundingBox<T>>,
    smoothing_iter: usize,
    smoothing_factor: T,
    padding: bool,
}

impl<T> DenseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    pub fn new() -> Self {
        Self {
            model: None,
            bounds: None,
            smoothing_iter: 0,
            smoothing_factor: T::from(0.5).unwrap(),
            padding: false,
        }
    }

    pub fn with_model(mut self, model: ImplicitModel<T>) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_bounds(mut self, bounds: BoundingBox<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    pub fn with_smoothing_iter(mut self, smoothing_iter: usize) -> Self {
        self.smoothing_iter = smoothing_iter;
        self
    }

    pub fn with_smoothing_factor(mut self, smoothing_factor: T) -> Self {
        self.smoothing_factor = smoothing_factor;
        self
    }

    pub fn with_padding(mut self, padding: bool) -> Self {
        self.padding = padding;
        self
    }

    pub fn build(self) -> Result<DenseSampler<T>, &'static str> {
        let model = self.model.ok_or("model is required")?;
        let bounds = self.bounds.ok_or("bounds is required")?;

        Ok(DenseSampler {
            model,
            bounds,
            smoothing_iter: self.smoothing_iter,
            smoothing_factor: self.smoothing_factor,
            padding: self.padding,
            dense_field: None,
        })
    }
}

impl<T> DenseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    pub fn builder() -> DenseSamplerBuilder<T> {
        DenseSamplerBuilder::new()
    }
}

impl<T: Float + Send + Sync + Serialize + 'static + Pi> Sampler<T, DenseField<T>>
    for DenseSampler<T>
{
    fn sample_field(
        &mut self,
        cell_size: T,
        component_tag: &str,
    ) -> Result<&DenseField<T>, ModelError> {
        let mut field = DenseField::from_bounds(&self.bounds, cell_size);

        let graph = self.model.compile(&component_tag)?;
        field.sample_from_graph(&graph);
        self.dense_field = Some(field);
        Ok(self.dense_field.as_ref().unwrap())
    }

    fn iso_surface(&self, iso_val: T) -> Result<Mesh<T>, ModelError> {
        if let Some(field_ref) = &self.dense_field {
            let tris = algorithms::marching_cubes::generate_iso_surface(field_ref, iso_val);
            return Ok(Mesh::from_triangles(&tris, true));
        }

        Err(ModelError::Custom("Field not computed".to_owned()))
    }

    fn field(&self) -> Option<&DenseField<T>> {
        self.dense_field.as_ref()
    }
}
