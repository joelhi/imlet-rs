struct DataBuf {
    data: array<f32>,
}

@group(0)
@binding(0)
var<storage, read_write> v_indices: DataBuf;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // TODO: a more interesting computation than this.
    v_indices.data[global_id.x] = v_indices.data[global_id.x] + 42.0;
}