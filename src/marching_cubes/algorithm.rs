use crate::marching_cubes::types::core::*;
use super::{tables::{EDGE_TABLE, TRI_TABLE}, types::dense_grid::DenseGrid3f};

pub fn generate_iso_surface(grid : DenseGrid3f, iso_val: f32, coord: &Vec<XYZ>, values: &Vec<f32>) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();

    // Generate triangles for cell
    for i in 0..grid.num_x - 1 {
        for j in 0..grid.num_y - 1 {
            for k in 0..grid.num_z - 1 {
                polygonize_cell(iso_val, &grid.get_cell_ids(i, j, k), coord, values, &mut triangles);
            }
        }
    }
    

    triangles
}

fn polygonize_cell(
    iso_val: f32,
    cell_ids: &[usize; 8],
    coord: &Vec<XYZ>,
    values: &Vec<f32>,
    triangles: &mut Vec<Triangle>,
) -> i32 {
    let cell_coord = get_cell_coord(cell_ids, coord);
    let cell_values = get_cell_values(cell_ids, values);

    let cube_index = get_cube_index(cell_values, iso_val);
    get_triangles(cube_index, &cell_coord, &cell_values, iso_val, triangles)
}

fn get_cell_coord(cell_ids: &[usize; 8], coord: &Vec<XYZ>) -> [XYZ; 8] {
    [
        coord[cell_ids[0]],
        coord[cell_ids[1]],
        coord[cell_ids[2]],
        coord[cell_ids[3]],
        coord[cell_ids[4]],
        coord[cell_ids[5]],
        coord[cell_ids[6]],
        coord[cell_ids[7]],
    ]
}

fn get_cell_values(cell_ids: &[usize; 8], values: &Vec<f32>) -> [f32; 8] {
    [
        values[cell_ids[0]],
        values[cell_ids[1]],
        values[cell_ids[2]],
        values[cell_ids[3]],
        values[cell_ids[4]],
        values[cell_ids[5]],
        values[cell_ids[6]],
        values[cell_ids[7]],
    ]
}

fn get_cube_index(cell_values: [f32; 8], iso_val: f32) -> usize {
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
    cell_coord: &[XYZ; 8],
    cell_values: &[f32; 8],
    iso_val: f32,
    triangles: &mut Vec<Triangle>,
) -> i32 {
    let mut triangle_num = 0;

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
        triangle_num += 1;
    }
    triangle_num
}

fn get_vertices(
    cube_index: usize,
    cell_coord: &[XYZ; 8],
    cell_values: &[f32; 8],
    iso_val: f32,
) -> [XYZ; 12] {
    let mut vertices = [XYZ::get_origin(); 12];

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
    first_coord: XYZ,
    second_coord: XYZ,
    first_value: f32,
    second_value: f32,
) -> XYZ {
    const ISO_THRESHOLD: f32 = 0.00001;

    if (iso_val - first_value).abs() < ISO_THRESHOLD || (first_value - second_value).abs() < ISO_THRESHOLD
    {
        return first_coord;
    } else if (iso_val - second_value).abs() < ISO_THRESHOLD {
        return second_coord;
    }

    let parameter = (iso_val - first_value) / (second_value - first_value);
    XYZ::interpolate(&first_coord, &second_coord, parameter)
}
