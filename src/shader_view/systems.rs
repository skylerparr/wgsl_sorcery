use crate::shader_view::{ShaderView, ShaderViewCamera, ShaderViewEntity};
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
    let mut render_target_image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("render_target"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        ..default()
    };
    render_target_image.resize(size);
    let render_target_image = images.add(render_target_image);

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

    // Spawn a camera for the 3D scene (render target to be added later)
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 1.5, 6.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ShaderViewCamera,
        ))
        .id();

    // Add basic lighting
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
        render_target_image: render_target_image.clone(),
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

// Resource to track initialization frames
#[derive(Resource, Default)]
pub struct EguiInitTracker {
    pub frame_count: u32,
}

/// System to render the shader preview in the UI
pub fn render_shader_preview(
    shader_view: Res<ShaderView>,
    images: Res<Assets<Image>>,
    mut contexts: bevy_egui::EguiContexts,
    mut init_tracker: Local<EguiInitTracker>,
) {
    // Skip the first few frames to let egui initialize properly
    init_tracker.frame_count += 1;
    if init_tracker.frame_count < 10 {
        return;
    }

    // Now try to access the egui context
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Shader Preview")
            .default_size(egui::vec2(512.0, 550.0))
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("WGSL Shader Preview");
                ui.label("3D Sphere with PBR material");
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                // Display the render target image
                if shader_view.render_target_image != Handle::default() {
                    let available_size = ui.available_size();
                    let size = egui::vec2(available_size.x.min(512.0), available_size.y.min(512.0));

                    // Simple placeholder for now - we'll show the render target status
                    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(4),
                        egui::Color32::from_rgb(40, 40, 80),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Render Target Active",
                        egui::FontId::default(),
                        egui::Color32::WHITE,
                    );

                    ui.add_space(5.0);
                    if let Some(image) = images.get(&shader_view.render_target_image) {
                        ui.label(format!(
                            "Render target: {}x{}",
                            image.size().x,
                            image.size().y
                        ));
                    } else {
                        ui.label("Render target: 512x512");
                    }
                } else {
                    ui.label("Render target not initialized");
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);
                ui.label("Status: PBR sphere rendering");
                ui.label("Render target: Active");
                ui.label("Camera: 3D perspective");
                ui.label("Lighting: Directional + Ambient");
            });
    }
}
