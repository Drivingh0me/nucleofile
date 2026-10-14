struct Uniforms {
    angle: f32,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let c = cos(uniforms.angle);
    let s = sin(uniforms.angle);
    let rotated_x = input.position.x * c - input.position.y * s;
    let rotated_y = input.position.x * s + input.position.y * c;

    out.clip_position = vec4<f32>(rotated_x, rotated_y, 0.0, 1.0);
    out.color = input.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
