# Shader View Systems

This module provides a complete 3D shader preview system for the WGSL Sorcery application. It creates a render-to-texture setup where a 3D scene with a sphere is rendered using custom WGSL shaders, and the result is displayed in an egui UI window.

## Overview

The shader view system consists of several interconnected components:

1. **3D Scene Setup** - Creates a sphere, camera, and lighting
2. **Render Target** - Renders the 3D scene to a texture instead of the screen
3. **UI Integration** - Displays the rendered texture in an egui window
4. **Shader Management** - Handles shader loading and hot-reloading

## Components and Resources

### ShaderView Resource
```rust
pub struct ShaderView {
    pub shader_handle: Handle<Shader>,
    pub mesh_entity: Option<Entity>,
    pub camera_entity: Option<Entity>,
    pub render_target_image: Handle<Image>,
}
```
This resource stores the state of the shader preview system:
- `shader_handle`: Handle to the currently loaded WGSL shader
- `mesh_entity`: Entity ID of the preview sphere
- `camera_entity`: Entity ID of the render-to-texture camera
- `render_target_image`: Handle to the texture where the 3D scene is rendered

### Component Markers
- `ShaderViewEntity`: Marks entities that are part of the shader preview scene
- `ShaderViewCamera`: Marks the camera used for rendering to texture

## Systems

### 1. setup_shader_view

**Purpose**: Initializes the complete 3D scene for shader preview.

**Functionality**:
- Creates a 512x512 render target texture with appropriate usage flags
- Spawns a sphere mesh with PBR material (blue color, non-metallic, medium roughness)
- Sets up lighting:
  - Point light positioned above the sphere
  - Directional light for overall illumination
  - Ambient light for better visibility
- Creates two cameras:
  - Main scene camera (order: 0) - renders to screen
  - Render-to-texture camera (order: -1) - renders to the target texture
- Loads the default WGSL shader from `../default.wgsl`
- Stores all entities and handles in the `ShaderView` resource

**Technical Details**:
- Uses `Sphere::new(1.0).mesh().ico(5)` for sphere geometry (5 subdivision levels)
- Camera positioned at (0, 1.5, 6) looking at origin
- Render target format: `Rgba8UnormSrgb`
- Texture usage includes binding, render attachment, and copy destination

### 2. apply_shader

**Purpose**: Ensures a shader is loaded and available.

**Current Implementation**: Placeholder system that maintains the default shader if no shader is currently loaded.

**Future Use**: Intended for applying custom shaders to the preview sphere based on user input or node graph output.

### 3. hot_reload_shaders

**Purpose**: Handles automatic shader reloading when shader files change.

**Current Implementation**: Basic placeholder that:
- Checks if the current shader is still valid
- Reloads the default shader if it was unloaded
- Logs debug information about shader status

**Future Implementation**: Should watch for file system changes and automatically reload modified WGSL files.

### 4. render_shader_preview

**Purpose**: Renders the shader preview UI window using egui.

**Functionality**:
- Skips the first 10 frames to allow egui to initialize properly
- Creates a resizable "Shader Preview" window (default 512x550)
- Displays the render target texture as an image
- Shows status information including:
  - Render target dimensions
  - Rendering status
  - Camera and lighting information

**UI Elements**:
- Title: "Shader Preview"
- Description: "WGSL Shader Preview" and "3D Sphere with PBR material"
- Rendered texture display (scales to fit window, max 512x512)
- Status panel with technical details

## Default Shader

The system uses `default.wgsl` as the initial shader:

### Vertex Shader
- Passes through position, normal, and UV coordinates
- Sets clip position directly from vertex position
- Outputs world position, normal, and UV for fragment shader

### Fragment Shader
- Creates a simple blue gradient based on UV coordinates
- Base color: RGB(0.2, 0.6, 1.0) with vertical gradient
- Outputs final color with alpha = 1.0

## Render Pipeline Flow

1. **Scene Setup**: `setup_shader_view` creates all entities and resources
2. **Render Pass 1** (order: -1): Render-to-texture camera renders the sphere with custom shader to the render target texture
3. **Render Pass 2** (order: 0): Main scene camera renders the rest of the application
4. **UI Pass**: `render_shader_preview` displays the render target texture in egui

## Integration Points

### With Node Graph
- The system is designed to receive shader code from the node graph
- Future implementations should connect node graph output to shader compilation

### With Input System
- Camera controls could be added for interactive shader preview
- Mouse interactions for shader parameter adjustment

## Configuration

### Render Target Settings
- Size: 512x512 pixels
- Format: Rgba8UnormSrgb (HDR-compatible)
- Usage: Texture binding, render attachment, copy destination

### Camera Settings
- Field of view: Default Bevy perspective
- Near/far planes: Default Bevy values
- Clear color: Dark blue-gray (0.1, 0.1, 0.15)

### Lighting Setup
- Point light: White, positioned at (0, 0, 10)
- Directional light: 15000 lux, 45-degree angle
- Ambient light: 30% brightness, white color

## Performance Considerations

- Render target is created once during setup
- Shader compilation happens during initialization
- UI rendering is optimized with egui's immediate mode GUI
- Frame skipping during initialization prevents UI flicker

## Future Enhancements

1. **Live Shader Editing**: Real-time shader compilation and application
2. **Multiple Meshes**: Support for different geometry types
3. **Shader Parameters**: UI controls for uniform variables
4. **Camera Controls**: Interactive camera movement and zoom
5. **Export Functionality**: Save rendered images or shader code
6. **Performance Metrics**: FPS and render time display
7. **Shader Presets**: Library of example shaders
8. **Error Handling**: Graceful shader compilation error display

## Dependencies

- `bevy`: Core game engine and ECS
- `bevy_egui`: UI framework for the preview window
- `bevy::render`: Rendering pipeline and resources
- `bevy::camera`: Camera components and systems

## Usage Example

To add this system to your Bevy app:

```rust
use bevy::prelude::*;
use crate::shader_view::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_systems(Startup, setup_shader_view)
        .add_systems(Update, (
            apply_shader,
            hot_reload_shaders,
            render_shader_preview,
        ))
        .run();
}
```

This will create a complete shader preview system with a 3D sphere rendered to texture and displayed in a resizable UI window.
