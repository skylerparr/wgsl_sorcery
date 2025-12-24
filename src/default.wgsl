// default.wgsl - Basic WGSL shader for sphere rendering
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};

// Camera and transform uniforms
struct CameraUniforms {
    view_proj: mat4x4<f32>,
};

struct ModelUniforms {
    model: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@group(0) @binding(1)
var<uniform> model: ModelUniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply model transformation to get world position
    let world_pos = model.model * vec4<f32>(vertex.position, 1.0);
    out.world_position = world_pos.xyz;
    
    // Transform normal to world space (using only the rotation part of model matrix)
    out.world_normal = (model.model * vec4<f32>(vertex.normal, 0.0)).xyz;
    
    // Apply view-projection transformation to get clip position
    out.clip_position = camera.view_proj * world_pos;
    
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    
    // Simple gradient based on UV coordinates with blue base color
    let base_color = vec3<f32>(0.2, 0.6, 1.0);
    let gradient = input.uv.y * 0.3;
    out.color = vec4<f32>(base_color + gradient, 1.0);
    
    return out;
}
