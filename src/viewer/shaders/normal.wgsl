// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>
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
    var light_dir = normalize(vec3<f32>(0.0, -1.0, -1.0));
    var projection = 0.5*(1.0 + dot(in.normal, light_dir));

    let max = 0.85;
    let min = 0.3;

    // Define the number of discrete steps
    let num_steps: i32 = 10;

    // Calculate the step size based on the clamp range
    let step_size: f32 = 1.0 / f32(num_steps);

    // Scale and round the adjusted projection value to obtain discrete steps
    let step: i32 = i32(round((projection) / step_size));

    // Calculate the adjusted color based on the step value and clamp range
    var shade: f32 = f32(step) * step_size + step_size * 0.5;
    shade = remap(shade, 0.0, 1.0, min, max);

    // Invert the color for back-facing fragments
    if !front_face {
        shade = max - shade + min; // Reflect the color around the midpoint of the clamp range
    }

    // Output the color
    return vec4<f32>(shade * 0.5*(1.0 + in.normal[0]), shade * 0.5*(1.0 + in.normal[1]), shade * 0.5*(1.0 + in.normal[2]), 1.0);
}

fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // Normalize the value to the [0, 1] range within the input domain
    let normalized_value = (value - from_min) / (from_max - from_min);
    
    // Map the normalized value to the output domain
    let remapped_value = to_min + normalized_value * (to_max - to_min);
    
    return remapped_value;
}