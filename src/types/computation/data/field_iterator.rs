use crate::{
    types::geometry::{BoundingBox, Vec3},
    utils::math_helper::index1d_from_index3d,
};
use num_traits::Float;

/// Iterate raw scalar values.
pub trait ValueIterator<T> {
    /// A different `Iterator<Item=T>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = T>
    where
        Self: 'a;

    fn iter_values<'a>(&'a self) -> Self::Iter<'a>;
}

/// Iterate 3D points.
pub trait PointIterator<T> {
    /// A different `Iterator<Item=Vec3<T>>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = Vec3<T>>
    where
        Self: 'a;

    fn iter_points<'a>(&'a self) -> Self::Iter<'a>;
}

/// Iterate points in a regular 3D grid.
pub trait GridIterator<T>: PointIterator<T> {
    /// A different grid‐iterator for each borrow‐lifetime `'a`.
    type GridIter<'a>: Iterator<Item = Vec3<T>>
    where
        Self: 'a;

    fn iter_grid<'a>(&'a self) -> Self::GridIter<'a>;
}

/// Iterate the eight corner‐values of each cell.
pub trait CellValueIterator<T> {
    /// A different `Iterator<Item=[T;8]>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = [T; 8]>
    where
        Self: 'a;

    fn iter_cell_values<'a>(&'a self) -> Self::Iter<'a>;
}

/// Iterate the bounding‐boxes of individual cells.
pub trait CellIterator<T> {
    /// A different `Iterator<Item=BoundingBox<T>>` for each borrow‐lifetime `'a`.
    type Iter<'a>: Iterator<Item = BoundingBox<T>>
    where
        Self: 'a;

    fn iter_cells<'a>(&'a self) -> Self::Iter<'a>;
}

/// Iterate cell‐grid bounding‐boxes in a regular 3D grid.
pub trait CellGridIterator<T>: CellIterator<T> {
    /// A different grid‐cell iterator for each borrow‐lifetime `'a`.
    type GridIter<'a>: Iterator<Item = BoundingBox<T>>
    where
        Self: 'a;

    fn iter_cell_grid<'a>(&'a self) -> Self::GridIter<'a>;
}

/// Helper struct for grid‐based point iteration.
pub struct PointGridIter<T> {
    bounds: BoundingBox<T>,
    current: (usize, usize, usize),
    point_counts: (usize, usize, usize),
}

impl<T> PointGridIter<T> {
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

/// Helper struct for cell‐grid iteration.
pub struct CellGridIter<T> {
    bounds: BoundingBox<T>,
    current: (usize, usize, usize),
    cell_counts: (usize, usize, usize),
}

impl<T> CellGridIter<T> {
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

/// Iterator over cell values in a dense field
pub struct DenseCellValueIterator<'a, T: Float> {
    pub(crate) data: &'a [T],
    pub(crate) current: (usize, usize, usize),
    pub(crate) point_count: (usize, usize, usize),
}

impl<T: Float> DenseCellValueIterator<'_, T> {
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
