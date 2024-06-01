use std::time::Instant;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::types::computation::DenseFieldF32;
use crate::types::geometry::Triangle;
use crate::types::geometry::Vec3f;

use super::tables::*;

pub fn generate_iso_surface(field: &DenseFieldF32, iso_val: f32) -> Vec<Triangle> {
    let before = Instant::now();
    // Generate triangles for cell
    let mut triangles: Vec<Triangle> = Vec::with_capacity(field.get_num_cells() * 1);

    // Iterate over cell indices in parallel and collect triangles
    triangles.extend(
        (0..field.get_num_cells())
            .into_par_iter()
            .map(|cell_index| {
                let (i, j, k) = field.get_cell_index3d(cell_index);
                let cell_vec3f = field.get_cell_corners(i, j, k);
                let cell_values = field.get_cell_values(i, j, k);
                polygonize_cell(iso_val, &cell_vec3f, &cell_values)
            })
            .reduce(Vec::new, |mut acc, triangles| {
                acc.extend(triangles);
                acc
            }),
    );

    log::info!(
        "Marching cubes generated {} triangles in {:.2?}",
        triangles.len(),
        before.elapsed()
    );

    triangles
}

fn polygonize_cell(iso_val: f32, cell_coord: &[Vec3f; 8], cell_values: &[f32; 8]) -> Vec<Triangle> {
    let cube_index = get_cube_index(cell_values, iso_val);
    get_triangles(cube_index, &cell_coord, &cell_values, iso_val)
}

fn get_cube_index(cell_values: &[f32; 8], iso_val: f32) -> usize {
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

fn get_triangles(
    cube_index: usize,
    cell_coord: &[Vec3f; 8],
    cell_values: &[f32; 8],
    iso_val: f32,
) -> Vec<Triangle> {
    let mut triangles = Vec::new();

    let vertices = get_vertices(cube_index, cell_coord, cell_values, iso_val);
    for i in (0..15).step_by(3) {
        let tri = TRI_TABLE[cube_index][i];

        if tri == -1 {
            break;
        }

        triangles.push(Triangle {
            p1: vertices[TRI_TABLE[cube_index][i] as usize],
            p2: vertices[TRI_TABLE[cube_index][i + 1] as usize],
            p3: vertices[TRI_TABLE[cube_index][i + 2] as usize],
        });
    }
    triangles
}

fn get_vertices(
    cube_index: usize,
    cell_coord: &[Vec3f; 8],
    cell_values: &[f32; 8],
    iso_val: f32,
) -> [Vec3f; 12] {
    let mut vertices = [Vec3f::origin(); 12];

    if EDGE_TABLE[cube_index] == 0 {
        return vertices;
    }

    if EDGE_TABLE[cube_index] & 1 != 0 {
        vertices[0] = interpolate_vertex(
            iso_val,
            cell_coord[0],
            cell_coord[1],
            cell_values[0],
            cell_values[1],
        )
    }

    if EDGE_TABLE[cube_index] & 2 != 0 {
        vertices[1] = interpolate_vertex(
            iso_val,
            cell_coord[1],
            cell_coord[2],
            cell_values[1],
            cell_values[2],
        )
    }

    if EDGE_TABLE[cube_index] & 4 != 0 {
        vertices[2] = interpolate_vertex(
            iso_val,
            cell_coord[2],
            cell_coord[3],
            cell_values[2],
            cell_values[3],
        )
    }

    if EDGE_TABLE[cube_index] & 8 != 0 {
        vertices[3] = interpolate_vertex(
            iso_val,
            cell_coord[3],
            cell_coord[0],
            cell_values[3],
            cell_values[0],
        )
    }

    if EDGE_TABLE[cube_index] & 16 != 0 {
        vertices[4] = interpolate_vertex(
            iso_val,
            cell_coord[4],
            cell_coord[5],
            cell_values[4],
            cell_values[5],
        )
    }

    if EDGE_TABLE[cube_index] & 32 != 0 {
        vertices[5] = interpolate_vertex(
            iso_val,
            cell_coord[5],
            cell_coord[6],
            cell_values[5],
            cell_values[6],
        )
    }

    if EDGE_TABLE[cube_index] & 64 != 0 {
        vertices[6] = interpolate_vertex(
            iso_val,
            cell_coord[6],
            cell_coord[7],
            cell_values[6],
            cell_values[7],
        )
    }

    if EDGE_TABLE[cube_index] & 128 != 0 {
        vertices[7] = interpolate_vertex(
            iso_val,
            cell_coord[7],
            cell_coord[4],
            cell_values[7],
            cell_values[4],
        )
    }

    if EDGE_TABLE[cube_index] & 256 != 0 {
        vertices[8] = interpolate_vertex(
            iso_val,
            cell_coord[0],
            cell_coord[4],
            cell_values[0],
            cell_values[4],
        )
    }
    if EDGE_TABLE[cube_index] & 512 != 0 {
        vertices[9] = interpolate_vertex(
            iso_val,
            cell_coord[1],
            cell_coord[5],
            cell_values[1],
            cell_values[5],
        )
    }

    if EDGE_TABLE[cube_index] & 1024 != 0 {
        vertices[10] = interpolate_vertex(
            iso_val,
            cell_coord[2],
            cell_coord[6],
            cell_values[2],
            cell_values[6],
        )
    }

    if EDGE_TABLE[cube_index] & 2048 != 0 {
        vertices[11] = interpolate_vertex(
            iso_val,
            cell_coord[3],
            cell_coord[7],
            cell_values[3],
            cell_values[7],
        )
    }

    vertices
}

fn interpolate_vertex(
    iso_val: f32,
    first_coord: Vec3f,
    second_coord: Vec3f,
    first_value: f32,
    second_value: f32,
) -> Vec3f {
    const ISO_THRESHOLD: f32 = 0.00001;

    if (iso_val - first_value).abs() < ISO_THRESHOLD
        || (first_value - second_value).abs() < ISO_THRESHOLD
    {
        return first_coord;
    } else if (iso_val - second_value).abs() < ISO_THRESHOLD {
        return second_coord;
    }

    let parameter = (iso_val - first_value) / (second_value - first_value);
    Vec3f::interpolate(&first_coord, &second_coord, parameter)
}

#[cfg(test)]
mod tests {

    use crate::types::{computation::{distance_functions::Sphere, Model}, geometry::BoundingBox};

    use super::*;

    #[test]
    fn test_generate_iso_surface_2x2x2() {
        let field = DenseFieldF32::with_data(
            Vec3f::origin(),
            1.0,
            (2, 2, 2).into(),
            vec![1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
        );
        let triangles = generate_iso_surface(&field, 0.0);

        assert_eq!(2, triangles.len());
        for tri in triangles {
            assert!(tri.p1.z - 0.5 < 0.0001);
            assert!(tri.p2.z - 0.5 < 0.0001);
            assert!(tri.p3.z - 0.5 < 0.0001);
            assert!(tri.compute_area() - 0.5 < 0.0001);
        }
    }

    #[test]
    fn test_generate_iso_surface_3x2x2() {
        let field = DenseFieldF32::with_data(
            Vec3f::origin(),
            1.0,
            (3, 2, 2).into(),
            vec![
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            ],
        );
        let triangles = generate_iso_surface(&field, 0.0);

        assert_eq!(4, triangles.len());
        for tri in triangles {
            assert!(tri.p1.z - 0.5 < 0.0001);
            assert!(tri.p2.z - 0.5 < 0.0001);
            assert!(tri.p3.z - 0.5 < 0.0001);
            assert!(tri.compute_area() - 0.5 < 0.0001);
        }
    }

    #[test]
    fn test_generate_iso_surface_sphere() {
        let size = 10.0;
        let cell_size = 0.5;
        let bounds = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

        // Function
        let mut model = Model::new();
        let sphere = model.add_function(Sphere::new(
            Vec3f::new(size / 2.0, size / 2.0, size / 2.0),
            size * 0.4,
        ));

        let field = model.evaluate(bounds, cell_size, sphere);

        // Generate mesh
        let triangles = generate_iso_surface(&field, 0.0);

        let area: f32 = triangles.iter().map(|tri| tri.compute_area()).sum();

        assert!(200.079 - area < 0.1);
        assert_eq!(2312, triangles.len());
    }
}
