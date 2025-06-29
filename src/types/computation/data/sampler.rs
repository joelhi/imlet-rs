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

/// Trait for sampling implicit models into discrete fields and extracting iso-surfaces.
///
/// This trait defines the core functionality for converting implicit models into
/// discretized fields and extracting meshes at specific iso-values.
pub trait Sampler<T, F> {
    /// Sample a field with a certain resolution from one of the model components.
    ///
    /// # Arguments
    ///
    /// * `cell_size` - The size of each cell in the discretized field.
    /// * `component_tag` - The tag of the model component to evaluate.
    fn sample_field(&mut self, cell_size: T, component_tag: &str) -> Result<&F, ModelError>;

    /// Access the field data if present.
    ///
    /// Returns [`None`] if no field has been sampled yet.
    fn field(&self) -> Option<&F>;

    /// Extract an iso-surface for a certain iso value.
    ///
    /// # Arguments
    ///
    /// * `iso_val` - The iso-value at which to extract the surface.
    ///
    /// # Returns
    ///
    /// A mesh representing the iso-surface, or an error if the field hasn't been computed.
    fn iso_surface(&self, iso_val: T) -> Result<Mesh<T>, ModelError>;
}

/// A sampler that uses sparse field representation for memory-efficient sampling.
///
/// This sampler is particularly useful for models where the interesting features
/// are concentrated in specific regions, as it only stores data in active regions.
///
/// # Example
///
/// ```rust
/// use imlet::types::computation::data::{
///     sampler::{Sampler, SparseSampler},
///     BlockSize, SamplingMode, SparseFieldConfig,
/// };
/// use imlet::types::geometry::BoundingBox;
///
/// // Configure the sparse field parameters
/// let config = SparseFieldConfig {
///     internal_size: BlockSize::Size64,
///     leaf_size: BlockSize::Size4,
///     sampling_mode: SamplingMode::CENTRE,
/// };
///
/// // Create and configure the sampler
/// let mut sampler = SparseSampler::builder()
///     .with_bounds(bounds)
///     .with_model(model)
///     .with_sparse_config(config)
///     .build()
///     .expect("Failed to build sampler");
///
/// // Sample the field
/// sampler.sample_field(cell_size, &model_tag)
///     .expect("Sampling failed");
///
/// // Extract the iso-surface
/// let mesh = sampler.iso_surface(0.0)
///     .expect("Failed to extract surface");
/// ```
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

/// A builder for configuring and creating sparse samplers.
///
/// This builder provides a fluent interface for setting up a [`SparseSampler`] with
/// all necessary parameters.
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

impl<T> Default for SparseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SparseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            max_val: Some(SparseSampler::default_max()),
            min_val: Some(SparseSampler::default_min()),
            model: None,
            bounds: None,
            sparse_config: None,
        }
    }

    /// Sets the maximum value threshold for the field.
    pub fn with_max_val(mut self, max_val: T) -> Self {
        self.max_val = Some(max_val);
        self
    }

    /// Sets the minimum value threshold for the field.
    pub fn with_delta_neg(mut self, min_val: T) -> Self {
        self.min_val = Some(min_val);
        self
    }

    /// Sets the implicit model to sample.
    pub fn with_model(mut self, model: ImplicitModel<T>) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the bounding box defining the sampling region.
    pub fn with_bounds(mut self, bounds: BoundingBox<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Sets the configuration for the sparse field structure.
    pub fn with_sparse_config(mut self, config: SparseFieldConfig) -> Self {
        self.sparse_config = Some(config);
        self
    }

    /// Builds the sampler with the configured parameters.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the configured sampler or an error message if required parameters are missing.
    pub fn build(self) -> Result<SparseSampler<T>, ModelError> {
        let model = self.model.ok_or(ModelError::Custom("model is required".to_owned()))?;
        let bounds = self.bounds.ok_or(ModelError::Custom("bounds is required".to_owned()))?;
        let sparse_config = self.sparse_config.ok_or(ModelError::Custom("config is required".to_owned()))?;

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
        sparse_field.sample_from_graph(&comp_graph, self.min_val, self.max_val)?;
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

/// A sampler that uses dense field representation for uniform sampling.
///
/// This sampler stores values at every point in the sampling grid, making it suitable
/// for models where the interesting features are distributed throughout the volume.
///
/// # Example
///
/// ```rust
/// use imlet::types::computation::data::sampler::{DenseSampler, Sampler};
/// use imlet::types::geometry::BoundingBox;
///
/// // Create and configure the sampler
/// let mut sampler = DenseSampler::builder()
///     .with_bounds(bounds)
///     .with_model(model)
///     .with_smoothing_iter(2)       // Optional: Apply smoothing
///     .with_smoothing_factor(0.5)   // Optional: Set smoothing strength
///     .with_padding(true)           // Optional: Add padding
///     .build()
///     .expect("Failed to build sampler");
///
/// // Sample the field
/// sampler.sample_field(cell_size, &model_tag)
///     .expect("Sampling failed");
///
/// // Extract the iso-surface
/// let mesh = sampler.iso_surface(0.0)
///     .expect("Failed to extract surface");
/// ```
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

/// A builder for configuring and creating dense samplers.
///
/// This builder provides a fluent interface for setting up a [`DenseSampler`] with
/// all necessary parameters.
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

impl<T> Default for DenseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DenseSamplerBuilder<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            model: None,
            bounds: None,
            smoothing_iter: 0,
            smoothing_factor: T::from(0.5).unwrap(),
            padding: false,
        }
    }

    /// Sets the implicit model to sample.
    pub fn with_model(mut self, model: ImplicitModel<T>) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the bounding box defining the sampling region.
    pub fn with_bounds(mut self, bounds: BoundingBox<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Sets the number of smoothing iterations to apply to the field.
    pub fn with_smoothing_iter(mut self, smoothing_iter: usize) -> Self {
        self.smoothing_iter = smoothing_iter;
        self
    }

    /// Sets the smoothing factor controlling the strength of smoothing.
    pub fn with_smoothing_factor(mut self, smoothing_factor: T) -> Self {
        self.smoothing_factor = smoothing_factor;
        self
    }

    /// Enables or disables padding at the field boundaries.
    pub fn with_padding(mut self, padding: bool) -> Self {
        self.padding = padding;
        self
    }

    /// Builds the sampler with the configured parameters.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the configured sampler or an error message if required parameters are missing.
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

        let graph = self.model.compile(component_tag)?;
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
