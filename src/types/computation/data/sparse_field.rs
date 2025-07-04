use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use crate::utils;
use hashbrown::HashMap;
use num_traits::Float;
use rayon::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::field_iterator::{
    CellGridIter, CellGridIterator, GridIterator, PointGridIter, PointIterator,
};
use crate::types::computation::data::field_iterator::{
    CellIterator, CellValueIterator, DenseCellValueIterator, ValueIterator,
};
use crate::types::computation::model::ComputationGraph;
use crate::types::computation::traits::ModelFloat;
use crate::types::computation::ModelError;
use crate::types::geometry::{BoundingBox, Vec3};

/// 3-dimensional sparse field for scalar values.
///
/// The field uses a three-level hierarchical structure:
/// - A dynamic root node that can be subdivided arbitrarily (e.g., 1x1x2, 2x2x2, etc.)
/// - Internal nodes with a fixed subdivision pattern defined by [`BlockSize`]
/// - Dense leaf nodes containing the actual field data
///
/// This structure provides a balance between memory efficiency and computational
/// performance, allowing dense sampling only in regions where the field is active.
/// Empty or uniform regions can be efficiently represented at the internal node level.
///
/// Note: This type should not be constructed directly. Instead, use [`~SparseSampler`][crate::types::computation::data::sampler::SparseSampler]
/// to sample and extract a sparse field from an implicit model.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct SparseField<T: Float> {
    /// The configuration for block sizes
    config: SparseFieldConfig<T>,
    /// The root node of the tree structure
    root: RootNode<T>,
}

impl<T: Float> SparseField<T> {
    /// Creates a new empty sparse field with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration specifying block sizes and sampling mode.
    pub fn new(config: SparseFieldConfig<T>) -> Self {
        Self {
            config,
            root: RootNode::new(),
        }
    }

    /// Initializes the field's bounds and creates the necessary internal nodes.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box defining the field's extents.
    /// * `cell_size` - The size of each cell in the field.
    pub fn init_bounds(&mut self, bounds: &BoundingBox<T>) {
        self.root.init_bounds(bounds, &self.config);
    }
}

impl<T: ModelFloat + 'static + Default> SparseField<T> {
    /// Samples the field using a computation graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The computation graph to evaluate.
    /// * `min_val` - The minimum value threshold.
    /// * `max_val` - The maximum value threshold.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if sampling was successful, or an error if the space was not initialized.
    pub(crate) fn sample_from_graph(
        &mut self,
        graph: &ComputationGraph<T>,
        min_val: T,
        max_val: T,
    ) -> Result<(), ModelError> {
        let before = Instant::now();
        if self.root.table.is_empty() {
            return Err(ModelError::Custom("Space not initialized.".to_owned()));
        }

        let mut count: AtomicUsize = 0.into();
        for (_, node) in self.root.table.iter_mut() {
            if let NodeHandle::Internal(internal) = node {
                internal.sample_cells(
                    graph,
                    min_val,
                    max_val,
                    self.config.leaf_size,
                    self.config.sampling_mode,
                    &mut count,
                );
            }
        }

        log::info!(
            "Sparse field generated with {} sampled ({:.2?}% of active) points in {:.2?}",
            utils::math_helper::format_integer(count.load(Ordering::Relaxed)),
            100.0
                * (count.load(Ordering::Relaxed) as f64
                    / (self.n_active_nodes() * self.config.leaf_size.total_size()) as f64),
            before.elapsed()
        );
        Ok(())
    }

    /// Compute the total number of active leaf nodes.
    pub fn n_active_nodes(&self) -> usize {
        self.root
            .table
            .values()
            .filter_map(|node| {
                if let NodeHandle::Internal(internal) = node {
                    Some(internal)
                } else {
                    None
                }
            })
            .flat_map(|internal| internal.children.iter())
            .filter(|child| {
                matches!(child, NodeHandle::Leaf(_)) || matches!(child, NodeHandle::Constant(_, _))
            })
            .count()
    }
}

/// Block size options for the sparse field
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockSize {
    /// 2x2x2 blocks (8 values)
    Size2,
    /// 4x4x4 blocks (64 values)
    Size4,
    /// 8x8x8 blocks (512 values)
    Size8,
    /// 16x16x16 blocks (4096 values)
    Size16,
    /// 32x32x32 blocks (32768 values)
    Size32,
    /// 64x64x64 blocks (262144 values)
    Size64,
}

impl BlockSize {
    /// Returns the size value of the block.
    ///
    /// For example:
    /// - Size2 returns 2
    /// - Size4 returns 4
    /// - etc.
    #[inline(always)]
    fn value(&self) -> usize {
        match self {
            BlockSize::Size2 => 2,
            BlockSize::Size4 => 4,
            BlockSize::Size8 => 8,
            BlockSize::Size16 => 16,
            BlockSize::Size32 => 32,
            BlockSize::Size64 => 64,
        }
    }

    /// Returns the total number of elements in a block (size^3).
    ///
    /// For example:
    /// - Size2 returns 8 (2^3)
    /// - Size4 returns 64 (4^3)
    /// - etc.
    #[inline(always)]
    fn total_size(&self) -> usize {
        match self {
            BlockSize::Size2 => 8,
            BlockSize::Size4 => 64,      // 4^3
            BlockSize::Size8 => 512,     // 8^3
            BlockSize::Size16 => 4096,   // 16^3
            BlockSize::Size32 => 32768,  // 32^3
            BlockSize::Size64 => 262144, // 64^3
        }
    }
}

/// Configuration for the sparse field structure
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct SparseFieldConfig<T> {
    /// The size of internal nodes.
    pub internal_size: BlockSize,
    /// The size of leaf nodes.
    pub leaf_size: BlockSize,
    /// Sampling mode.
    pub sampling_mode: SamplingMode,
    /// Cell size
    pub cell_size: T,
}

impl<T: Float> Default for SparseFieldConfig<T> {
    fn default() -> Self {
        Self {
            internal_size: BlockSize::Size32,
            leaf_size: BlockSize::Size8,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: T::one(),
        }
    }
}

impl<T: Float> SparseFieldConfig<T> {
    /// Set the cell size of the config. Returns self for chaining.
    pub fn set_cell_size(mut self, cell_size: T) -> Self {
        self.cell_size = cell_size;
        self
    }

    /// Set the internal node size of the config. Returns self for chaining.
    pub fn set_internal_size(mut self, internal_size: BlockSize) -> Self {
        self.internal_size = internal_size;
        self
    }

    /// Set the leaf node size of the config. Returns self for chaining.
    pub fn set_leaf_size(mut self, leaf_size: BlockSize) -> Self {
        self.leaf_size = leaf_size;
        self
    }

    /// Set the sampling mode of the config. Returns self for chaining.
    pub fn set_sampling_mode(mut self, sampling_mode: SamplingMode) -> Self {
        self.sampling_mode = sampling_mode;
        self
    }
}

/// Root node containing pointers to other nodes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
struct RootNode<T: Float> {
    /// Table of nodes (can be internal nodes, leaves, constants, or empty)
    table: HashMap<(usize, usize, usize), NodeHandle<T>>,
}

impl<T: Float> RootNode<T> {
    /// Creates a new empty root node.
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<T: Float> RootNode<T> {
    /// Initializes the root node's bounds and creates the necessary internal nodes.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box defining the field's extents.
    /// * `cell_size` - The size of each cell in the field.
    /// * `config` - The configuration specifying block sizes and sampling mode.
    fn init_bounds(&mut self, bounds: &BoundingBox<T>, config: &SparseFieldConfig<T>) {
        let steps = config.internal_size.value() * (config.leaf_size.value() - 1);
        let node_size = config.cell_size * T::from(steps).unwrap();

        let size = bounds.dimensions();
        let nodes_x = ((size.0 / node_size).ceil().to_usize().unwrap()).max(1);
        let nodes_y = ((size.1 / node_size).ceil().to_usize().unwrap()).max(1);
        let nodes_z = ((size.2 / node_size).ceil().to_usize().unwrap()).max(1);

        self.table.clear();

        log::info!(
            "Initialized ({},{},{}) internal nodes.",
            nodes_x,
            nodes_y,
            nodes_z
        );
        for k in 0..nodes_z {
            for j in 0..nodes_y {
                for i in 0..nodes_x {
                    let min_x = bounds.min.x + T::from(i).unwrap() * node_size;
                    let min_y = bounds.min.y + T::from(j).unwrap() * node_size;
                    let min_z = bounds.min.z + T::from(k).unwrap() * node_size;

                    let node_bounds = BoundingBox::new(
                        Vec3::new(min_x, min_y, min_z),
                        Vec3::new(min_x + node_size, min_y + node_size, min_z + node_size),
                    );

                    let mut internal_node = InternalNode::new(node_bounds, config.internal_size);
                    internal_node.init_cells(bounds);
                    self.table
                        .insert((i, j, k), NodeHandle::Internal(Box::new(internal_node)));
                }
            }
        }
    }
}

/// Handle to a node in the tree - can be a leaf with actual data, an internal node with children,
/// a constant value for uniform regions, or empty space
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
enum NodeHandle<T> {
    /// Pointer to a leaf node containing values.
    Leaf(LeafNode<T>),
    /// Pointer to an internal node.
    Internal(Box<InternalNode<T>>),
    /// A constant value for uniform regions.
    Constant(BoundingBox<T>, T),
    /// Empty space (no data). Will be ignored when sampling.
    Empty,
}

/// Internal node in the sparse field tree structure
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
struct InternalNode<T> {
    /// The origin index of this block in the global coordinate system
    bounds: BoundingBox<T>,
    /// Child pointers (can be leaves, other internal nodes, constants, or empty)
    children: Vec<NodeHandle<T>>,
}

impl<T: Float> InternalNode<T> {
    /// Creates a new internal node with the given bounds and size.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box defining the node's extents.
    /// * `size` - The block size configuration for this node.
    fn new(bounds: BoundingBox<T>, size: BlockSize) -> Self {
        Self {
            bounds,
            children: vec![NodeHandle::Empty; size.total_size()],
        }
    }

    /// Initializes the cells within this node that intersect with the given bounds.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box to check for intersection.
    fn init_cells(&mut self, bounds: &BoundingBox<T>) {
        let mut num_active = 0;
        for i in 0..self.children.len() {
            let cell_bound = self.cell_bounds(i);
            if bounds.intersects(&cell_bound) {
                self.children[i] = NodeHandle::Constant(cell_bound, T::zero());
                num_active += 1;
            }
        }
        log::debug!(
            "Initialized node with {} cells. (Max: {})",
            num_active,
            self.children.len()
        );
    }

    /// Returns the bounds of the cell at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The linear index of the cell.
    ///
    /// # Returns
    ///
    /// The bounding box of the specified cell.
    pub fn cell_bounds(&self, index: usize) -> BoundingBox<T> {
        let cells_per_dim = (self.children.len() as f64).cbrt() as usize;
        let size = self.bounds.dimensions();
        let dx = size.0 / T::from(cells_per_dim).unwrap();
        let dy = size.1 / T::from(cells_per_dim).unwrap();
        let dz = size.2 / T::from(cells_per_dim).unwrap();

        // Convert linear index to 3D coordinates
        let k = index / (cells_per_dim * cells_per_dim);
        let temp = index - (k * cells_per_dim * cells_per_dim);
        let j = temp / cells_per_dim;
        let i = temp % cells_per_dim;

        let min_x = self.bounds.min.x + T::from(i).unwrap() * dx;
        let min_y = self.bounds.min.y + T::from(j).unwrap() * dy;
        let min_z = self.bounds.min.z + T::from(k).unwrap() * dz;

        BoundingBox::new(
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(min_x + dx, min_y + dy, min_z + dz),
        )
    }
}

/// Controls how internal nodes in the sparse field are evaluated to determine if they should be filled.
///
/// - [`SamplingMode::CENTRE`]: Evaluates only the center point and estimates coverage based on the size.
///   Faster but only accurate for linear distance fields. (For example may not be valid for TPS such as gyroids.)
/// - [`SamplingMode::CORNERS`]: Evaluates all corners to determine if node intersects the iso-surface.
///   More robust but computationally expensive.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum SamplingMode {
    /// Sample only the center point. Fast but requires linear distance fields.
    CENTRE,
    /// Sample all corners. More robust but slower.
    CORNERS,
}

impl<T: ModelFloat + Default + 'static> InternalNode<T> {
    /// Checks if a cell overlaps with the computation graph's non-zero region.
    ///
    /// # Arguments
    ///
    /// * `graph` - The computation graph to evaluate.
    /// * `min_val` - The minimum value threshold.
    /// * `max_val` - The maximum value threshold.
    /// * `cell_bounds` - The bounds of the cell to check.
    /// * `sampling_mode` - The mode to use for sampling points.
    ///
    /// # Returns
    ///
    /// `true` if the cell overlaps with the non-zero region, `false` otherwise.
    fn is_overlapping(
        graph: &ComputationGraph<T>,
        min_val: T,
        max_val: T,
        cell_bounds: &BoundingBox<T>,
        sampling_mode: SamplingMode,
    ) -> bool {
        match sampling_mode {
            SamplingMode::CENTRE => {
                // Sample only the centre. Assumes a linear & valid distance field.
                let centre = cell_bounds.centroid();
                let half_diag = cell_bounds.min.distance_to_vec3(&centre);
                let centre_val = graph.evaluate_at_coord(centre.x, centre.y, centre.z);
                centre_val - half_diag <= max_val && centre_val + half_diag >= min_val
            }
            SamplingMode::CORNERS => {
                let corners = cell_bounds.corners();
                let half_diag =
                    cell_bounds.min.distance_to_vec3(&cell_bounds.max) / T::from(2).unwrap();
                for pt in corners.iter() {
                    let corner_val = graph.evaluate_at_coord(pt.x, pt.y, pt.z);
                    if corner_val - half_diag <= max_val && corner_val + half_diag >= min_val {
                        return true;
                    }
                }

                false
            }
        }
    }

    /// Samples the cells in this node using the computation graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The computation graph to evaluate.
    /// * `min_val` - The minimum value threshold.
    /// * `max_val` - The maximum value threshold.
    /// * `leaf_size` - The size configuration for leaf nodes.
    /// * `sampling_mode` - The mode to use for sampling points.
    /// * `count` - Counter for the total number of sampled points.
    pub(crate) fn sample_cells(
        &mut self,
        graph: &ComputationGraph<T>,
        min_val: T,
        max_val: T,
        leaf_size: BlockSize,
        sampling_mode: SamplingMode,
        count: &mut AtomicUsize,
    ) {
        let bounds: Vec<_> = (0..self.children.len())
            .map(|index| self.cell_bounds(index))
            .collect();

        self.children
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, child)| {
                if !matches!(child, NodeHandle::Empty) {
                    let cell_bounds = bounds[index];
                    if Self::is_overlapping(graph, min_val, max_val, &cell_bounds, sampling_mode) {
                        let mut leaf = LeafNode::new(cell_bounds, leaf_size);
                        leaf.sample_points(graph);
                        *child = NodeHandle::Leaf(leaf);
                        count.fetch_add(leaf_size.total_size().into(), Ordering::Relaxed);
                    } else {
                        let centre = cell_bounds.centroid();
                        *child = NodeHandle::Constant(
                            cell_bounds,
                            graph.evaluate_at_coord(centre.x, centre.y, centre.z),
                        );
                    }
                }
            });
    }
}

/// A dense block of values used as leaf nodes in the sparse field
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
struct LeafNode<T> {
    /// The bounding box of this block in index space
    bounds: BoundingBox<T>,
    /// The actual data values stored in this block
    values: Vec<T>,
}

impl<T: Float + Default> LeafNode<T> {
    /// Creates a new leaf node with the given bounds and size.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box defining the node's extents.
    /// * `size` - The block size configuration for this node.
    fn new(bounds: BoundingBox<T>, size: BlockSize) -> Self {
        Self {
            bounds,
            values: vec![T::default(); size.total_size()],
        }
    }
}

impl<T: ModelFloat + Default> LeafNode<T> {
    /// Samples points in this leaf node using the computation graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The computation graph to evaluate.
    fn sample_points(&mut self, graph: &ComputationGraph<T>) {
        // Now we can iterate over the collected points and modify self
        for (index, point) in self.iter_points().enumerate() {
            let value = graph.evaluate_at_coord(point.x, point.y, point.z);
            self.values[index] = value;
        }
    }
}

// Iterators

impl<T: Float + 'static> ValueIterator<T> for LeafNode<T> {
    type Iter<'a> = std::iter::Copied<std::slice::Iter<'a, T>>;

    /// Returns an iterator that yields each value in this leaf node.
    fn iter_values<'a>(&'a self) -> Self::Iter<'a> {
        self.values.iter().copied()
    }
}

impl<T: Float + 'static> PointIterator<T> for LeafNode<T> {
    type Iter<'a>
        = PointGridIter<T>
    where
        T: 'a;

    /// Returns an iterator that yields all point coordinates in this leaf node.
    fn iter_points(&self) -> PointGridIter<T> {
        self.iter_grid()
    }
}

impl<T: Float + 'static> CellIterator<T> for LeafNode<T> {
    type Iter<'a>
        = CellGridIter<T>
    where
        T: 'a;

    /// Returns an iterator that yields all cell coordinates in this leaf node.
    fn iter_cells(&self) -> CellGridIter<T> {
        self.iter_cell_grid()
    }
}

impl<T: Float + 'static> CellGridIterator<T> for LeafNode<T> {
    type GridIter<'a>
        = CellGridIter<T>
    where
        Self: 'a;

    /// Returns an iterator that yields all cell coordinates in this leaf node's grid.
    fn iter_cell_grid(&self) -> CellGridIter<T> {
        let cells_per_dim = (self.values.len() as f64).cbrt() as usize - 1;
        CellGridIter::new(self.bounds, (cells_per_dim, cells_per_dim, cells_per_dim))
    }
}

impl<T: Float + 'static> GridIterator<T> for LeafNode<T> {
    type GridIter<'a>
        = PointGridIter<T>
    where
        Self: 'a;

    /// Returns an iterator that yields all grid point coordinates in this leaf node.
    fn iter_grid<'a>(&'a self) -> Self::GridIter<'a> {
        let points_per_dim = (self.values.len() as f64).cbrt() as usize;
        PointGridIter::new(
            self.bounds,
            (points_per_dim, points_per_dim, points_per_dim),
        )
    }
}

impl<T: Float> CellValueIterator<T> for LeafNode<T> {
    type Iter<'a>
        = DenseCellValueIterator<'a, T>
    where
        Self: 'a;

    /// Returns an iterator that yields the values at each cell's corners in this leaf node.
    fn iter_cell_values<'a>(&'a self) -> Self::Iter<'a> {
        let pts_per_dim = (self.values.len() as f64).cbrt() as usize;
        DenseCellValueIterator {
            data: &self.values,
            current: (0, 0, 0),
            point_count: (pts_per_dim, pts_per_dim, pts_per_dim),
        }
    }
}

impl<T: Float + Copy> CellIterator<T> for InternalNode<T> {
    type Iter<'a>
        = CellGridIter<T>
    where
        Self: 'a;

    /// Returns an iterator that yields all cell coordinates in this internal node.
    fn iter_cells<'a>(&'a self) -> Self::Iter<'a> {
        self.iter_cell_grid()
    }
}

impl<T: Float + Copy> CellGridIterator<T> for InternalNode<T> {
    type GridIter<'a>
        = CellGridIter<T>
    where
        Self: 'a;

    /// Returns an iterator that yields all cell coordinates in this internal node's grid.
    fn iter_cell_grid<'a>(&'a self) -> Self::GridIter<'a> {
        let cells_per_dim = (self.children.len() as f64).cbrt() as usize;
        CellGridIter::new(self.bounds, (cells_per_dim, cells_per_dim, cells_per_dim))
    }
}

impl<T: Float + 'static> PointIterator<T> for SparseField<T> {
    type Iter<'a>
        = Box<dyn Iterator<Item = Vec3<T>> + 'a>
    where
        Self: 'a;

    /// Returns an iterator that yields all point coordinates in each leaf node in the sparse field.
    fn iter_points<'a>(&'a self) -> Self::Iter<'a> {
        let iter = self
            .root
            .table
            .values()
            .filter_map(|node| {
                if let NodeHandle::Internal(internal) = node {
                    Some(internal)
                } else {
                    None
                }
            })
            .flat_map(move |internal| {
                internal
                    .children
                    .iter()
                    .filter_map(|child| {
                        if let NodeHandle::Leaf(leaf) = child {
                            Some(leaf)
                        } else {
                            None
                        }
                    })
                    .flat_map(move |leaf| leaf.iter_points())
            });

        Box::new(iter)
    }
}

impl<T: Float + 'static> ValueIterator<T> for SparseField<T> {
    type Iter<'a>
        = Box<dyn Iterator<Item = T> + 'a>
    where
        Self: 'a;

    /// Returns an iterator that yields each value in the leaf nodes in the sparse field.
    fn iter_values<'a>(&'a self) -> Self::Iter<'a> {
        let iter = self
            .root
            .table
            .values()
            .filter_map(|node| {
                if let NodeHandle::Internal(internal) = node {
                    Some(internal)
                } else {
                    None
                }
            })
            .flat_map(move |internal| {
                internal
                    .children
                    .iter()
                    .filter_map(|child| {
                        if let NodeHandle::Leaf(leaf) = child {
                            Some(leaf)
                        } else {
                            None
                        }
                    })
                    .flat_map(move |leaf| leaf.iter_values())
            });

        Box::new(iter)
    }
}

impl<T: Float + 'static> CellIterator<T> for SparseField<T> {
    type Iter<'a>
        = Box<dyn Iterator<Item = BoundingBox<T>> + 'a>
    where
        Self: 'a;

    /// Returns an iterator that yields all cell coordinates in the leaf nodes in the sparse field.
    fn iter_cells<'a>(&'a self) -> Self::Iter<'a> {
        let iter = self
            .root
            .table
            .values()
            .filter_map(|node| {
                if let NodeHandle::Internal(internal) = node {
                    Some(internal)
                } else {
                    None
                }
            })
            .flat_map(move |internal| {
                internal
                    .children
                    .iter()
                    .filter_map(|child| {
                        if let NodeHandle::Leaf(leaf) = child {
                            Some(leaf)
                        } else {
                            None
                        }
                    })
                    .flat_map(move |leaf| leaf.iter_cells())
            });

        Box::new(iter)
    }
}

impl<T: Float + 'static> CellValueIterator<T> for SparseField<T> {
    type Iter<'a>
        = Box<dyn Iterator<Item = [T; 8]> + 'a>
    where
        Self: 'a;

    /// Returns an iterator that yields the values at each cell's corners in the leaf nodes in the sparse field.
    fn iter_cell_values<'a>(&'a self) -> Self::Iter<'a> {
        let iter = self
            .root
            .table
            .values()
            .filter_map(|node| {
                if let NodeHandle::Internal(internal) = node {
                    Some(internal)
                } else {
                    None
                }
            })
            .flat_map(move |internal| {
                internal
                    .children
                    .iter()
                    .filter_map(|child| {
                        if let NodeHandle::Leaf(leaf) = child {
                            Some(leaf)
                        } else {
                            None
                        }
                    })
                    .flat_map(move |leaf| leaf.iter_cell_values())
            });

        Box::new(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        computation::{
            data::sampler::{Sampler, SparseSampler},
            model::ImplicitModel,
        },
        geometry::{Sphere, Vec3},
    };

    fn create_test_bounds() -> BoundingBox<f32> {
        BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0))
    }

    fn create_test_model() -> ImplicitModel<f32> {
        let mut model = ImplicitModel::new();
        model.add_constant("constant", 1.0).unwrap();
        model
    }

    fn create_test_config() -> SparseFieldConfig<f32> {
        SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        }
    }

    #[test]
    fn test_sparse_field_initialization() {
        let mut field = SparseField::new(create_test_config());
        let bounds = create_test_bounds();

        // Test initialization
        field.init_bounds(&bounds);

        // Verify root node exists
        assert!(!field.root.table.is_empty());
    }

    #[test]
    fn test_sparse_field_sampling() {
        let mut field = SparseField::new(create_test_config());
        let bounds = create_test_bounds();
        let model = create_test_model();

        // Initialize and sample
        field.init_bounds(&bounds);
        let graph = model.compile("constant").unwrap();
        field.sample_from_graph(&graph, -0.1, 0.1).unwrap();

        // Verify field contains data
        assert!(!field.root.table.is_empty());

        // Test some points are within expected range
        let mut found_active = false;
        for node in field.root.table.values() {
            if let NodeHandle::Internal(internal) = node {
                for child in internal.children.iter() {
                    if let NodeHandle::Leaf(_) = child {
                        found_active = true;
                        break;
                    }
                }
            }
        }
        assert!(found_active, "No active leaf nodes found");
    }

    #[test]
    fn test_sparse_field_iterators() {
        let mut field = SparseField::new(create_test_config());
        let bounds = create_test_bounds();
        let model = create_test_model();

        // Initialize and sample
        field.init_bounds(&bounds);
        let graph = model.compile("constant").unwrap();
        field.sample_from_graph(&graph, -0.1, 0.1).unwrap();

        // Test value iterator
        let values: Vec<_> = field.iter_values().collect();
        assert!(!values.is_empty(), "Value iterator should yield values");

        // Test cell value iterator
        let cell_values: Vec<_> = field.iter_cell_values().collect();
        assert!(
            !cell_values.is_empty(),
            "Cell value iterator should yield values"
        );
        assert_eq!(
            cell_values[0].len(),
            8,
            "Each cell should have 8 corner values"
        );

        // Test cell iterator
        let cells: Vec<_> = field.iter_cells().collect();
        assert!(!cells.is_empty(), "Cell iterator should yield cells");
    }

    #[test]
    fn test_sampling_modes() {
        // Test CENTRE mode
        let mut field_centre = SparseField::new(SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CENTRE,
            cell_size: 1.0,
        });
        let bounds = create_test_bounds();
        let model = create_test_model();

        field_centre.init_bounds(&bounds);
        let graph = model.compile("constant").unwrap();
        field_centre.sample_from_graph(&graph, -0.1, 0.1).unwrap();

        // Test CORNERS mode
        let mut field_corners = SparseField::new(SparseFieldConfig {
            internal_size: BlockSize::Size8,
            leaf_size: BlockSize::Size4,
            sampling_mode: SamplingMode::CORNERS,
            cell_size: 1.0,
        });

        field_corners.init_bounds(&bounds);
        field_corners.sample_from_graph(&graph, -0.1, 0.1).unwrap();

        // Both modes should produce valid fields
        assert!(!field_centre.root.table.is_empty());
        assert!(!field_corners.root.table.is_empty());
    }

    #[test]
    fn test_block_sizes() {
        let sizes = [
            BlockSize::Size2,
            BlockSize::Size4,
            BlockSize::Size8,
            BlockSize::Size16,
            BlockSize::Size32,
            BlockSize::Size64,
        ];

        for &size in sizes.iter() {
            let expected = size.value().pow(3);
            let actual = size.total_size();
            assert_eq!(
                actual, expected,
                "Block total size should be cube of value: expected {expected}, got {actual}"
            );
        }

        // Test specific values
        assert_eq!(BlockSize::Size2.value(), 2);
        assert_eq!(BlockSize::Size2.total_size(), 8);
        assert_eq!(BlockSize::Size64.value(), 64);
        assert_eq!(BlockSize::Size64.total_size(), 262144);
    }

    #[test]
    fn test_error_handling() {
        let mut field = SparseField::new(create_test_config());
        let bounds = create_test_bounds();
        let model = create_test_model();

        // Test sampling without initialization
        let graph = model.compile("constant").unwrap();
        let result = field.sample_from_graph(&graph, -0.1, 0.1);
        assert!(
            result.is_err(),
            "Sampling without initialization should fail"
        );

        // Initialize and test with invalid component
        field.init_bounds(&bounds);
        let result = model.compile("nonexistent");
        assert!(result.is_err(), "Compiling invalid component should fail");
    }

    #[test]
    fn sample_sparse_field_multiple_root_nodes() {
        let cell_size = 0.5;
        let size: f32 = 16.0;
        let sphere = Sphere::at_coord(0., 0., 0., 0.9 * size);
        let bounds = BoundingBox::new(
            Vec3 {
                x: -size,
                y: -size,
                z: -size,
            },
            Vec3 {
                x: size,
                y: size,
                z: size,
            },
        );

        let mut model = ImplicitModel::new();
        let _ = model.add_function("sphere", sphere).unwrap();

        let config = SparseFieldConfig::default()
            .set_cell_size(cell_size)
            .set_internal_size(BlockSize::Size32)
            .set_leaf_size(BlockSize::Size2);

        let mut sampler = SparseSampler::builder()
            .with_bounds(bounds)
            .with_config(config)
            .build()
            .unwrap();

        sampler.sample_field(&model).unwrap();
        let mesh = sampler.iso_surface(0.0).unwrap();

        let num_f = mesh.faces().len();
        let num_v = mesh.vertices().len();
        let expected_f = 31208;
        let expected_v = 15606;
        assert_eq!(
            expected_v, num_v,
            "Expected {expected_v} vertices but found {num_v}"
        );
        assert_eq!(
            expected_f, num_f,
            "Expected {expected_f} faces but found {num_f}"
        );
    }
}
