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
    @location(1) view_dir: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let expansion = 1.0;
    let smoothness = 0.1; // Adjust the smoothness factor

    var out: VertexOutput;
    out.normal = model.normal;
    out.view_dir = normalize(camera.camera_location - model.position);

    out.clip_position = camera.view_proj * vec4<f32>(
        model.position,
        1.0
    );

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) front_face: bool) -> @location(0) vec4<f32> {
    // Outline
    var view_dir_proj = dot(in.view_dir, in.normal);

    var proj = abs(view_dir_proj);

    var outline_width = clamp(1.0 - abs(view_dir_proj), 0.0, 0.2);
    var smooth_outline_width = smootherstep(0.0, 1.0, outline_width);

    var outline_threshold = 0.3;
    var outline = step(outline_threshold, smooth_outline_width);

    var shade = mix(proj, 0.0, outline);

    let final_shade = shade;
    // Output the color
    return vec4<f32>(final_shade - 0.05, final_shade, final_shade + 0.05, 1.0);
}

fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // Normalize the value to the [0, 1] range within the input domain
    let normalized_value = (value - from_min) / (from_max - from_min);
    
    // Map the normalized value to the output domain
    let remapped_value = to_min + normalized_value * (to_max - to_min);
    
    return remapped_value;
}

fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    var t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}