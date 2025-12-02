// Optimized shaders for Apple Silicon (Metal backend)
//
// Optimization notes:
// - All structs use 16-byte aligned fields where possible
// - No divergent control flow (uniform performance across pixels)
// - Texture sampling uses hardware filtering (efficient on Apple GPUs)
// - FMA (fused multiply-add) operations are implicit in texture sampling
//
// Metal-specific considerations:
// - vec2/vec3 fields are aligned to their component size
// - vec4 fields are 16-byte aligned
// - TBDR architecture benefits from simple fragment shaders

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.uv = model.uv;
    // Convert from pixel coordinates to clip space
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

// Texture bindings
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_texture(in: VertexOutput) -> @location(0) vec4<f32> {
    // Multiply by color for tinting support
    // Hardware filtering and FMA operations are handled by GPU
    return textureSample(t_diffuse, s_diffuse, in.uv) * in.color;
}
