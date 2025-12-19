use bevy::prelude::*;

/// Resource that stores the shader view state
#[derive(Resource)]
pub struct ShaderView {
    pub shader_handle: Handle<Shader>,
    pub mesh_entity: Option<Entity>,
    pub camera_entity: Option<Entity>,
    pub render_target_image: Handle<Image>,
}

impl Default for ShaderView {
    fn default() -> Self {
        Self {
            shader_handle: Handle::default(),
            mesh_entity: None,
            camera_entity: None,
            render_target_image: Handle::default(),
        }
    }
}

/// Component to mark entities as part of the shader view
#[derive(Component)]
pub struct ShaderViewEntity;

/// Component to mark the camera for the shader view
#[derive(Component)]
pub struct ShaderViewCamera;
