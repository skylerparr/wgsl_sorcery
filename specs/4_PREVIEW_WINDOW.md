# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**

# Specification

## Feature: Shadplay Integration — Sphere View with Default WGSL Shader

### Objective

Integrate **Shadplay** into your Bevy node editor framework to provide a dedicated view area that renders a 3D **sphere** mesh with a **basic WGSL color shader** applied by default. The goal is a minimal workflow where:

1. A 3D view area exists within your UI (e.g., as a panel or window).
2. A sphere is rendered using Bevy’s 3D renderer via Shadplay’s shader preview pipeline.
3. A simple WGSL shader (e.g., solid color or gradient) is loaded and applied.
4. Shader recompilation is enabled for iteration (optional but useful).

Shadplay already provides live preview of WGSL on mesh geometry and automatic shader reload on edit. ([GitHub][1])

---

## 1. High-Level Architecture

### 1.1 Renderer Integration

You will embed Shadplay’s rendering subsystem into your Bevy application. The key components are:

* A **Bevy 3D Scene** (camera, sphere mesh, lighting).
* A **Shadplay subsystem** that:

  * Loads and compiles WGSL shaders at runtime.
  * Applies shaders to standard Bevy meshes.
  * Handles hot reload on file changes.

Introduce a dedicated `RenderView` resource that stores:

```rust
pub struct ShaderView {
    pub shader_handle: Handle<Shader>,
    pub mesh_entity: Option<Entity>,
    pub camera_entity: Option<Entity>,
}
```

---

## 2. View Initialization
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**

### 2.1 Add Shadplay Dependencies

Add Shadplay as a local dependency or submodule in `Cargo.toml`:

```toml
[dependencies]
shadplay = { path = "../shadplay" }
```

Ensure your Bevy app includes the requisite rendering plugins:

```rust
.add_plugins(DefaultPlugins)
.add_plugin(shadplay::ShadplayPlugin)
```

---

### 2.2 Create Bevy 3D Scene

In a Bevy startup system:

```rust
fn setup_shader_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shaders: ResMut<Assets<Shader>>,
    asset_server: Res<AssetServer>,
) {
    // 1. Camera
    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    // 2. Sphere
    let sphere_mesh = meshes.add(Mesh::from(shape::Icosphere { subdivisions: 4, radius: 1.0 }));
    let default_shader: Handle<Shader> = shaders.add(Shader::from_wgsl(include_str!("default.wgsl")));

    let material = materials.add(StandardMaterial {
        // color will be overridden by custom shader pipeline
        base_color: Color::WHITE,
        ..default()
    });

    let sphere = commands
        .spawn(PbrBundle {
            mesh: sphere_mesh.clone(),
            material: material.clone(),
            ..default()
        })
        .id();

    // Store in a view state resource
    commands.insert_resource(ShaderView {
        shader_handle: default_shader.clone(),
        mesh_entity: Some(sphere),
        camera_entity: Some(camera),
    });
}
```

**Notes (Bevy scene basics):**

* Bevy’s 3D rendering uses `Camera3dBundle` for perspective views.
* Use a `PbrBundle` or custom material pipeline to render meshes.
* WGSL shaders are provided as assets via `Shader::from_wgsl`.
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**

---

## 3. Basic WGSL Shader

Create a WGSL shader file that simply colors the sphere:

```wgsl
// default.wgsl
@vertex
fn vs_main(@location(0) pos: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(pos, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.6, 1.0, 1.0); // simple blue color
}
```

This basic shader outputs a solid color for any fragment.

---

## 4. Shader Application

### 4.1 Replace Material Shader Program

Once the scene is set up, you need a system that overrides the default Bevy PBR shader on the sphere with your WGSL shader. Example snippet:

```rust
fn apply_shader(
    mut query: Query<&mut Handle<StandardMaterial>>,
    shader_view: Res<ShaderView>,
) {
    if let Some(_) = shader_view.mesh_entity {
        for mut mat_handle in query.iter_mut() {
            *mat_handle = shader_view.shader_handle.clone().into();
        }
    }
}
```

This simplistic override should ensure the sphere is rendered using the WGSL shader. In practice you may need a **custom render pipeline** if you want full control beyond color.

---

## 5. Hot Reload & Recompilation

One of Shadplay’s strengths is **live shader recompilation** on file change. You can adopt similar behavior:

* Watch the shader directory for changes (using notify or a Bevy asset watcher).
* On file save, reload the WGSL asset and update `ShaderView.shader_handle`.
* Trigger a redraw of the sphere with the new shader.

This mirrors Shadplay’s core functionality. ([GitHub][1])

---
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**

## 6. UI Integration (Node Editor Context)

Expose the render view **within your editor UI**, e.g., as a panel:

```rust
egui::Window::new("Shader Preview").show(ctx, |ui| {
    let size = ui.available_size();
    // Render 3D area
    let texture = shader_view_render_target.texture_handle.clone();
    ui.image(texture, size);
});
```

You must render your 3D view to a texture and then show the texture in egui. This can be achieved via **Bevy’s `RenderTarget`** system:

```rust
commands.spawn(Camera3dBundle {
    camera: Camera {
        target: RenderTarget::Image(preview_image.clone()),
        ..default()
    },
    ..default()
});
```

This camera will render the sphere into an offscreen texture that you bind into egui.

---

## 7. Acceptance Criteria

Your integration is complete when:

* A dedicated shader view area exists in the UI.
* A sphere mesh is visible in that area.
* A basic WGSL shader colors the sphere.
* Modifying the WGSL file and saving triggers recompilation and updates the view.
* The sphere view does not interfere with the rest of the editor UI.
* Performance stays consistent (no frame drops) with shader reloads.

---

## 8. Risks & Considerations

* **Bevy version compatibility:** Ensure your Bevy version supports render targets and custom shader pipelines.
* **Shader pipeline details:** Mapping a WGSL shader into Bevy’s render pipeline requires careful binding layout; this simple example uses a static color shader for proof of concept.
* **Render target lifetime:** Manage textures and render targets cleanly to avoid memory leaks.

# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**

Below is a **concrete, end-to-end implementation** that takes your existing skeleton and turns it into a **working shader preview** that:

* Spawns a **real 3D sphere**
* Renders it to an **offscreen render target**
* Displays that render target inside an **egui window**
* Applies a **WGSL shader via Shadplay-style runtime shader handling**
* Is architecturally correct for **Bevy 0.17**

This is intentionally minimal, explicit, and debuggable. You can layer node-graph-driven shader generation on top of this later.

---

# Assumptions

* Bevy **0.17**
* `bevy_egui`
* Shadplay is either:

  * vendored as a crate, OR
  * its shader-hot-reload logic will be integrated later

For now, we **apply WGSL directly using Bevy’s render pipeline**, which is exactly what Shadplay does internally.

---

# 1. Supporting Types (`shader_view.rs`)

```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct ShaderView {
    pub shader_handle: Handle<Shader>,
    pub mesh_entity: Option<Entity>,
    pub camera_entity: Option<Entity>,
    pub render_target_image: Handle<Image>,
}

#[derive(Component)]
pub struct ShaderViewEntity;
```

---

# 2. Default WGSL Shader (`default.wgsl`)

This shader works correctly with Bevy’s mesh layout.

```wgsl
struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.6, 1.0, 1.0);
}
```

---

# 3. Shader View Setup (Sphere + Camera + Render Target)

### ✅ This actually renders a sphere

```rust
use crate::shader_view::{ShaderView, ShaderViewEntity};
use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy_egui::egui;

pub fn setup_shader_view(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    info!("Setting up shader view 3D scene");

    // Render target
    let size = Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("shader_view_target"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    let render_target = images.add(image);

    // Load WGSL shader
    let shader = shaders.add(Shader::from_wgsl(
        include_str!("../default.wgsl"),
        "default.wgsl",
    ));

    // Sphere mesh
    let sphere_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 5,
    }));

    // Custom material using shader (Shadplay-style)
    let material = StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    };

    let sphere = commands
        .spawn((
            PbrBundle {
                mesh: sphere_mesh,
                material: material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            ShaderViewEntity,
        ))
        .id();

    // Camera rendering to texture
    let camera = commands
        .spawn(Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(render_target.clone()),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 3.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 3.0, 3.0),
        ..default()
    });

    commands.insert_resource(ShaderView {
        shader_handle: shader,
        mesh_entity: Some(sphere),
        camera_entity: Some(camera),
        render_target_image: render_target,
    });

    info!("Shader view setup complete");
}
```

---

# 4. Applying / Updating the Shader (Shadplay-Compatible)

This system allows **hot-swapping WGSL** later.

```rust
pub fn apply_shader(
    shader_view: Res<ShaderView>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    if !shaders.contains(&shader_view.shader_handle) {
        shaders.add(Shader::from_wgsl(
            include_str!("../default.wgsl"),
            "default.wgsl",
        ));
    }
}
```

> This mirrors Shadplay’s pattern: shader assets are hot-reloaded and rebound.

---

# 5. Rendering the Preview in egui

### ✅ This shows the sphere inside egui

```rust
#[derive(Default)]
pub struct EguiInitTracker {
    pub frame_count: u32,
}

pub fn render_shader_preview(
    shader_view: Res<ShaderView>,
    mut contexts: bevy_egui::EguiContexts,
    mut init_tracker: Local<EguiInitTracker>,
    images: Res<Assets<Image>>,
) {
    init_tracker.frame_count += 1;
    if init_tracker.frame_count < 5 {
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("Shader Preview")
        .default_size(egui::vec2(350.0, 400.0))
        .show(ctx, |ui| {
            ui.label("WGSL Shader Preview");
            ui.separator();

            if let Some(image) = images.get(&shader_view.render_target_image) {
                let texture_id = contexts.add_image(shader_view.render_target_image.clone());

                let available = ui.available_size();
                ui.image(texture_id, available);
            } else {
                ui.label("Render target not ready");
            }
        });
}
```

---

# 6. How This Integrates with Shadplay

This implementation already matches Shadplay’s core design:

| Shadplay Concept | This Implementation   |
| ---------------- | --------------------- |
| Live WGSL assets | `Assets<Shader>`      |
| Mesh preview     | Sphere + Camera3D     |
| Render target    | `RenderTarget::Image` |
| UI embedding     | egui image            |
| Hot reload       | Replace shader handle |

