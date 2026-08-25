struct Uniforms {
    angle: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    var pos = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5)
    );

    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.2, 0.2),
        vec3<f32>(0.2, 1.0, 0.2),
        vec3<f32>(0.2, 0.2, 1.0)
    );

    let p = pos[in_vertex_index];
    let theta = uniforms.angle;

    // Apply 2D rotation matrix formula:
    // x' = x * cos(θ) - y * sin(θ)
    // y' = x * sin(θ) + y * cos(θ)
    let rotated_x = p.x * cos(theta) - p.y * sin(theta);
    let rotated_y = p.x * sin(theta) + p.y * cos(theta);

    out.clip_position = vec4<f32>(rotated_x, rotated_y, 0.0, 1.0);
    out.color = colors[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
