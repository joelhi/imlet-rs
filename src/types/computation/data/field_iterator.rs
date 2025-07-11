use crate::{
    types::geometry::{BoundingBox, Vec3},
    utils::math_helper::index1d_from_index3d,
};
use num_traits::Float;

/// Trait for iterating over raw scalar values in a field.
///
/// This trait provides functionality to iterate over the individual values stored in a field,
/// regardless of their spatial arrangement.
pub trait ValueIterator<T> {
    /// A different `Iterator<Item=T>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = T>
    where
        Self: 'a;

    /// Returns an iterator that yields each value in the field.
    fn iter_values<'a>(&'a self) -> Self::Iter<'a>;
}

/// Trait for iterating over 3D points in a field.
///
/// This trait provides functionality to iterate over the spatial locations of points in a field,
/// returning their 3D coordinates.
pub trait PointIterator<T> {
    /// A different `Iterator<Item=Vec3<T>>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = Vec3<T>>
    where
        Self: 'a;

    /// Returns an iterator that yields the 3D coordinates of each point in the field.
    fn iter_points<'a>(&'a self) -> Self::Iter<'a>;
}

/// Trait for iterating over points in a regular 3D grid.
///
/// This trait extends [`PointIterator`] to provide specialized iteration over points
/// arranged in a regular grid pattern.
pub trait GridIterator<T>: PointIterator<T> {
    /// A different grid‐iterator for each borrow‐lifetime `'a`.
    type GridIter<'a>: Iterator<Item = Vec3<T>>
    where
        Self: 'a;

    /// Returns an iterator that yields the 3D coordinates of each point in the grid.
    fn iter_grid<'a>(&'a self) -> Self::GridIter<'a>;
}

/// Trait for iterating over the values at cell corners.
///
/// This trait provides functionality to iterate over cells in a field, returning
/// the values at the eight corners of each cell.
pub trait CellValueIterator<T> {
    /// A different `Iterator<Item=[T;8]>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = [T; 8]>
    where
        Self: 'a;

    /// Returns an iterator that yields an array of the eight corner values for each cell.
    fn iter_cell_values<'a>(&'a self) -> Self::Iter<'a>;
}

/// Trait for iterating over cell bounding boxes.
///
/// This trait provides functionality to iterate over cells in a field, returning
/// the bounding box of each cell.
pub trait CellIterator<T> {
    /// A different `Iterator<Item=BoundingBox<T>>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = BoundingBox<T>>
    where
        Self: 'a;

    /// Returns an iterator that yields the bounding box of each cell.
    fn iter_cells<'a>(&'a self) -> Self::Iter<'a>;
}

/// Trait for iterating over cells in a regular 3D grid.
///
/// This trait extends [`CellIterator`] to provide specialized iteration over cells
/// arranged in a regular grid pattern.
pub trait CellGridIterator<T>: CellIterator<T> {
    /// A different grid‐cell iterator for each borrow‐lifetime `'a`.
    type GridIter<'a>: Iterator<Item = BoundingBox<T>>
    where
        Self: 'a;

    /// Returns an iterator that yields the bounding box of each cell in the grid.
    fn iter_cell_grid<'a>(&'a self) -> Self::GridIter<'a>;
}

/// Iterator for traversing points in a regular 3D grid.
///
/// This struct provides efficient iteration over points arranged in a regular grid pattern,
/// computing their 3D coordinates based on the grid's bounds and point counts.
pub struct PointGridIter<T> {
    bounds: BoundingBox<T>,
    current: (usize, usize, usize),
    point_counts: (usize, usize, usize),
}

impl<T> PointGridIter<T> {
    /// Creates a new point grid iterator.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box containing all points.
    /// * `point_count` - The number of points in each dimension (x, y, z).
    pub fn new(bounds: BoundingBox<T>, point_count: (usize, usize, usize)) -> Self {
        Self {
            bounds,
            current: (0, 0, 0),
            point_counts: point_count,
        }
    }
}

impl<T: Float> Iterator for PointGridIter<T> {
    type Item = Vec3<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, k) = self.current;
        let (nx, ny, nz) = self.point_counts;

        if k >= nz {
            return None;
        }

        let size = self.bounds.dimensions();
        let dx = size.0 / T::from(nx - 1).unwrap();
        let dy = size.1 / T::from(ny - 1).unwrap();
        let dz = size.2 / T::from(nz - 1).unwrap();

        let x = self.bounds.min.x + T::from(i).unwrap() * dx;
        let y = self.bounds.min.y + T::from(j).unwrap() * dy;
        let z = self.bounds.min.z + T::from(k).unwrap() * dz;
        let pt = Vec3::new(x, y, z);

        // advance indices
        self.current.0 += 1;
        if self.current.0 >= nx {
            self.current.0 = 0;
            self.current.1 += 1;
            if self.current.1 >= ny {
                self.current.1 = 0;
                self.current.2 += 1;
            }
        }

        Some(pt)
    }
}

/// Iterator for traversing cells in a regular 3D grid.
///
/// This struct provides iteration over cells arranged in a regular grid pattern,
/// computing their bounding boxes based on the grid's bounds and cell counts.
pub struct CellGridIter<T> {
    bounds: BoundingBox<T>,
    current: (usize, usize, usize),
    cell_counts: (usize, usize, usize),
}

impl<T> CellGridIter<T> {
    /// Creates a new cell grid iterator.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The bounding box containing all cells.
    /// * `cell_count` - The number of cells in each dimension (x, y, z).
    pub fn new(bounds: BoundingBox<T>, cell_count: (usize, usize, usize)) -> Self {
        Self {
            bounds,
            current: (0, 0, 0),
            cell_counts: cell_count,
        }
    }
}

impl<T: Float> Iterator for CellGridIter<T> {
    type Item = BoundingBox<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, k) = self.current;
        let (nx, ny, nz) = self.cell_counts;

        if k >= nz {
            return None;
        }

        let size = self.bounds.dimensions();
        let dx = size.0 / T::from(nx).unwrap();
        let dy = size.1 / T::from(ny).unwrap();
        let dz = size.2 / T::from(nz).unwrap();

        let min_x = self.bounds.min.x + T::from(i).unwrap() * dx;
        let min_y = self.bounds.min.y + T::from(j).unwrap() * dy;
        let min_z = self.bounds.min.z + T::from(k).unwrap() * dz;

        let cell_bb = BoundingBox::new(
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(min_x + dx, min_y + dy, min_z + dz),
        );

        // advance indices
        self.current.0 += 1;
        if self.current.0 >= nx {
            self.current.0 = 0;
            self.current.1 += 1;
            if self.current.1 >= ny {
                self.current.1 = 0;
                self.current.2 += 1;
            }
        }

        Some(cell_bb)
    }
}

/// Iterator for traversing cell values in a dense field.
///
/// This struct provides efficient iteration over cell values in a dense field,
/// returning the eight corner values for each cell.
pub struct DenseCellValueIterator<'a, T> {
    pub(crate) data: &'a [T],
    pub(crate) current: (usize, usize, usize),
    pub(crate) point_count: (usize, usize, usize),
}

impl<'a, T> DenseCellValueIterator<'a, T> {
    /// Creates a new dense cell value iterator.
    ///
    /// # Arguments
    ///
    /// * `data` - The data buffer. The size must match the total number of points.
    /// * `point_count` - The number of points in each dimension (x, y, z).
    pub fn new(data: &'a [T], point_count: (usize, usize, usize)) -> Self {
        Self {
            data,
            current: (0, 0, 0),
            point_count,
        }
    }
}

impl<T: Float> DenseCellValueIterator<'_, T> {
    /// Returns the values at the eight corners of a cell.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first dimension.
    /// * `j` - Index in second dimension.
    /// * `k` - Index in third dimension.
    fn cell_values(&self, i: usize, j: usize, k: usize) -> [T; 8] {
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

    /// Returns the indices of the eight corners of a cell.
    ///
    /// # Arguments
    ///
    /// * `i` - Index in first dimension.
    /// * `j` - Index in second dimension.
    /// * `k` - Index in third dimension.
    fn cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        let (n_i, n_j, n_k) = self.point_count;
        [
            index1d_from_index3d(i, j, k, n_i, n_j, n_k),
            index1d_from_index3d(i + 1, j, k, n_i, n_j, n_k),
            index1d_from_index3d(i + 1, j + 1, k, n_i, n_j, n_k),
            index1d_from_index3d(i, j + 1, k, n_i, n_j, n_k),
            index1d_from_index3d(i, j, k + 1, n_i, n_j, n_k),
            index1d_from_index3d(i + 1, j, k + 1, n_i, n_j, n_k),
            index1d_from_index3d(i + 1, j + 1, k + 1, n_i, n_j, n_k),
            index1d_from_index3d(i, j + 1, k + 1, n_i, n_j, n_k),
        ]
    }
}

impl<'a, T: Float> Iterator for DenseCellValueIterator<'a, T> {
    type Item = [T; 8];

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, k) = self.current;
        let (nx, ny, nz) = self.point_count;

        if k >= nz - 1 {
            return None;
        }

        let values = self.cell_values(i, j, k);
        self.current.0 += 1;
        if self.current.0 >= nx - 1 {
            self.current.0 = 0;
            self.current.1 += 1;
            if self.current.1 >= ny - 1 {
                self.current.1 = 0;
                self.current.2 += 1;
            }
        }

        Some(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn assert_float_eq(a: f64, b: f64) {
        assert!((a - b).abs() < EPSILON, "Expected {b} but got {a}");
    }

    #[test]
    fn test_point_grid_iterator() {
        let bounds = BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let point_counts = (2, 2, 2);
        let iter = PointGridIter::new(bounds, point_counts);

        let points: Vec<Vec3<f64>> = iter.collect();

        // Should have 8 points total (2x2x2)
        assert_eq!(points.len(), 8);

        // Check first point (0,0,0)
        assert_float_eq(points[0].x, 0.0);
        assert_float_eq(points[0].y, 0.0);
        assert_float_eq(points[0].z, 0.0);

        // Check last point (1,1,1)
        assert_float_eq(points[7].x, 1.0);
        assert_float_eq(points[7].y, 1.0);
        assert_float_eq(points[7].z, 1.0);
    }

    #[test]
    fn test_cell_grid_iterator() {
        let bounds = BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let cell_counts = (1, 1, 1);
        let iter = CellGridIter::new(bounds, cell_counts);

        let cells: Vec<BoundingBox<f64>> = iter.collect();

        // Should have 1 cell (1x1x1)
        assert_eq!(cells.len(), 1);

        // Check cell bounds
        assert_float_eq(cells[0].min.x, 0.0);
        assert_float_eq(cells[0].min.y, 0.0);
        assert_float_eq(cells[0].min.z, 0.0);
        assert_float_eq(cells[0].max.x, 1.0);
        assert_float_eq(cells[0].max.y, 1.0);
        assert_float_eq(cells[0].max.z, 1.0);
    }

    #[test]
    fn test_dense_cell_value_iterator() {
        // Create a 2x2x2 grid of points (8 points total)
        let data = vec![
            0.0, 1.0, 2.0, 3.0, // z=0 layer
            4.0, 5.0, 6.0, 7.0, // z=1 layer
        ];
        let point_count = (2, 2, 2);
        let iter = DenseCellValueIterator::new(&data, point_count);

        let cell_values: Vec<[f64; 8]> = iter.collect();

        // Should have 1 cell (1x1x1)
        assert_eq!(cell_values.len(), 1);

        // Check cell corner values
        // The order should be:
        // z=0 layer: (0,0), (1,0), (1,1), (0,1)
        // z=1 layer: (0,0), (1,0), (1,1), (0,1)
        let expected = [0.0, 1.0, 3.0, 2.0, 4.0, 5.0, 7.0, 6.0];
        assert_eq!(cell_values[0], expected);
    }
}
