#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

struct VignetteParams {
    intensity: f32,
    power: f32,
    _padding: vec2<f32>,
};

@group(2) @binding(0) var<uniform> params: VignetteParams;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 0.5 + vec2<f32>(0.5, 0.5);
    let dist = distance(uv, vec2<f32>(0.5, 0.5));
    let vignette = clamp(pow(dist, params.power) * params.intensity, 0.0, 1.0);
    return vec4<f32>(vec3<f32>(0.0), vignette);
}
