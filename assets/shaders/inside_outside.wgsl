// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_location: vec3<f32>,
};
@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = model.normal;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) front_face: bool) -> @location(0) vec4<f32> {
    var light_dir = normalize(vec3<f32>(0.0, 1.0, 1.0));
    var projection = 0.5*(1.0 + dot(in.normal, light_dir));

    let max = 1.0;
    let min = 0.1;

    let num_steps: i32 = 7;

    let step_size: f32 = 1.0 / f32(num_steps);
    let step: i32 = i32(round((projection) / step_size));

    var color: f32 = f32(step) * step_size + step_size * 0.5;
    color = remap(color, 0.0, 1.0, min, max);

    if !front_face {
        color = max - color + min; 
        return vec4<f32>(0.05, 0.05, color, 1.0);
    }

    return vec4<f32>(color, 0.05, 0.05, 1.0);
}

fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let normalized_value = (value - from_min) / (from_max - from_min);
    return to_min + normalized_value * (to_max - to_min);
}