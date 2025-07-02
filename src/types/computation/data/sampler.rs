use num_traits::Float;
use serde::Serialize;

use crate::{
    algorithms::{self, marching_cubes},
    types::{
        computation::{
            data::{DenseField, SparseField, SparseFieldConfig}, model::ImplicitModel, traits::ModelFloat, ModelError
        },
        geometry::{BoundingBox, Mesh},
    },
    utils::math_helper::Pi,
};

/// Trait for sampling implicit models into discrete fields and extracting iso-surfaces.
///
/// This trait defines the core functionality for converting implicit models into
/// discretized fields and extracting meshes at specific iso-values.
pub trait Sampler<T: ModelFloat, F> {
    /// Sample a field with a certain resolution for the default component given by [`ImplicitModel::get_default_output`]
    ///
    /// # Arguments
    ///
    /// * `cell_size` - The size of each cell in the discretized field.
    /// * `component_tag` - The tag of the model component to evaluate.
    fn sample_field(&mut self, model: &ImplicitModel<T>) -> Result<&F, ModelError>;

    /// Sample a field with a certain resolution from one of the model components.
    ///
    /// # Arguments
    ///
    /// * `cell_size` - The size of each cell in the discretized field.
    /// * `component_tag` - The tag of the model component to evaluate.
    fn sample_field_for_component(
        &mut self,
        model: &ImplicitModel<T>,
        component_tag: &str,
    ) -> Result<&F, ModelError>;

    /// Access the field data if present.
    ///
    /// Returns the current field.
    fn field(&self) -> &F;

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
///
/// // Configure the sparse field parameters
/// let config = SparseFieldConfig {
///     internal_size: BlockSize::Size64,
///     leaf_size: BlockSize::Size4,
///     sampling_mode: SamplingMode::CENTRE,
///     cell_size: 1.0,
/// };
///
/// // Create and configure the sampler
/// let mut sampler = SparseSampler::builder()
///     .with_bounds(bounds)
///     .with_config(config)
///     .build()
///     .expect("Failed to build sampler");
///
/// // Sample the field
/// sampler.sample_field(&model)
///     .expect("Sampling failed");
///
/// // Extract the iso-surface
/// let mesh = sampler.iso_surface(0.0)
///     .expect("Failed to extract surface");
///
/// ```
pub struct SparseSampler<T>
where
    T: ModelFloat + 'static
{
    min_val: T,
    max_val: T,
    field: SparseField<T>,
}

impl<T> SparseSampler<T>
where
    T: ModelFloat + 'static,
{
    fn new(
        min_val: T,
        max_val: T,
        bounds: BoundingBox<T>,
        sparse_config: SparseFieldConfig<T>,
    ) -> Self {
        let mut field = SparseField::new(sparse_config);
        field.init_bounds(&bounds);
        Self {
            min_val,
            max_val,
            field,
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
    T: ModelFloat,
{
    max_val: Option<T>,
    min_val: Option<T>,
    bounds: Option<BoundingBox<T>>,
    sparse_config: Option<SparseFieldConfig<T>>,
}

impl<T> Default for SparseSamplerBuilder<T>
where
    T: ModelFloat + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SparseSamplerBuilder<T>
where
    T: ModelFloat + 'static,
{
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            max_val: Some(SparseSampler::default_max()),
            min_val: Some(SparseSampler::default_min()),
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
    pub fn with_min_val(mut self, min_val: T) -> Self {
        self.min_val = Some(min_val);
        self
    }

    /// Sets the bounding box defining the sampling region.
    pub fn with_bounds(mut self, bounds: BoundingBox<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Sets the configuration for the sparse field structure.
    pub fn with_config(mut self, config: SparseFieldConfig<T>) -> Self {
        self.sparse_config = Some(config);
        self
    }

    /// Builds the sampler with the configured parameters.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the configured sampler or an error message if required parameters are missing.
    pub fn build(self) -> Result<SparseSampler<T>, ModelError> {
        let bounds = self
            .bounds
            .ok_or(ModelError::Custom("bounds is required".to_owned()))?;
        let sparse_config = self
            .sparse_config
            .ok_or(ModelError::Custom("config is required".to_owned()))?;

        let min_val = self.min_val.unwrap_or(SparseSampler::default_min());
        let max_val = self.max_val.unwrap_or(SparseSampler::default_max());
        let field = SparseSampler::new(min_val, max_val, bounds, sparse_config);
        Ok(field)
    }
}

impl<T> SparseSampler<T>
where
    T: ModelFloat,
{
    pub fn builder() -> SparseSamplerBuilder<T> {
        SparseSamplerBuilder::new()
    }
}

impl<T: ModelFloat + 'static + Default>
    Sampler<T, SparseField<T>> for SparseSampler<T>
{
    fn sample_field(&mut self, model: &ImplicitModel<T>) -> Result<&SparseField<T>, ModelError> {
        let component_tag = model
            .get_default_output()
            .ok_or(ModelError::Custom("No default output defined.".to_owned()))?;
        self.sample_field_for_component(model, component_tag)
    }

    fn sample_field_for_component(
        &mut self,
        model: &ImplicitModel<T>,
        component_tag: &str,
    ) -> Result<&SparseField<T>, ModelError> {
        let comp_graph = model.compile(component_tag)?;
        self.field
            .sample_from_graph(&comp_graph, self.min_val, self.max_val)?;
        Ok(&self.field)
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
        let tris = marching_cubes::generate_iso_surface(&self.field, iso_val);
        Ok(Mesh::from_triangles(&tris, true))
    }

    fn field(&self) -> &SparseField<T> {
        &self.field
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
///     .with_cell_size(cell_size)
///     .with_smoothing_iter(2)       // Optional: Apply smoothing
///     .with_smoothing_factor(0.5)   // Optional: Set smoothing strength
///     .with_padding(true)           // Optional: Add padding
///     .build()
///     .expect("Failed to build sampler");
///
/// // Sample the field
/// sampler.sample_field(&model)
///     .expect("Sampling failed");
///
/// // Extract the iso-surface
/// let mesh = sampler.iso_surface(0.0)
///     .expect("Failed to extract surface");
/// ```
pub struct DenseSampler<T>
where
    T: ModelFloat + 'static,
{
    smoothing_iter: u32,
    smoothing_factor: T,
    padding: bool,
    dense_field: DenseField<T>,
}

/// A builder for configuring and creating dense samplers.
///
/// This builder provides a fluent interface for setting up a [`DenseSampler`] with
/// all necessary parameters.
pub struct DenseSamplerBuilder<T>
where
    T: ModelFloat + 'static,
{
    cell_size: Option<T>,
    bounds: Option<BoundingBox<T>>,
    smoothing_iter: u32,
    smoothing_factor: T,
    padding: bool,
}

impl<T> Default for DenseSamplerBuilder<T>
where
    T: ModelFloat + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DenseSamplerBuilder<T>
where
    T: ModelFloat,
{
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            cell_size: None,
            bounds: None,
            smoothing_iter: 0,
            smoothing_factor: T::from(0.5).unwrap(),
            padding: false,
        }
    }

    /// Sets the resolution of the sampling.
    pub fn with_cell_size(mut self, cell_size: T) -> Self {
        self.cell_size = Some(cell_size);
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
        let bounds = self.bounds.ok_or("bounds is required")?;
        let cell_size = self.cell_size.ok_or("cell size is required")?;

        let dense_field = DenseField::from_bounds(&bounds, cell_size);
        Ok(DenseSampler {
            smoothing_iter: self.smoothing_iter,
            smoothing_factor: self.smoothing_factor,
            padding: self.padding,
            dense_field,
        })
    }
}

impl<T> DenseSampler<T>
where
    T: ModelFloat + 'static,
{
    pub fn builder() -> DenseSamplerBuilder<T> {
        DenseSamplerBuilder::new()
    }
}

impl<T: ModelFloat> Sampler<T, DenseField<T>> for DenseSampler<T> {
    fn sample_field(&mut self, model: &ImplicitModel<T>) -> Result<&DenseField<T>, ModelError> {
        let component_tag = model
            .get_default_output()
            .ok_or(ModelError::Custom("No default output defined.".to_owned()))?;
        self.sample_field_for_component(model, component_tag)
    }

    fn sample_field_for_component(
        &mut self,
        model: &ImplicitModel<T>,
        component_tag: &str,
    ) -> Result<&DenseField<T>, ModelError> {
        let graph = model.compile(component_tag)?;
        self.dense_field.sample_from_graph(&graph);

        // Apply padding if specified.
        if self.padding {
            self.dense_field.padding(T::zero());
        }

        // Apply smoothing if specified.
        if self.smoothing_iter > 0 {
            self.dense_field
                .smooth_par(self.smoothing_factor, self.smoothing_iter);
        }

        Ok(&self.dense_field)
    }

    fn iso_surface(&self, iso_val: T) -> Result<Mesh<T>, ModelError> {
        let tris = algorithms::marching_cubes::generate_iso_surface(&self.dense_field, iso_val);
        Ok(Mesh::from_triangles(&tris, true))
    }

    fn field(&self) -> &DenseField<T> {
        &self.dense_field
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        computation::{
            data::{field_iterator::ValueIterator, BlockSize, SamplingMode},
            functions::XYZValue,
        },
        geometry::Vec3,
    };

    fn create_test_model() -> ImplicitModel<f32> {
        let mut model = ImplicitModel::new();
        model.add_function("z_coord", XYZValue::z()).unwrap();
        model
    }

    fn create_test_bounds() -> BoundingBox<f32> {
        BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0))
    }

    #[test]
    fn test_dense_sampler_builder() {
        let bounds = create_test_bounds();
        let cell_size = 1.0;

        // Test basic builder
        let sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(cell_size)
            .build();
        assert!(sampler.is_ok());

        // Test builder with all options
        let sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(cell_size)
            .with_smoothing_iter(2)
            .with_smoothing_factor(0.5)
            .with_padding(true)
            .build();
        assert!(sampler.is_ok());

        // Test builder fails without bounds
        let sampler = DenseSampler::<f32>::builder().build();
        assert!(sampler.is_err());

        // Test builder fails without cell size
        let sampler = DenseSampler::builder().with_bounds(bounds).build();
        assert!(sampler.is_err());
    }

    #[test]
    fn test_dense_sampler_field_operations() {
        let bounds = create_test_bounds();
        let model = create_test_model();
        let cell_size = 1.0;
        let mut sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(cell_size)
            .build()
            .unwrap();

        // Test sampling
        let field = sampler.sample_field(&model);
        assert!(field.is_ok());

        // Test iso-surface extraction
        let mesh = sampler.iso_surface(0.0);
        assert!(mesh.is_ok());
    }

    #[test]
    fn test_sparse_sampler_builder() {
        let bounds = create_test_bounds();
        let config = SparseFieldConfig {
            internal_size: BlockSize::Size64,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        };

        // Test basic builder
        let sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .build();
        assert!(sampler.is_ok());

        // Test builder with custom thresholds
        let sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .with_max_val(0.5)
            .with_min_val(-0.5)
            .build();
        assert!(sampler.is_ok());

        // Test builder fails without bounds
        let sampler = SparseSampler::<f32>::builder().with_config(config).build();
        assert!(sampler.is_err());

        // Test builder fails without config
        let sampler = SparseSampler::builder().with_bounds(bounds).build();
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
            cell_size: 1.0,
        };

        let mut sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .build()
            .unwrap();

        // Test sampling
        let field = sampler.sample_field(&model);
        assert!(field.is_ok());

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
            cell_size: 1.0,
        };

        let mut sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .with_max_val(0.5)
            .with_min_val(-0.5)
            .build()
            .unwrap();

        sampler.sample_field(&model).unwrap();

        // Test iso-surface extraction within bounds succeeds
        assert!(sampler.iso_surface(0.0).is_ok());

        // Test iso-surface extraction outside bounds fails
        assert!(sampler.iso_surface(1.0).is_err());
        assert!(sampler.iso_surface(-1.0).is_err());
    }

    #[test]
    fn test_dense_sampler_with_smoothing() {
        let model = create_test_model();
        let bounds = create_test_bounds();
        let cell_size = 1.0;

        let mut sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(cell_size)
            .with_smoothing_iter(3)
            .with_smoothing_factor(0.1)
            .build()
            .expect("Failed to build sampler");

        sampler
            .sample_field(&model)
            .expect("Failed to sample field");

        let field = sampler.field();
        assert!(field.iter_values().count() != 0);
    }

    #[test]
    fn test_dense_sampler_with_padding() {
        let model = create_test_model();
        let bounds = create_test_bounds();
        let cell_size = 1.0;

        let mut sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(cell_size)
            .with_padding(true)
            .build()
            .expect("Failed to build sampler");

        sampler
            .sample_field(&model)
            .expect("Failed to sample field");

        let field = sampler.field();
        assert!(field.iter_values().count() != 0);
    }

    #[test]
    fn test_sparse_sampler_different_block_sizes() {
        let bounds = create_test_bounds();
        let model = create_test_model();

        // Test with small blocks
        let config_small = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size2,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        };

        let mut sampler_small = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config_small)
            .build()
            .expect("Failed to build sampler");

        sampler_small
            .sample_field(&model)
            .expect("Failed to sample field with small blocks");

        // Test with large blocks
        let config_large = SparseFieldConfig {
            internal_size: BlockSize::Size32,
            leaf_size: BlockSize::Size8,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        };

        let mut sampler_large = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config_large)
            .build()
            .expect("Failed to build sampler");

        sampler_large
            .sample_field(&model)
            .expect("Failed to sample field with large blocks");

        assert!(sampler_small.field().iter_values().count() != 0);
        assert!(sampler_large.field().iter_values().count() != 0);
    }

    #[test]
    fn test_sparse_sampler_sampling_modes() {
        let bounds = create_test_bounds();
        let model = create_test_model();

        // Test CENTRE mode
        let config_centre = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        };

        let mut sampler_centre = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config_centre)
            .build()
            .expect("Failed to build sampler");

        sampler_centre
            .sample_field(&model)
            .expect("Failed to sample field with CENTRE mode");

        // Test CORNERS mode
        let config_corners = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CORNERS,
            cell_size: 1.0,
        };

        let mut sampler_corners = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config_corners)
            .build()
            .expect("Failed to build sampler");

        sampler_corners
            .sample_field(&model)
            .expect("Failed to sample field with CORNERS mode");

        // Both samplers should produce valid fields
        assert!(sampler_centre.field().iter_values().count() != 0);
        assert!(sampler_corners.field().iter_values().count() != 0);
    }

    #[test]
    fn test_sampler_invalid_component() {
        let bounds = create_test_bounds();
        let model = ImplicitModel::new();

        let mut dense_sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(1.0)
            .build()
            .expect("Failed to build dense sampler");

        let dense_result = dense_sampler.sample_field(&model);
        assert!(dense_result.is_err());

        let config = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        };

        let mut sparse_sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .build()
            .expect("Failed to build sparse sampler");

        let sparse_result = sparse_sampler.sample_field(&model);
        assert!(sparse_result.is_err());
    }

    #[test]
    fn test_sampler_iso_surface_without_field() {
        let bounds = create_test_bounds();

        let dense_sampler = DenseSampler::builder()
            .with_bounds(bounds)
            .with_cell_size(1.0)
            .build()
            .expect("Failed to build dense sampler");

        let dense_result = dense_sampler.iso_surface(0.0);
        assert!(dense_result.is_ok());

        let config = SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        };

        let sparse_sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .build()
            .expect("Failed to build sparse sampler");

        let sparse_result = sparse_sampler.iso_surface(0.0);
        assert!(sparse_result.is_ok(), "Expected mesh to be ok");
        assert!(
            sparse_result.unwrap().vertices().is_empty(),
            "Expected empty vertex list."
        );
    }
}
