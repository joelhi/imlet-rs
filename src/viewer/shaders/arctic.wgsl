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
    var light_dir = normalize(vec3<f32>(0.0, 1.0, 1.0));
    var projection = 0.5*(1.0 + dot(in.normal, light_dir));

    // Define the clamp range
    let clamp_min: f32 = 0.15;
    let clamp_max: f32 = 0.85;

    // Define the number of discrete steps
    let num_steps: i32 = 20;

    // Calculate the step size based on the clamp range
    let step_size: f32 = (clamp_max - clamp_min) / f32(num_steps);

    // Calculate the adjusted projection value within the clamp range
    let adjusted_projection = clamp(projection, clamp_min, clamp_max);

    // Scale and round the adjusted projection value to obtain discrete steps
    let step: i32 = i32(round((adjusted_projection - clamp_min) / step_size));

    // Calculate the adjusted color based on the step value and clamp range
    var color: f32 = clamp_min + f32(step) * step_size + step_size * 0.5;

    // Invert the color for back-facing fragments
    if !front_face {
        color = clamp_max - color + clamp_min; // Reflect the color around the midpoint of the clamp range
        return vec4<f32>(color, color, color, 1.0);
    }

    // Output the color
    return vec4<f32>(color, color, color, 1.0);
}