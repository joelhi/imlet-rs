use std::fmt::Debug;
use std::time::Instant;

use hashbrown::HashMap;
use num_traits::Float;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use super::field_iterator::{
    CellGridIter, CellGridIterator, GridIterator, PointGridIter, PointIterator,
};
use crate::types::computation::data::field_iterator::{
    CellIterator, CellValueIterator, DenseCellValueIterator, ValueIterator,
};
use crate::types::computation::model::ComputationGraph;
use crate::types::computation::ModelError;
use crate::types::geometry::{BoundingBox, Vec3};
use crate::utils::math_helper::Pi;

/// 3-dimensional sparse field for scalar values.
///
/// The field uses a hierarchical tree structure where each node can be:
/// - A leaf node containing actual values
/// - An internal node with child nodes
/// - A constant value for uniform regions
/// - Empty space
///
/// This allows for arbitrary nesting depth and efficient representation of
/// both sparse and dense regions of the field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseField<T: Float> {
    /// The configuration for block sizes
    config: SparseFieldConfig,
    /// The root node of the tree structure
    root: RootNode<T>,
}

impl<T: Float> SparseField<T> {
    /// Create a new empty sparse field
    pub fn new(config: SparseFieldConfig) -> Self {
        Self {
            config,
            root: RootNode::new(),
        }
    }

    pub fn init_bounds(&mut self, bounds: &BoundingBox<T>, cell_size: T) {
        self.root.init_bounds(bounds, cell_size, &self.config);
    }
}

impl<T: Float + Default + Copy + Send + Sync + Serialize + 'static + Pi> SparseField<T> {
    /// Sample the field using a computation graph
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

        for (_, node) in self.root.table.iter_mut() {
            if let NodeHandle::Internal(internal) = node {
                internal.sample_cells(
                    graph,
                    min_val,
                    max_val,
                    self.config.leaf_size,
                    self.config.sampling_mode,
                );
            }
        }

        log::info!("Sparse field generated in {:.2?}", before.elapsed());
        Ok(())
    }
}

/// Block size options for the sparse field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Get the size value
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

    /// Get the total number of elements in a block (size^3)
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SparseFieldConfig {
    /// The size of internal nodes.
    pub internal_size: BlockSize,
    /// The size of leaf nodes.
    pub leaf_size: BlockSize,
    /// Sampling mode
    pub sampling_mode: SamplingMode,
}

/// Root node containing pointers to other nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RootNode<T: Float> {
    /// Table of nodes (can be internal nodes, leaves, constants, or empty)
    table: HashMap<(usize, usize, usize), NodeHandle<T>>,
}

impl<T: Float> RootNode<T> {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<T: Float> RootNode<T> {
    fn init_bounds(&mut self, bounds: &BoundingBox<T>, cell_size: T, config: &SparseFieldConfig) {
        let steps = config.internal_size.value() * (config.leaf_size.value() - 1);
        let node_size = cell_size * T::from(steps).unwrap();

        let size = bounds.dimensions();
        let nodes_x = ((size.0 / node_size).ceil().to_usize().unwrap()).max(1);
        let nodes_y = ((size.1 / node_size).ceil().to_usize().unwrap()).max(1);
        let nodes_z = ((size.2 / node_size).ceil().to_usize().unwrap()).max(1);

        self.table.clear();

        log::info!("Initalized ({},{},{})", nodes_x, nodes_y, nodes_z);
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
struct InternalNode<T> {
    /// The origin index of this block in the global coordinate system
    bounds: BoundingBox<T>,
    /// Child pointers (can be leaves, other internal nodes, constants, or empty)
    children: Vec<NodeHandle<T>>,
}

impl<T: Float> InternalNode<T> {
    fn new(bounds: BoundingBox<T>, size: BlockSize) -> Self {
        Self {
            bounds,
            children: vec![NodeHandle::Empty; size.total_size()],
        }
    }

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SamplingMode {
    CENTRE,
    CORNERS,
}

impl<T: Float + Send + Sync + Serialize + Default + 'static + Pi> InternalNode<T> {
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

    /// Sample cells and create leaf nodes where needed
    pub(crate) fn sample_cells(
        &mut self,
        graph: &ComputationGraph<T>,
        min_val: T,
        max_val: T,
        leaf_size: BlockSize,
        sampling_mode: SamplingMode,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LeafNode<T> {
    /// The bounding box of this block in index space
    bounds: BoundingBox<T>,
    /// The actual data values stored in this block
    values: Vec<T>,
}

impl<T: Float + Default> LeafNode<T> {
    fn new(bounds: BoundingBox<T>, size: BlockSize) -> Self {
        Self {
            bounds,
            values: vec![T::default(); size.total_size()],
        }
    }
}

impl<T: Float + Default + Send + Sync + Serialize + Pi> LeafNode<T> {
    /// Sample all points in the leaf node
    fn sample_points(&mut self, graph: &ComputationGraph<T>) {
        // Now we can iterate over the collected points and modify self
        for (index, point) in self.iter_points().enumerate() {
            let value = graph.evaluate_at_coord(point.x, point.y, point.z);
            self.values[index] = value;
        }
    }
}

/// Iterators

impl<T: Float + 'static> ValueIterator<T> for LeafNode<T> {
    type Iter<'a> = std::iter::Copied<std::slice::Iter<'a, T>>;

    fn iter_values<'a>(&'a self) -> Self::Iter<'a> {
        self.values.iter().copied()
    }
}

impl<T: Float + 'static> PointIterator<T> for LeafNode<T> {
    type Iter<'a>
        = PointGridIter<T>
    where
        T: 'a;

    fn iter_points(&self) -> PointGridIter<T> {
        self.iter_grid()
    }
}

impl<T: Float + 'static> CellIterator<T> for LeafNode<T> {
    type Iter<'a>
        = CellGridIter<T>
    where
        T: 'a;

    fn iter_cells(&self) -> CellGridIter<T> {
        self.iter_cell_grid()
    }
}

impl<T: Float + 'static> CellGridIterator<T> for LeafNode<T> {
    type GridIter<'a>
        = CellGridIter<T>
    where
        Self: 'a;

    fn iter_cell_grid(&self) -> CellGridIter<T> {
        let cells_per_dim = (self.values.len() as f64).cbrt() as usize - 1;
        CellGridIter::new(
            self.bounds.clone(),
            (cells_per_dim, cells_per_dim, cells_per_dim),
        )
    }
}

impl<T: Float + 'static> GridIterator<T> for LeafNode<T> {
    type GridIter<'a>
        = PointGridIter<T>
    where
        Self: 'a;

    fn iter_grid<'a>(&'a self) -> Self::GridIter<'a> {
        let points_per_dim = (self.values.len() as f64).cbrt() as usize;
        PointGridIter::new(
            self.bounds.clone(),
            (points_per_dim, points_per_dim, points_per_dim),
        )
    }
}

impl<T: Float> CellValueIterator<T> for LeafNode<T> {
    type Iter<'a>
        = DenseCellValueIterator<'a, T>
    where
        Self: 'a;

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

    fn iter_cells<'a>(&'a self) -> Self::Iter<'a> {
        self.iter_cell_grid()
    }
}

impl<T: Float + Copy> CellGridIterator<T> for InternalNode<T> {
    type GridIter<'a>
        = CellGridIter<T>
    where
        Self: 'a;

    fn iter_cell_grid<'a>(&'a self) -> Self::GridIter<'a> {
        let cells_per_dim = (self.children.len() as f64).cbrt() as usize;
        CellGridIter::new(
            self.bounds.clone(),
            (cells_per_dim, cells_per_dim, cells_per_dim),
        )
    }
}

impl<T: Float + 'static> PointIterator<T> for SparseField<T> {
    type Iter<'a>
        = Box<dyn Iterator<Item = Vec3<T>> + 'a>
    where
        Self: 'a;

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
