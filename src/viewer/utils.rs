use crate::types::geometry::Vec3;

pub fn convert_vec3_to_raw<T: Copy>(vec: &Vec<Vec3<T>>) -> Vec<[T; 3]> {
    let len = vec.len();
    let mut new_vec = Vec::<[T; 3]>::with_capacity(len);

    unsafe {
        std::ptr::copy_nonoverlapping(vec.as_ptr(), new_vec.as_mut_ptr() as *mut Vec3<T>, len);

        new_vec.set_len(len);
    }

    new_vec
}

pub fn faces_as_flat_u32(vec: &Vec<[usize; 3]>) -> Vec<u32> {
    // Create a new Vec<u32> with 3 * vec.len() capacity
    let mut flat_vec = Vec::with_capacity(3 * vec.len());

    for arr in vec {
        // Convert each usize element to u32 and push it to the new vector
        flat_vec.push(arr[0] as u32);
        flat_vec.push(arr[1] as u32);
        flat_vec.push(arr[2] as u32);
    }

    flat_vec
}
