use std::time::Instant;

use num_traits::Float;

use crate::types::computation::data::field_iterator::CellIterator;
use crate::types::computation::data::field_iterator::CellValueIterator;
use crate::types::geometry::Triangle;
use crate::types::geometry::Vec3;
use crate::utils;

use super::tables::*;

/// Generate a list of triangles from a field using the marching cubes algorithm.
///
/// This function is based on the logic by [Paul Bourke](https://paulbourke.net/geometry/polygonise/).
///
/// # Arguments
///
/// * `field` - The field from which the iso surface should be generated.
/// * `iso_val` - The target iso value.
pub fn generate_iso_surface<T, F>(field: &F, iso_val: T) -> Vec<Triangle<T>>
where
    T: Float,
    F: CellIterator<T> + CellValueIterator<T>,
{
    let before = Instant::now();
    // Generate triangles for cell
    let mut triangles: Vec<Triangle<T>> = Vec::new();

    // Iterate over cell indices in parallel and collect triangles
    for (cell_bounds, cell_values) in field.iter_cells().zip(field.iter_cell_values()) {
        let cell_coord = cell_bounds.corners();
        triangles.extend(polygonize_cell(iso_val, &cell_coord, &cell_values));
    }

    log::info!(
        "Marching cubes generated {} triangles in {:.2?}",
        utils::math_helper::format_integer(triangles.len()),
        before.elapsed()
    );

    triangles
}

fn polygonize_cell<T: Float>(
    iso_val: T,
    cell_coord: &[Vec3<T>; 8],
    cell_values: &[T; 8],
) -> Vec<Triangle<T>> {
    let cube_index = cube_index(cell_values, iso_val);
    triangles(cube_index, cell_coord, cell_values, iso_val)
}

fn cube_index<T: Float>(cell_values: &[T; 8], iso_val: T) -> usize {
    let mut cube_index: usize = 0;

    if cell_values[0] < iso_val {
        cube_index |= 1;
    }
    if cell_values[1] < iso_val {
        cube_index |= 2;
    }
    if cell_values[2] < iso_val {
        cube_index |= 4;
    }
    if cell_values[3] < iso_val {
        cube_index |= 8;
    }
    if cell_values[4] < iso_val {
        cube_index |= 16;
    }
    if cell_values[5] < iso_val {
        cube_index |= 32;
    }
    if cell_values[6] < iso_val {
        cube_index |= 64;
    }
    if cell_values[7] < iso_val {
        cube_index |= 128;
    }

    cube_index
}

fn triangles<T: Float>(
    cube_index: usize,
    cell_coord: &[Vec3<T>; 8],
    cell_values: &[T; 8],
    iso_val: T,
) -> Vec<Triangle<T>> {
    let mut triangles = Vec::new();

    let vertices = vertices(cube_index, cell_coord, cell_values, iso_val);
    for i in (0..15).step_by(3) {
        let tri = TRI_TABLE[cube_index][i];

        if tri == -1 {
            break;
        }

        triangles.push(Triangle::new(
            vertices[TRI_TABLE[cube_index][i] as usize],
            vertices[TRI_TABLE[cube_index][i + 1] as usize],
            vertices[TRI_TABLE[cube_index][i + 2] as usize],
        ));
    }
    triangles
}

fn vertices<T: Float>(
    cube_index: usize,
    cell_coord: &[Vec3<T>; 8],
    cell_values: &[T; 8],
    iso_val: T,
) -> [Vec3<T>; 12] {
    let mut vertices = [Vec3::origin(); 12];

    if EDGE_TABLE[cube_index] == 0 {
        return vertices;
    }

    if EDGE_TABLE[cube_index] & 1 != 0 {
        vertices[0] = interpolate_vertex(
            iso_val,
            &cell_coord[0],
            &cell_coord[1],
            cell_values[0],
            cell_values[1],
        )
    }

    if EDGE_TABLE[cube_index] & 2 != 0 {
        vertices[1] = interpolate_vertex(
            iso_val,
            &cell_coord[1],
            &cell_coord[2],
            cell_values[1],
            cell_values[2],
        )
    }

    if EDGE_TABLE[cube_index] & 4 != 0 {
        vertices[2] = interpolate_vertex(
            iso_val,
            &cell_coord[2],
            &cell_coord[3],
            cell_values[2],
            cell_values[3],
        )
    }

    if EDGE_TABLE[cube_index] & 8 != 0 {
        vertices[3] = interpolate_vertex(
            iso_val,
            &cell_coord[3],
            &cell_coord[0],
            cell_values[3],
            cell_values[0],
        )
    }

    if EDGE_TABLE[cube_index] & 16 != 0 {
        vertices[4] = interpolate_vertex(
            iso_val,
            &cell_coord[4],
            &cell_coord[5],
            cell_values[4],
            cell_values[5],
        )
    }

    if EDGE_TABLE[cube_index] & 32 != 0 {
        vertices[5] = interpolate_vertex(
            iso_val,
            &cell_coord[5],
            &cell_coord[6],
            cell_values[5],
            cell_values[6],
        )
    }

    if EDGE_TABLE[cube_index] & 64 != 0 {
        vertices[6] = interpolate_vertex(
            iso_val,
            &cell_coord[6],
            &cell_coord[7],
            cell_values[6],
            cell_values[7],
        )
    }

    if EDGE_TABLE[cube_index] & 128 != 0 {
        vertices[7] = interpolate_vertex(
            iso_val,
            &cell_coord[7],
            &cell_coord[4],
            cell_values[7],
            cell_values[4],
        )
    }

    if EDGE_TABLE[cube_index] & 256 != 0 {
        vertices[8] = interpolate_vertex(
            iso_val,
            &cell_coord[0],
            &cell_coord[4],
            cell_values[0],
            cell_values[4],
        )
    }
    if EDGE_TABLE[cube_index] & 512 != 0 {
        vertices[9] = interpolate_vertex(
            iso_val,
            &cell_coord[1],
            &cell_coord[5],
            cell_values[1],
            cell_values[5],
        )
    }

    if EDGE_TABLE[cube_index] & 1024 != 0 {
        vertices[10] = interpolate_vertex(
            iso_val,
            &cell_coord[2],
            &cell_coord[6],
            cell_values[2],
            cell_values[6],
        )
    }

    if EDGE_TABLE[cube_index] & 2048 != 0 {
        vertices[11] = interpolate_vertex(
            iso_val,
            &cell_coord[3],
            &cell_coord[7],
            cell_values[3],
            cell_values[7],
        )
    }

    vertices
}

fn interpolate_vertex<T: Float>(
    iso_val: T,
    first_coord: &Vec3<T>,
    second_coord: &Vec3<T>,
    first_value: T,
    second_value: T,
) -> Vec3<T> {
    let iso_threshold =
        T::from(0.00001).expect("Should be able to convert number into generic type.");

    if (iso_val - first_value).abs() < iso_threshold
        || (first_value - second_value).abs() < iso_threshold
    {
        return *first_coord;
    } else if (iso_val - second_value).abs() < iso_threshold {
        return *second_coord;
    }

    let parameter = (iso_val - first_value) / (second_value - first_value);
    Vec3::interpolate(first_coord, second_coord, parameter)
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::data::DenseField,
        geometry::{traits::SignedDistance, Sphere, Vec3},
    };

    use super::*;

    #[test]
    fn test_generate_iso_surface_2x2x2() {
        let field = DenseField::from_data(
            Vec3::origin(),
            1.0,
            (2, 2, 2).into(),
            vec![1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
        )
        .unwrap();
        let triangles = generate_iso_surface(&field, 0.0);

        assert_eq!(2, triangles.len());
        for tri in triangles {
            assert!(tri.p1().z - 0.5 < 0.0001);
            assert!(tri.p2().z - 0.5 < 0.0001);
            assert!(tri.p3().z - 0.5 < 0.0001);
            assert!(tri.compute_area() - 0.5 < 0.0001);
        }
    }

    #[test]
    fn test_generate_iso_surface_sphere() {
        let mut field = DenseField::new(Vec3::origin(), 1.0, (10, 10, 10).into());
        let sphere = Sphere::new(Vec3::new(5.0, 5.0, 5.0), 4.0);

        // Sample field
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    let point = Vec3::new(
                        f64::from(i as i32),
                        f64::from(j as i32),
                        f64::from(k as i32),
                    );
                    let index = field.point_index1d(i, j, k);
                    field.set_value(index, sphere.signed_distance(point.x, point.y, point.z));
                }
            }
        }

        let triangles = generate_iso_surface(&field, 0.0);
        assert!(!triangles.is_empty());
    }
}
