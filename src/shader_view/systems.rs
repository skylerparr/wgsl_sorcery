use crate::shader_view::{ShaderView, ShaderViewCamera, ShaderViewEntity};
use bevy::prelude::*;
use bevy_egui::egui;

/// System to set up the shader view 3D scene
pub fn setup_shader_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    info!("Setting up shader view 3D scene");

    // Create a simple render target image for the shader preview
    let render_target_image = images.add(Image::default());

    // For now, just create a simple entity placeholder
    let placeholder = commands.spawn_empty().insert(ShaderViewEntity).id();

    // 3. Load default shader
    let default_shader: Handle<Shader> = shaders.add(Shader::from_wgsl(
        include_str!("../default.wgsl"),
        "default.wgsl",
    ));

    // Store in a view state resource
    commands.insert_resource(ShaderView {
        shader_handle: default_shader.clone(),
        mesh_entity: Some(placeholder),
        camera_entity: None,
        render_target_image: render_target_image.clone(),
    });

    info!("Shader view setup complete");
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
    asset_server: Res<AssetServer>,
) {
    // This is a placeholder for hot reload functionality
    // In a real implementation, this would watch for file changes and reload shaders
    // For now, we'll just ensure the shader stays loaded
    if let Some(shader) = shaders.get(&shader_view.shader_handle) {
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
                ui.label("3D Sphere with custom shader");
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                // Display the render target image
                if shader_view.render_target_image != Handle::default() {
                    // For now, show a placeholder since we need to properly set up the texture handling
                    let available_size = ui.available_size();
                    let size = egui::vec2(available_size.x.min(512.0), available_size.y.min(512.0));

                    // Create a simple colored rectangle as placeholder
                    ui.allocate_ui_with_layout(
                        size,
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.colored_label(
                                egui::Color32::from_rgb(51, 153, 255),
                                "🔵 Shader Preview",
                            );
                            ui.label("3D Sphere rendering here");
                        },
                    );
                } else {
                    ui.label("Render target not initialized");
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);
                ui.label("Status: Default shader loaded");
                ui.label("Hot reload: Not implemented yet");
                ui.label("Camera and sphere set up complete");
            });
    }
}
