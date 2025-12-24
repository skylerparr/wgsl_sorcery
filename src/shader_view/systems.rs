use crate::shader_view::{ShaderView, ShaderViewCamera, ShaderViewEntity};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy_egui::egui;

/// System to set up the shader view 3D scene
pub fn setup_shader_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut images: ResMut<Assets<Image>>,
    _asset_server: Res<AssetServer>,
) {
    info!("Setting up shader view 3D scene");

    // Create a render target image for the shader preview
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };
    // Create a simple sphere mesh using basic shape
    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    // Create a basic material for the sphere
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.7, 1.0),
        metallic: 0.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    // Create a proper 3D entity with PBR material
    let sphere_entity = commands
        .spawn((
            Mesh3d(sphere_mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ShaderViewEntity,
        ))
        .id();

    commands.spawn((
        PointLight::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        RenderLayers::layer(0).with(1),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            // render before the main pass cameras
            order: -1,
            clear_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
    ));

    // Spawn a camera for the 3D scene that renders to the texture
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Camera {
                // render before the main pass cameras
                order: 0,
                clear_color: Color::srgb(0.1, 0.1, 0.15).into(),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 1.5, 6.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ShaderViewCamera,
        ))
        .id();

    // Add lighting for the render-to-texture scene
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
    ));

    // Add ambient light for better visibility
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        affects_lightmapped_meshes: false,
    });

    // Load default shader (for future use with custom materials)
    let default_shader: Handle<Shader> = shaders.add(Shader::from_wgsl(
        include_str!("../default.wgsl"),
        "default.wgsl",
    ));

    // Store in a view state resource
    commands.insert_resource(ShaderView {
        shader_handle: default_shader.clone(),
        mesh_entity: Some(sphere_entity),
        camera_entity: Some(camera_entity),
    });

    info!("Shader view setup complete with 3D sphere and render target");
}

/// System to apply custom shader to sphere
pub fn apply_shader(
    mut shader_view: ResMut<ShaderView>,
    mut shaders: ResMut<Assets<Shader>>,
    _asset_server: Res<AssetServer>,
) {
    // This system will be used to update the shader when needed
    // For now, it ensures the shader is loaded
    if shader_view.shader_handle == Handle::default() {
        shader_view.shader_handle = shaders.add(Shader::from_wgsl(
            include_str!("../default.wgsl"),
            "default.wgsl",
        ));
    }
}

/// System to handle shader hot reload
pub fn hot_reload_shaders(
    mut shader_view: ResMut<ShaderView>,
    mut shaders: ResMut<Assets<Shader>>,
    _asset_server: Res<AssetServer>,
) {
    // This is a placeholder for hot reload functionality
    // In a real implementation, this would watch for file changes and reload shaders
    // For now, we'll just ensure the shader stays loaded
    if let Some(_shader) = shaders.get(&shader_view.shader_handle) {
        // Shader is loaded and valid
        debug!("Shader is loaded: {:?}", shader_view.shader_handle);
    } else {
        // Reload the shader if it was unloaded
        info!("Reloading shader");
        shader_view.shader_handle = shaders.add(Shader::from_wgsl(
            include_str!("../default.wgsl"),
            "default.wgsl",
        ));
    }
}
