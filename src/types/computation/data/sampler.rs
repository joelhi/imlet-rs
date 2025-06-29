use std::rc::Rc;

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
/// # use imlet::types::computation::{
/// #    data::{sampler::{Sampler, SparseSampler}, BlockSize, SamplingMode, SparseFieldConfig},
/// #    model::ImplicitModel,
/// # };
/// # use imlet::types::geometry::BoundingBox;
/// #
/// # let mut model = ImplicitModel::new();
/// # let model_tag = model.add_constant("tag", 1.0).unwrap();
/// # let bounds = BoundingBox::zero();
/// # let cell_size = 1.0;
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

pub struct SparseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    min_val: T,
    max_val: T,
    model: Rc<ImplicitModel<T>>,
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
        model: Rc<ImplicitModel<T>>,
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
    model: Option<Rc<ImplicitModel<T>>>,
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
    pub fn with_model(mut self, model: Rc<ImplicitModel<T>>) -> Self {
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
        let model = self
            .model
            .ok_or(ModelError::Custom("model is required".to_owned()))?;
        let bounds = self
            .bounds
            .ok_or(ModelError::Custom("bounds is required".to_owned()))?;
        let sparse_config = self
            .sparse_config
            .ok_or(ModelError::Custom("config is required".to_owned()))?;

        let min_val = self.min_val.unwrap_or(SparseSampler::default_min());
        let max_val = self.max_val.unwrap_or(SparseSampler::default_max());
        let field = SparseSampler::new(min_val, max_val, model, bounds, sparse_config);
        Ok(field)
    }
}

impl<T> SparseSampler<T>
where
    T: Float + Send + Sync + Serialize + Pi,
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
/// # use imlet::types::computation::{
/// #    data::sampler::{DenseSampler, Sampler},
/// #    model::ImplicitModel,
/// # };
/// # use imlet::types::geometry::BoundingBox;
/// #
/// # let mut model = ImplicitModel::new();
/// # let model_tag = model.add_constant("tag", 1.0).unwrap();
/// # let bounds = BoundingBox::zero();
/// # let cell_size = 1.0;
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
pub struct DenseSampler<T>
where
    T: Float + Send + Sync + Serialize + 'static + Pi,
{
    model: Rc<ImplicitModel<T>>,
    bounds: BoundingBox<T>,
    smoothing_iter: u32,
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
    model: Option<Rc<ImplicitModel<T>>>,
    bounds: Option<BoundingBox<T>>,
    smoothing_iter: u32,
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
    T: Float + Send + Sync + Serialize + Pi,
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
    pub fn with_model(mut self, model: Rc<ImplicitModel<T>>) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the bounding box defining the sampling region.
    pub fn with_bounds(mut self, bounds: BoundingBox<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Sets the number of smoothing iterations to apply to the field.
    pub fn with_smoothing_iter(mut self, smoothing_iter: u32) -> Self {
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

impl<T: Float + Send + Sync + Serialize + Pi> Sampler<T, DenseField<T>> for DenseSampler<T> {
    fn sample_field(
        &mut self,
        cell_size: T,
        component_tag: &str,
    ) -> Result<&DenseField<T>, ModelError> {
        let mut field = DenseField::from_bounds(&self.bounds, cell_size);
        let graph = self.model.compile(component_tag)?;
        field.sample_from_graph(&graph);

        // Apply padding if specified.
        if self.padding {
            field.padding(T::zero());
        }

        // Apply smoothing if specified.
        if self.smoothing_iter > 0 {
            field.smooth_par(self.smoothing_factor, self.smoothing_iter);
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        computation::data::{BlockSize, SamplingMode},
        geometry::Vec3,
    };

    fn create_test_model() -> ImplicitModel<f32> {
        let mut model = ImplicitModel::new();
        model.add_constant("constant", 1.0).unwrap();
        model
    }

    fn create_test_bounds() -> BoundingBox<f32> {
        BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0))
    }

    #[test]
    fn test_dense_sampler_builder() {
        let bounds = create_test_bounds();
        let model_ptr = Rc::new(create_test_model());
        // Test basic builder
        let sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .build();
        assert!(sampler.is_ok());

        // Test builder with all options
        let sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_smoothing_iter(2)
            .with_smoothing_factor(0.5)
            .with_padding(true)
            .build();
        assert!(sampler.is_ok());

        // Test builder fails without model
        let sampler = DenseSampler::builder().with_bounds(bounds).build();
        assert!(sampler.is_err());

        // Test builder fails without bounds
        let sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .build();
        assert!(sampler.is_err());
    }

    #[test]
    fn test_dense_sampler_field_operations() {
        let bounds = create_test_bounds();
        let model_ptr = Rc::new(create_test_model());
        let mut sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .build()
            .unwrap();

        // Test field is initially None
        assert!(sampler.field().is_none());

        // Test sampling
        let field = sampler.sample_field(1.0, "constant");
        assert!(field.is_ok());
        assert!(sampler.field().is_some());

        // Test iso-surface extraction
        let mesh = sampler.iso_surface(0.0);
        assert!(mesh.is_ok());
    }

    #[test]
    fn test_sparse_sampler_builder() {
        let bounds = create_test_bounds();
        let model = create_test_model();
        let config = SparseFieldConfig {
            internal_size: BlockSize::Size64,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
        };

        let model_ptr = Rc::new(model);
        // Test basic builder
        let sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config)
            .build();
        assert!(sampler.is_ok());

        // Test builder with custom thresholds
        let sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config)
            .with_max_val(0.5)
            .with_delta_neg(-0.5)
            .build();
        assert!(sampler.is_ok());

        // Test builder fails without model
        let sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_sparse_config(config)
            .build();
        assert!(sampler.is_err());

        // Test builder fails without bounds
        let sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_sparse_config(config)
            .build();
        assert!(sampler.is_err());

        // Test builder fails without config
        let sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .build();
        assert!(sampler.is_err());
    }

    #[test]
    fn test_sparse_sampler_field_operations() {
        let bounds = create_test_bounds();
        let model = create_test_model();
        let config = SparseFieldConfig {
            internal_size: BlockSize::Size64,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
        };

        let model_ptr = Rc::new(model);
        let mut sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config)
            .build()
            .unwrap();

        // Test field is initially None
        assert!(sampler.field().is_none());

        // Test sampling
        let field = sampler.sample_field(1.0, "constant");
        assert!(field.is_ok());
        assert!(sampler.field().is_some());

        // Test iso-surface extraction
        let mesh = sampler.iso_surface(0.0);
        assert!(mesh.is_ok());
    }

    #[test]
    fn test_sparse_sampler_iso_value_bounds() {
        let bounds = create_test_bounds();
        let model = create_test_model();
        let config = SparseFieldConfig {
            internal_size: BlockSize::Size64,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
        };

        let model_ptr = Rc::new(model);
        let mut sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config)
            .with_max_val(0.5)
            .with_delta_neg(-0.5)
            .build()
            .unwrap();

        sampler.sample_field(1.0, "constant").unwrap();

        // Test iso-surface extraction within bounds succeeds
        assert!(sampler.iso_surface(0.0).is_ok());

        // Test iso-surface extraction outside bounds fails
        assert!(sampler.iso_surface(1.0).is_err());
        assert!(sampler.iso_surface(-1.0).is_err());
    }

    #[test]
    fn test_dense_sampler_with_smoothing() {
        let model_ptr = Rc::new(create_test_model());
        let bounds = create_test_bounds();

        let mut sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_smoothing_iter(3)
            .with_smoothing_factor(0.1)
            .build()
            .expect("Failed to build sampler");

        sampler
            .sample_field(1.0, "constant")
            .expect("Failed to sample field");

        let field = sampler.field().unwrap();
        assert!(!field.data().is_empty());
    }

    #[test]
    fn test_dense_sampler_with_padding() {
        let model_ptr = Rc::new(create_test_model());
        let bounds = create_test_bounds();

        let mut sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_padding(true)
            .build()
            .expect("Failed to build sampler");

        sampler
            .sample_field(1.0, "constant")
            .expect("Failed to sample field");

        let field = sampler.field().unwrap();
        assert!(!field.data().is_empty());
    }

    #[test]
    fn test_sparse_sampler_different_block_sizes() {
        let bounds = create_test_bounds();
        let model_ptr = Rc::new(create_test_model());

        // Test with small blocks
        let config_small = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size2,
            sampling_mode: SamplingMode::CENTRE,
        };

        let mut sampler_small = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config_small)
            .build()
            .expect("Failed to build sampler");

        sampler_small
            .sample_field(1.0, "constant")
            .expect("Failed to sample field with small blocks");

        let config_large = SparseFieldConfig {
            internal_size: BlockSize::Size32,
            leaf_size: BlockSize::Size8,
            sampling_mode: SamplingMode::CENTRE,
        };

        let mut sampler_large = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config_large)
            .build()
            .expect("Failed to build sampler");

        sampler_large
            .sample_field(1.0, "constant")
            .expect("Failed to sample field with large blocks");

        assert!(sampler_small.field().is_some());
        assert!(sampler_large.field().is_some());
    }

    #[test]
    fn test_sparse_sampler_sampling_modes() {
        let bounds = create_test_bounds();
        let model_ptr = Rc::new(create_test_model());

        // Test CENTRE mode
        let config_centre = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
        };

        let mut sampler_centre = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config_centre)
            .build()
            .expect("Failed to build sampler");

        sampler_centre
            .sample_field(1.0, "constant")
            .expect("Failed to sample field with CENTRE mode");

        // Test CORNERS mode
        let config_corners = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CORNERS,
        };

        let mut sampler_corners = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config_corners)
            .build()
            .expect("Failed to build sampler");

        sampler_corners
            .sample_field(1.0, "constant")
            .expect("Failed to sample field with CORNERS mode");

        // Both samplers should produce valid fields
        assert!(sampler_centre.field().is_some());
        assert!(sampler_corners.field().is_some());
    }

    #[test]
    fn test_sampler_invalid_component() {
        let bounds = create_test_bounds();
        let model_ptr = Rc::new(create_test_model());

        let mut dense_sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .build()
            .expect("Failed to build dense sampler");

        let dense_result = dense_sampler.sample_field(1.0, "nonexistent_component");
        assert!(dense_result.is_err());

        let config = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
        };

        let mut sparse_sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config)
            .build()
            .expect("Failed to build sparse sampler");

        let sparse_result = sparse_sampler.sample_field(1.0, "nonexistent_component");
        assert!(sparse_result.is_err());
    }

    #[test]
    fn test_sampler_iso_surface_without_field() {
        let bounds = create_test_bounds();
        let model_ptr = Rc::new(create_test_model());

        let dense_sampler = DenseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .build()
            .expect("Failed to build dense sampler");

        let dense_result = dense_sampler.iso_surface(0.0);
        assert!(dense_result.is_err());

        let config = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
        };

        let sparse_sampler = SparseSampler::builder()
            .with_model(model_ptr.clone())
            .with_bounds(bounds)
            .with_sparse_config(config)
            .build()
            .expect("Failed to build sparse sampler");

        let sparse_result = sparse_sampler.iso_surface(0.0);
        assert!(sparse_result.is_err());
    }
}
