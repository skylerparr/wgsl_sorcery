# Specification

# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**

## Feature: Full Visual Node Rendering Layer for Bevy + egui

### Objective

Implement a complete **egui-based visual node editor component** that sits on top of the existing node-graph framework. This layer must provide:

* Fully rendered nodes (styled, interactive)
* Clickable input and output pins
* Reliable connection rendering
* Correct positional transforms between canvas space and screen space
* Smooth dragging behavior
* A minimal but reliable “node editor viewport” inside egui

No shader generation or domain logic is required—this spec covers **only the rendering and interaction layer**.
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
---

# 1. Rendering Architecture

### 1.1 Rendering Approach

Use **egui Areas** (not `Window`) because Areas allow fully custom styling, no OS window chrome, customizable hitboxes, and precise control similar to Unreal’s node UI. The architecture must include:

* A parent "Node Canvas Layer" that transforms canvas→screen coordinates.
* A rendering pass for:

  1. Background
  2. Connections (under nodes)
  3. Nodes (widgets inside Areas)
* Node content drawn with standard egui UI primitives: frames, labels, layout rows, interactables, painter operations for pins.

### 1.2 Global Editor State

Rendering systems must consume:

* Existing `NodeGraph` resource
* CanvasState (zoom, offset)
* Pending connection state (from pin)

### 1.3 Coordinate Transformation

All node windows must be placed using:

```
screen_pos = canvas_to_screen(node.position)
```

Pin hit detection and connection endpoints must apply the same transform.

---
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# 2. Node Rendering Specification

### 2.1 Node Visual Layout

Each node consists of:

#### 1. **Node Frame**

* Rounded rectangle background
* Slight shadow
* Header bar with node title
* Optional color strip to differentiate node types (can be uniform for now)

#### 2. **Pin Columns**

* Left column: input pins
* Right column: output pins
* Each pin row contains:

  * Small pin circle (rendered via `ui.painter().circle`)
  * Label text next to pin
* Column alignment must remain consistent regardless of UI scale

#### 3. **Internal Content Area**

* Empty for now
* Must reserve spacing so nodes can vary height cleanly

### 2.2 Node Frame Implementation

Use:

* `egui::Area` with `.movable(false)` – dragging will be implemented manually via interaction system
* `egui::Frame::none().fill(background_color).stroke(...)` around an internal container
* Node width: dynamic based on content, but minimum width ~220 px

### 2.3 Node Dragging
From browsing those examples, there isn’t a custom “window dragging system” written by the author – dragging is provided by `egui::Window` itself. What the examples show is:

- how to set up `bevy_egui` so egui windows can be dragged
- how a custom draggable area is implemented in egui (the `Painting` example uses `Sense::drag`, which is exactly the pattern you’d use for your own drag logic)
- how to control whether a “window-like” UI element is draggable or fixed (anchored).

Below is a distilled “how to implement window dragging” based on that.

---

## 1. Basic setup (so windows can be dragged at all)

All examples follow the same pattern: add `EguiPlugin` and then draw windows inside an `EguiPrimaryContextPass` system. For example (from `ui.rs`):

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_camera_system)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn ui_system(mut contexts: EguiContexts) -> Result<(), bevy_egui::EguiError> {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("My Window")
        .vscroll(true)
        .show(ctx, |ui| {
            ui.label("Drag me by the title bar.");
        });

    Ok(())
}
```

Once you’re at this point, egui’s windows are *already draggable*:

- You drag by grabbing the window’s title bar.
- Egui tracks the position for you; you don’t need your own position state unless you want to override it.

This is exactly what the `ui.rs` example demonstrates in the line:

```rust
ui.label("Windows can be moved by dragging them.");
```

inside an `egui::Window::new("Window")` block.

---
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
## 2. Controlling “draggability” of windows

The examples show two main modes:

### 2.1 Normal draggable window (`ui.rs`, `two_windows.rs`)

Plain `egui::Window::new("...")`:

```rust
egui::Window::new("First Window")
    .vscroll(true)
    .show(egui_ctx.get_mut(), |ui| {
        // ...
    });
```

Behavior:

- Has a title bar.
- Is movable (draggable) by default.
- Egui stores the window position between frames.

You can further tweak:

```rust
egui::Window::new("My Window")
    .movable(true)         // explicit (default)
    .resizable(true)       // allow resizing
    .default_pos([100.0, 100.0])
    .show(ctx, |ui| { /* ... */ });
```

### 2.2 Fixed, non-draggable HUD-style window (`split_screen.rs`)

The `players_count_ui_system` example:

```rust
egui::Window::new("")
    .fixed_size([200.0, 30.0])
    .title_bar(false)
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
    .show(egui_contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            if ui.button("-").clicked() { /* ... */ }
            ui.label(format!("Player count: {}", players_count.0));
            if ui.button("+").clicked() { /* ... */ }
        })
    });
```

Key points:

- `title_bar(false)` removes the title bar, so there is no drag handle.
- `anchor(...)` pins the window to a fixed point of the screen.
- Result: a HUD overlay that *doesn’t* move, regardless of dragging.

So if you want a draggable window: **don’t call `title_bar(false)`** and don’t use `anchor` for a fixed position, or use `movable(true)` explicitly.

---

## 3. Implementing a custom draggable area (pattern from `Painting`)

If you want to implement a *custom* drag system (for your own panels, nodes in a graph, etc.), the most instructive code is the `Painting` widget in `ui.rs`, which uses `Sense::drag()`:

```rust
pub fn ui_content(&mut self, ui: &mut egui::Ui) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

    let rect = response.rect;

    if self.lines.is_empty() {
        self.lines.push(vec![]);
    }
    let current_line = self.lines.last_mut().unwrap();

    if let Some(pointer_pos) = response.interact_pointer_pos() {
        let canvas_pos = pointer_pos - rect.min;
        if current_line.last() != Some(&canvas_pos) {
            current_line.push(canvas_pos);
        }
    } else if !current_line.is_empty() {
        self.lines.push(vec![]);
    }

    // draw
    for line in &self.lines {
        if line.len() >= 2 {
            let points: Vec<_> = line.iter().map(|p| rect.min + *p).collect();
            painter.add(egui::Shape::line(points, self.stroke));
        }
    }
}
```
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
How this translates to a generic “dragging system”:

1. **Allocate an interactive region with drag sense**:

   ```rust
   let (response, _painter) =
       ui.allocate_painter(size, egui::Sense::drag());
   ```

2. **Check if the pointer is interacting**:

   ```rust
   if let Some(pointer_pos) = response.interact_pointer_pos() {
       // pointer is inside and active
   }
   ```

3. **Use the drag delta to move your item**:

   ```rust
   if response.dragged() {
       let delta = response.drag_delta();
       my_pos += delta; // egui::Pos2 or Vec2 – your custom position
   }
   ```

Putting it together into a “draggable window-like panel” you control yourself:

```rust
#[derive(Default)]
struct DraggablePanel {
    pos: egui::Pos2,
    size: egui::Vec2,
}

impl DraggablePanel {
    fn ui(&mut self, ctx: &egui::Context) {
        let layer_id = egui::LayerId::new(egui::Order::Middle, egui::Id::new("my_panel"));
        let mut ui = egui::Ui::new(
            ctx.clone(),
            layer_id,
            egui::Rect::from_min_size(self.pos, self.size),
            egui::Id::new("my_panel_ui"),
        );

        // Header: drag handle
        let header_height = 24.0;
        let (response, _) = ui.allocate_painter(
            egui::Vec2::new(self.size.x, header_height),
            egui::Sense::drag(),
        );

        if response.dragged() {
            let d = response.drag_delta();
            self.pos += d;
        }

        // Body: your content goes here
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                self.pos + egui::vec2(0.0, header_height),
                egui::vec2(self.size.x, self.size.y - header_height),
            ),
            |ui| {
                ui.label("Panel content");
            },
        );
    }
}
```

In Bevy + `bevy_egui`, you’d call `panel.ui(ctx)` from an `EguiPrimaryContextPass` system, just like other examples.

---

## 4. Multi-window + dragging (from `two_windows.rs`)

`two_windows.rs` shows:

- Using multiple Bevy OS windows.
- Attaching an egui context to each, drawing one `egui::Window` per Bevy window.

Each egui window in each OS window is draggable the same way – there is no extra logic per window:

```rust
fn ui_first_window_system(
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut ui_state: Local<UiState>,
    mut shared_ui_state: ResMut<SharedUiState>,
    images: Res<Images>,
    mut egui_ctx: Single<&mut EguiContext, With<PrimaryEguiContext>>,
) {
    let bevy_texture_id =
        egui_user_textures.add_image(EguiTextureHandle::Weak(images.bevy_icon.id()));

    egui::Window::new("First Window")
        .vscroll(true)
        .show(egui_ctx.get_mut(), |ui| {
            // ...
        });
}
```

So if your goal is:

- “Each Bevy window has its own draggable egui windows” – follow `two_windows.rs` for window setup, plus normal `egui::Window` usage.

---

## 5. Summary: how to implement a “window dragging system”

1. **If you just want draggable windows**:
   - Use `egui::Window::new("Title")` as in `ui.rs` and `two_windows.rs`.
   - Don’t disable the title bar or anchoring, or explicitly call `.movable(true)`.
   - Egui handles all drag logic and persistence of window position.

2. **If you want non-draggable HUD windows**:
   - Use `.title_bar(false)` and `.anchor(...)` like in `split_screen.rs` to pin them.

3. **If you want your own custom drag behavior** (for panels, custom windows, graph nodes, etc.):
   - Use the pattern from `Painting`:
     - Allocate a region with `Sense::drag()`.
     - In each frame, use `response.drag_delta()` to update your stored position.
   - Use your stored position to place your UI each frame.

If you tell me exactly what you’re trying to drag (egui windows only, custom panels, or Bevy entities in world space), I can give a focused snippet wired into Bevy systems.

---

# 3. Pin Rendering & Interaction

### 3.1 Pin Geometry

Each pin is defined by:

* Shape: Circle radius ~6px
* Alignment:

  * Inputs pinned to left margin
  * Outputs pinned to right margin
* Color: fixed neutral tone (framework-level), may be stylable later
* Hitbox:

  * Entire circle area
  * Egui interaction region using `ui.allocate_rect(...)`

### 3.2 Pin Placement

Pin anchors must be consistently computed as:

```
pin_screen_pos = node_screen_pos + local_offset
```

Local offsets must be stable per pin index based on content layout.

### 3.3 Pin Click Behavior

When user clicks an output pin:

* Set `PendingConnection { from_pin, from_screen_pos }`

When clicking an input pin while `PendingConnection` active:

* Validate connection
* Create `Connection` entry in NodeGraph
* Clear pending state

Clicking empty canvas cancels pending connection.

---

# 4. Connection Rendering Specification

### 4.1 Rendering Pass

Connections must render *under* nodes using the painter for the node canvas background.
Use `egui::Painter` from a top-level `Area` spanning the entire canvas region.

### 4.2 Curve Style

Draw cubic bezier:

```
start = output_pin_pos
end = input_pin_pos

ctrl_offset = Vec2::new(80.0 * zoom, 0.0)
ctrl1 = start + ctrl_offset
ctrl2 = end - ctrl_offset
```

### 4.3 Color and Thickness

* Line thickness: 2.0px (scaled by zoom)
* Color: mid-gray for now

### 4.4 Pending Connection Line

If `PendingConnection` exists:

* Use the same bezier logic with `end` being mouse position
* Style: dashed or slightly transparent

---

# 5. Canvas Rendering and Interaction

### 5.1 Background

Render a grid like Unreal/Blender:

* Light grid lines
* Grid spacing: 48px (adjusted by zoom)

Use `Painter` to draw grid manually.

### 5.2 Pan & Zoom

Your existing pan/zoom logic must be integrated with rendering:

* Background grid moves with canvas offset
* All node screen positions are recomputed based on transform
* All pin hitboxes updated each frame

### 5.3 Deterministic Layer Ordering

Rendering order:

1. Canvas background (grid)
2. Connections
3. Nodes (Areas)
4. Floating temporary effects (pending connection line)

---

# 6. Rendering Systems Required

The AI must implement the following systems:

### 6.1 `render_canvas_background_system`

* Draw grid
* Respond to zoom and pan

### 6.2 `render_connections_system`

* Iterate over connections in NodeGraph
* Compute pin screen positions
* Draw beziers

### 6.3 `render_pending_connection_system`

* If pending connection exists, draw temporary bezier

### 6.4 `render_nodes_system`

For each node:

* Convert canvas → screen position
* Render egui Area
* Inside area:

  * Render frame
  * Render header
  * Render input pins
  * Render output pins
* Record pin screen positions for interaction system

### 6.5 `node_drag_interaction_system`

* Detect pointer-down in header region
* Track drag motion
* Update node.position in canvas space

### 6.6 `pin_interaction_system`

* Register pin hitboxes
* Handle pin click logic
* Trigger pending connection creation
* Trigger final connection creation

---

# 7. Required Data Extensions

### 7.1 NodeInstance Additions

The AI must extend NodeInstance to include:

* `size: Vec2` (updated each frame)
* `header_height: f32`
* `pin_offsets: Vec<(PinId, Vec2)>` for both inputs and outputs

### 7.2 Interaction State

Implement a resource:

```
pub struct GraphUiState {
    pub pending_connection: Option<PendingConnection>,
    pub active_drag_node: Option<NodeId>,
    pub drag_origin: Vec2,
    pub drag_offset: Vec2,
}
```

---
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# 8. Style and UX Requirements

* Nodes must remain readable at all zoom levels
* Node titles centered horizontally
* Pins vertically spaced evenly
* Use consistent typography (egui default)
* Maintain easy-to-follow connection curves
* Node dragging feels smooth and responsive
* No flickering during movement
* No position drift between frames

---

# 9. Functional Acceptance Criteria

The feature is complete when:

1. Canvas shows grid and responds to pan/zoom.
2. Nodes render with styled frames, headers, input/output pins.
3. Pins are clickable and highlight on hover.
4. You can drag nodes freely across the canvas.
5. Connections render cleanly as beziers under nodes.
6. Pending connections visually follow the mouse.
7. All interactions work correctly with zoom and pan.
8. No jittering or misalignment in node windows.
9. UI updates occur each frame without errors/panics.
10. All rendering code respects the existing NodeGraph ECS resource.

# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
# **CRITICAL IMPORTANCE!!!!!!!!!!!!!!!!!!!!!!!!!!!**

**NEVER DELETE CODE TO MAKE THE PROJECT COMPILE. THAT IS THE BIGGEST VIOLATION. AGAIN, NEVER
EVER EVER EVER DELETE CODE TO GET THE PROJECT TO COMPILE.**
