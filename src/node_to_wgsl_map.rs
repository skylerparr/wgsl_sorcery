pub static NODE_WGSL_MAP: &[(&str, &str)] = &[
    ("Add", "fn add_node(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> { return a + b; }"),
    ("Multiply", "fn mul_node(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> { return a * b; }"),
    ("ColorRamp", "fn color_ramp(t: f32, colors: array<vec4<f32>, 4>) -> vec4<f32> { /* impl */ }"),
    // ... all your node types
];