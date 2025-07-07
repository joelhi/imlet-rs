use std::fmt::Debug;
use std::time::Instant;

use hashbrown::HashSet;
use num_traits::Float;
use rayon::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::computation::data::field_iterator::CellIterator;
use crate::types::computation::data::field_iterator::DenseCellValueIterator;
use crate::types::computation::data::field_iterator::GridIterator;
use crate::types::computation::model::ComputationGraph;
use crate::types::computation::traits::ModelFloat;
use crate::types::computation::ModelError;
use crate::types::geometry::BoundingBox;
use crate::types::geometry::Vec3;
use crate::types::geometry::Vec3i;
use crate::utils;
use crate::utils::math_helper::index1d_from_index3d;
use crate::utils::math_helper::index3d_from_index1d;

use super::field_iterator::{
    CellGridIter, CellGridIterator, CellValueIterator, PointGridIter, PointIterator, ValueIterator,
};

/// 3-dimensional dense field for scalar values.
///
/// A uniform grid representation that stores field values at every point in the sampling domain.
/// The field geometry is defined by:
/// - An origin point defining the minimum corner of the field
/// - A uniform cell size for all dimensions
/// - The number of points in x, y, and z directions
///
/// The field stores values in a contiguous array, providing efficient access and parallel
/// processing capabilities. This representation is memory-intensive but offers fast
/// random access and is well-suited for operations like smoothing and iso-surface extraction.
///
/// Note: This type should not be constructed directly. Instead, use [`~DenseSampler`][crate::types::computation::data::sampler::DenseSampler]
/// to sample and extract a dense field from an implicit model.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct DenseField<T> {
    origin: Vec3<T>,
    cell_size: T,
    n: Vec3i,
    data: Vec<T>,
    bounds: BoundingBox<T>,
}

impl<T> DenseField<T> {
    /// Returns a reference to the origin point of the field as a [`Vec3<T>`].
    pub fn origin(&self) -> &Vec3<T> {
        &self.origin
    }

    /// Returns the total number of points in the field as the product of points in each direction.
    pub fn num_points(&self) -> usize {
        self.n.product()
    }

    /// Returns the total number of cells in the field as the product of cells in each direction.
    /// Note that the number of cells is one less than the number of points in each direction.
    pub fn num_cells(&self) -> usize {
        (self.n.i - 1) * (self.n.j - 1) * (self.n.k - 1)
    }

    /// Returns a reference to the underlying data buffer as a slice.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns the indices of the eight corners of a cell at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    ///
    /// # Returns
    ///
    /// An array of 8 indices corresponding to the corners of the cell in the data buffer.
    pub(crate) fn cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        [
            self.point_index1d(i, j, k),
            self.point_index1d(i + 1, j, k),
            self.point_index1d(i + 1, j + 1, k),
            self.point_index1d(i, j + 1, k),
            self.point_index1d(i, j, k + 1),
            self.point_index1d(i + 1, j, k + 1),
            self.point_index1d(i + 1, j + 1, k + 1),
            self.point_index1d(i, j + 1, k + 1),
        ]
    }

    /// Converts 3D point coordinates to a 1D index in the data buffer.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    ///
    /// # Returns
    ///
    /// The 1D index corresponding to the given 3D coordinates.
    #[inline(always)]
    pub(crate) fn point_index1d(&self, i: usize, j: usize, k: usize) -> usize {
        index1d_from_index3d(i, j, k, self.n.i, self.n.j, self.n.k)
    }

    /// Converts a 1D index to 3D point coordinates.
    ///
    /// # Arguments
    ///
    /// * `index` - The 1D index in the data buffer.
    ///
    /// # Returns
    ///
    /// A tuple containing the (i, j, k) coordinates corresponding to the 1D index.
    #[inline(always)]
    pub(crate) fn point_index3d(&self, index: usize) -> (usize, usize, usize) {
        index3d_from_index1d(index, self.n.i, self.n.j, self.n.k)
    }

    /// Converts 3D cell coordinates to a 1D index in the cell space.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    ///
    /// # Returns
    ///
    /// The 1D index corresponding to the given 3D cell coordinates.
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn cell_index1d(&self, i: usize, j: usize, k: usize) -> usize {
        index1d_from_index3d(i, j, k, self.n.i - 1, self.n.j - 1, self.n.k - 1)
    }

    /// Converts a 1D index to 3D cell coordinates.
    ///
    /// # Arguments
    ///
    /// * `index` - The 1D index in the cell space.
    ///
    /// # Returns
    ///
    /// A tuple containing the (i, j, k) coordinates corresponding to the 1D cell index.
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn cell_index3d(&self, index: usize) -> (usize, usize, usize) {
        index3d_from_index1d(index, self.n.i - 1, self.n.j - 1, self.n.k - 1)
    }
}

impl<T: Float> DenseField<T> {
    /// Create a new empty field.
    ///
    /// # Arguments
    ///
    /// * `origin` - The base of the field, and the first data location.
    /// * `cell_size` - The size of each cell in the field.
    /// * `num_pts` - Number of points in each direction.
    ///
    /// # Returns
    ///
    /// A new [`DenseField`] initialized with zeros and the specified parameters.
    pub fn new(origin: Vec3<T>, cell_size: T, num_pts: Vec3i) -> Self {
        let size = Vec3::new(
            cell_size * T::from(num_pts.i - 1).unwrap(),
            cell_size * T::from(num_pts.j - 1).unwrap(),
            cell_size * T::from(num_pts.k - 1).unwrap(),
        );
        let bounds = BoundingBox::new(origin, origin + size);
        Self {
            origin,
            cell_size,
            n: num_pts,
            data: vec![T::zero(); num_pts.product()],
            bounds,
        }
    }

    /// Create a new empty field from bounds and a cell size.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The extents of the field.
    /// * `cell_size` - The size of each cell in the field.
    ///
    /// # Returns
    ///
    /// A new [`DenseField`] initialized with zeros and dimensions derived from the bounds and cell size.
    pub fn from_bounds(bounds: BoundingBox<T>, cell_size: T) -> Self {
        DenseField::new(
            bounds.min,
            cell_size,
            DenseField::point_count(&bounds, cell_size),
        )
    }

    /// Create a new field from a data buffer.
    ///
    /// The size of the data buffer must match the specified point count.
    ///
    /// # Arguments
    ///
    /// * `origin` - The base of the field, and the first value location in space.
    /// * `cell_size` - The size of each cell in the field.
    /// * `num_pts` - Number of points in each direction.
    /// * `data` - The data buffer.
    ///
    /// # Returns
    ///
    /// [`Ok`] with the generated [`DenseField`] if the data matches the point count, or [`Err`] if the data doesn't match.
    pub fn from_data(
        origin: Vec3<T>,
        cell_size: T,
        num_pts: Vec3i,
        data: Vec<T>,
    ) -> Result<Self, ModelError> {
        if num_pts.product() != data.len() {
            return Err(ModelError::Custom(
                "Failed to generate field from data. Point count and data length must match"
                    .to_owned(),
            ));
        }
        Ok(Self {
            origin,
            cell_size,
            n: num_pts,
            data,
            bounds: BoundingBox::new(
                origin,
                origin
                    + Vec3::new(
                        cell_size * T::from(num_pts.i - 1).unwrap(),
                        cell_size * T::from(num_pts.j - 1).unwrap(),
                        cell_size * T::from(num_pts.k - 1).unwrap(),
                    ),
            ),
        })
    }

    /// Sets the value at a specific index in the data buffer.
    ///
    /// # Arguments
    ///
    /// * `index` - The 1D index in the data buffer.
    /// * `value` - The new value to set.
    pub fn set_value(&mut self, index: usize, value: T) {
        self.data[index] = value;
    }

    /// Returns the cell size of the field.
    pub fn cell_size(&self) -> T {
        self.cell_size
    }

    /// Returns a copy of the data buffer in the field.
    pub fn copy_data(&self) -> Vec<T> {
        self.data.clone()
    }

    /// Returns the vertex locations at the corners of the specified cell.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    ///
    /// # Returns
    ///
    /// An array of 8 [`Vec3<T>`] coordinates corresponding to the corners of the cell.
    pub fn cell_corners(&self, i: usize, j: usize, k: usize) -> [Vec3<T>; 8] {
        let size = self.cell_size;
        let i_val = T::from(i).expect("Failed to convert number to T");
        let j_val = T::from(j).expect("Failed to convert number to T");
        let k_val = T::from(k).expect("Failed to convert number to T");
        let one = T::one();
        [
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: j_val * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: j_val * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: (j_val + one) * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: (j_val + one) * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: j_val * size,
                    z: (k_val + one) * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: j_val * size,
                    z: (k_val + one) * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: (j_val + one) * size,
                    z: (k_val + one) * size,
                },
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: (j_val + one) * size,
                    z: (k_val + one) * size,
                },
        ]
    }

    /// Returns the values at the corners of the specified cell.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    ///
    /// # Returns
    ///
    /// An array of 8 values corresponding to the corners of the cell.
    pub fn cell_values(&self, i: usize, j: usize, k: usize) -> [T; 8] {
        let cell_ids = self.cell_ids(i, j, k);
        [
            self.data[cell_ids[0]],
            self.data[cell_ids[1]],
            self.data[cell_ids[2]],
            self.data[cell_ids[3]],
            self.data[cell_ids[4]],
            self.data[cell_ids[5]],
            self.data[cell_ids[6]],
            self.data[cell_ids[7]],
        ]
    }

    /// Assigns 0 to any point with an absolute value below the limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit threshold for non-zero values.
    pub fn threshold(&mut self, limit: T) {
        self.data.iter_mut().for_each(|value| {
            if (*value).abs() < limit {
                *value = T::zero();
            }
        });
    }

    /// Calculates the sum of values at the six adjacent points for a given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The 1D index of the point in the data buffer.
    ///
    /// # Returns
    ///
    /// [`Some`] containing the sum of adjacent values if the point is not on the boundary,
    /// or [`None`] if the point is on the boundary.
    fn neighbours_sum(&self, index: usize) -> Option<T> {
        let (i, j, k) = self.point_index3d(index);

        if i < 1 || j < 1 || k < 1 || i == self.n.i - 1 || j == self.n.j - 1 || k == self.n.k - 1 {
            return None;
        }
        Some(
            self.data[self.point_index1d(i + 1, j, k)]
                + self.data[self.point_index1d(i - 1, j, k)]
                + self.data[self.point_index1d(i, j + 1, k)]
                + self.data[self.point_index1d(i, j - 1, k)]
                + self.data[self.point_index1d(i, j, k + 1)]
                + self.data[self.point_index1d(i, j, k - 1)],
        )
    }

    /// Performs a laplacian smoothing operation on the field data.
    ///
    /// The value of each point will be updated based on the average of the adjacent points.
    ///
    /// # Arguments
    ///
    /// * `factor` - Interpolation value between the average of the adjacent points and the current value.
    /// * `iterations` - Number of successive smoothing iterations.
    pub fn smooth(&mut self, factor: T, iterations: u32) {
        let before = Instant::now();
        let mut smoothed = vec![T::zero(); self.num_points()];
        for _ in 0..iterations {
            smoothed.iter_mut().enumerate().for_each(|(index, val)| {
                if let Some(sum) = self.neighbours_sum(index) {
                    let laplacian = sum / T::from(6.0).expect("Failed to convert number to T");
                    *val = (T::one() - factor) * self.data[index] + factor * laplacian;
                } else {
                    *val = self.data[index];
                };
            });
            std::mem::swap(&mut self.data, &mut smoothed);
        }

        log::info!(
            "Dense value data for {} points smoothed in {:.2?} for {} iterations",
            utils::math_helper::format_integer(self.num_points()),
            before.elapsed(),
            iterations
        );
    }

    /// Applies padding to the field boundaries. All boundary points will be assigned a specific value.
    ///
    /// This can be used to ensure open edges of a solid are capped at the bounds.
    ///
    /// # Arguments
    ///
    /// * `padding_value` - The value to assign to all boundary points.
    pub fn padding(&mut self, padding_value: T) {
        let before = Instant::now();
        let (num_x, num_y, num_z) = self.n.into();
        for i in 0..self.n.i {
            for j in 0..self.n.j {
                let index_a = index1d_from_index3d(i, j, 0, num_x, num_y, num_z);
                let index_b = index1d_from_index3d(i, j, num_z - 1, num_x, num_y, num_z);
                self.data[index_a] = padding_value;
                self.data[index_b] = padding_value;
            }
        }
        for i in 0..self.n.i {
            for k in 0..self.n.k {
                let index_a = index1d_from_index3d(i, 0, k, num_x, num_y, num_z);
                let index_b = index1d_from_index3d(i, num_y - 1, k, num_x, num_y, num_z);
                self.data[index_a] = padding_value;
                self.data[index_b] = padding_value;
            }
        }
        for k in 0..self.n.k {
            for j in 0..self.n.j {
                let index_a = index1d_from_index3d(0, j, k, num_x, num_y, num_z);
                let index_b = index1d_from_index3d(num_x - 1, j, k, num_x, num_y, num_z);
                self.data[index_a] = padding_value;
                self.data[index_b] = padding_value;
            }
        }

        log::info!(
            "Dense value data with {} points padded in {:.2?}.",
            utils::math_helper::format_integer(self.num_points()),
            before.elapsed()
        );
    }

    /// Calculates the number of points needed in each direction based on bounds and cell size.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The extents of the field.
    /// * `cell_size` - The size of each cell in the field.
    ///
    /// # Returns
    ///
    /// A [`Vec3i`] containing the number of points needed in each direction.
    ///
    /// # Panics
    ///
    /// Panics if the bounds have a non-positive size in any direction.
    pub(crate) fn point_count(bounds: &BoundingBox<T>, cell_size: T) -> Vec3i {
        let (x_dim, y_dim, z_dim) = bounds.dimensions();
        Vec3i::new(
            (x_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to get point count from bounds. Make sure the bounds have a positive size in all directions.")
                + 1,
            (y_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to get point count from bounds. Make sure the bounds have a positive size in all directions.")
                + 1,
            (z_dim / cell_size)
                .floor()
                .to_usize()
                .expect("Failed to get point count from bounds. Make sure the bounds have a positive size in all directions.")
                + 1,
        )
    }
}

impl<T: ModelFloat + 'static> DenseField<T> {
    /// Evaluate the computation graph over a discretized domain.
    ///
    /// # Arguments
    ///
    /// * `graph` - The computation graph to evaluate.
    ///
    /// This method evaluates the computation graph at each point in the field in parallel.
    /// The results are stored in the field's data buffer.
    pub(crate) fn sample_from_graph(&mut self, graph: &ComputationGraph<T>) {
        let before = Instant::now();

        log::info!(
            "Evaluating model with {}x{}x{} points",
            self.n.i,
            self.n.j,
            self.n.k
        );

        let n = self.n;
        self.data
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, value)| {
                let (i, j, k) = index3d_from_index1d(index, n.i, n.j, n.k);
                *value = graph.evaluate_at_coord(
                    self.origin.x
                        + self.cell_size * T::from(i).expect("Failed to convert number to T"),
                    self.origin.y
                        + self.cell_size * T::from(j).expect("Failed to convert number to T"),
                    self.origin.z
                        + self.cell_size * T::from(k).expect("Failed to convert number to T"),
                );
            });

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            utils::math_helper::format_integer(n.product()),
            before.elapsed()
        );
    }

    /// Evaluate the computation graph over a discretized domain for a selection of specific point ids.
    ///
    /// # Arguments
    ///
    /// * `graph` - The computation graph to evaluate.
    /// * `ids` - The point ids to sample.
    ///
    /// This method evaluates the computation graph at each point in the field in parallel.
    /// The results are stored in the field's data buffer.
    pub(crate) fn sample_selected_from_graph(
        &mut self,
        graph: &ComputationGraph<T>,
        ids: &HashSet<usize>,
    ) {
        let before = Instant::now();

        log::debug!(
            "Evaluating {} points from model with {}x{}x{} points",
            ids.len(),
            self.n.i,
            self.n.j,
            self.n.k
        );

        let n = self.n;
        self.data
            .par_iter_mut()
            .enumerate()
            .filter(|(id, _)| ids.contains(id))
            .for_each(|(index, value)| {
                let (i, j, k) = index3d_from_index1d(index, n.i, n.j, n.k);
                *value = graph.evaluate_at_coord(
                    self.origin.x
                        + self.cell_size * T::from(i).expect("Failed to convert number to T"),
                    self.origin.y
                        + self.cell_size * T::from(j).expect("Failed to convert number to T"),
                    self.origin.z
                        + self.cell_size * T::from(k).expect("Failed to convert number to T"),
                );
            });

        log::debug!("{} points computed in {:.2?}", ids.len(), before.elapsed());
    }

    /// Performs a laplacian smoothing operation on the field data using parallel iteration.
    ///
    /// The value of each point will be updated based on the average of the adjacent points.
    /// This is a parallel version of the `smooth` method.
    ///
    /// # Arguments
    ///
    /// * `factor` - Interpolation value between the average of the adjacent points and the current value.
    /// * `iterations` - Number of successive smoothing iterations.
    pub fn smooth_par(&mut self, factor: T, iterations: u32) {
        let before = Instant::now();
        let mut smoothed = vec![T::zero(); self.num_points()];
        for _ in 0..iterations {
            smoothed
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, val)| {
                    if let Some(sum) = self.neighbours_sum(index) {
                        let laplacian = sum / T::from(6.0).expect("Failed to convert number to T");
                        *val = (T::one() - factor) * self.data[index] + factor * laplacian;
                    } else {
                        *val = self.data[index];
                    };
                });
            std::mem::swap(&mut self.data, &mut smoothed);
        }

        log::info!(
            "Dense value data with {} points smoothed in {:.2?} over {} iterations",
            utils::math_helper::format_integer(self.num_points()),
            before.elapsed(),
            iterations
        );
    }
}

impl<T: Float> PointIterator<T> for DenseField<T> {
    /// Returns an iterator that yields all point coordinates in the field.
    fn iter_points(&self) -> PointGridIter<T> {
        self.iter_grid()
    }

    type Iter<'a>
        = PointGridIter<T>
    where
        T: 'a;
}

impl<T: Float> GridIterator<T> for DenseField<T> {
    type GridIter<'a>
        = PointGridIter<T>
    where
        Self: 'a;

    /// Returns an iterator that yields all grid point coordinates in the field.
    fn iter_grid<'a>(&'a self) -> Self::GridIter<'a> {
        self.bounds.iter_point_grid(self.n)
    }
}

impl<T: Float> CellIterator<T> for DenseField<T> {
    type Iter<'a>
        = CellGridIter<T>
    where
        T: 'a;

    /// Returns an iterator that yields all cell coordinates in the field.
    fn iter_cells(&self) -> CellGridIter<T> {
        self.iter_cell_grid()
    }
}

impl<T: Float> CellGridIterator<T> for DenseField<T> {
    /// Returns an iterator that yields all cell coordinates in the field grid.
    fn iter_cell_grid(&self) -> CellGridIter<T> {
        CellGridIter::new(self.bounds, (self.n.i - 1, self.n.j - 1, self.n.k - 1))
    }

    type GridIter<'a>
        = CellGridIter<T>
    where
        Self: 'a;
}

impl<T: Float + 'static> ValueIterator<T> for DenseField<T> {
    type Iter<'a> = std::iter::Copied<std::slice::Iter<'a, T>>;

    /// Returns an iterator that yields each value in the field's data buffer.
    fn iter_values<'a>(&'a self) -> Self::Iter<'a> {
        self.data.iter().copied()
    }
}

impl<T: Float> CellValueIterator<T> for DenseField<T> {
    type Iter<'a>
        = DenseCellValueIterator<'a, T>
    where
        Self: 'a;

    /// Returns an iterator that yields the values at each cell's corners.
    fn iter_cell_values<'a>(&'a self) -> Self::Iter<'a> {
        DenseCellValueIterator {
            data: &self.data,
            current: (0, 0, 0),
            point_count: self.n.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth_field_half() {
        let mut data = vec![1.0; 27];
        data[13] = 2.0;
        let mut field = DenseField::from_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data).unwrap();
        field.smooth(0.5, 1);

        let field_data = field.copy_data();

        for (idx, val) in field_data.iter().enumerate() {
            if idx == 13 {
                assert!((1.5 - val).abs() < 0.001);
            } else {
                assert!((1.0 - val).abs() < 0.001);
            }
        }
    }

    #[test]
    fn test_smooth_field_full() {
        let mut data = vec![1.0; 27];
        data[13] = 2.0;
        let mut field = DenseField::from_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data).unwrap();
        field.smooth(1.0, 1);

        let field_data = field.copy_data();

        for val in field_data {
            assert!((1.0 - val).abs() < 0.001);
        }
    }

    #[test]
    fn test_smooth_field_full_varied() {
        let mut data = vec![0.0; 27];
        data[4] = 10.0;
        data[10] = 20.0;
        data[12] = 5.0;
        data[14] = 20.0;
        data[16] = 15.0;
        data[22] = 20.0;
        let mut field = DenseField::from_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data).unwrap();
        field.smooth(1.0, 1);

        let field_data = field.copy_data();
        assert!((15.0 - field_data[13]).abs() < 0.001);
    }

    #[test]
    fn test_smooth_field_half_varied() {
        let mut data = vec![0.0; 27];
        data[4] = 10.0;
        data[10] = 20.0;
        data[12] = 5.0;
        data[14] = 20.0;
        data[16] = 15.0;
        data[22] = 20.0;
        let mut field = DenseField::from_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data).unwrap();
        field.smooth(0.5, 1);

        let field_data = field.copy_data();
        assert!((7.5 - field_data[13]).abs() < 0.001);
    }

    #[test]
    fn test_threshold_field() {
        let mut data = vec![0.01; 27];
        data[13] = 2.0;
        data[20] = 1.0;
        data[21] = 1.5;

        let mut field = DenseField::from_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data).unwrap();
        field.threshold(0.1);

        let field_data = field.copy_data();

        assert_eq!(24, field_data.iter().filter(|&val| *val == 0.0).count());
        assert!((2.0 - field_data[13]).abs() < 0.001);
        assert!((1.0 - field_data[20]).abs() < 0.001);
        assert!((1.5 - field_data[21]).abs() < 0.001);
    }

    #[test]
    fn test_map_point_index() {
        let field = DenseField::new(Vec3::origin(), 1.0, (10, 10, 10).into());

        assert_eq!(1, field.point_index1d(1, 0, 0));
        assert_eq!(10, field.point_index1d(0, 1, 0));
        assert_eq!(11, field.point_index1d(1, 1, 0));
        assert_eq!(100, field.point_index1d(0, 0, 1));
        assert_eq!(110, field.point_index1d(0, 1, 1));
        assert_eq!(111, field.point_index1d(1, 1, 1));
    }

    #[test]
    fn test_value_iterator() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let field = DenseField::from_data(Vec3::origin(), 1.0, (2, 2, 2).into(), data).unwrap();

        let values: Vec<f64> = field.iter_values().collect();
        assert_eq!(values, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_cell_value_iterator() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0,
        ];
        let field = DenseField::from_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data).unwrap();

        let cell_values: Vec<[f64; 8]> = field.iter_cell_values().collect();
        assert_eq!(cell_values.len(), 8); // 2x2x2 cells in a 3x3x3 grid

        // Test first cell (0,0,0)
        assert_eq!(cell_values[0], field.cell_values(0, 0, 0));

        // Test last cell (1,1,1)
        assert_eq!(cell_values[7], field.cell_values(1, 1, 1));
    }
}
