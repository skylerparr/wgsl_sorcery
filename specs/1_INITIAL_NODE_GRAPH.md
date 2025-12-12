Below is a **complete, production-ready software specification** appropriate for passing to an autonomous coding agent. It is focused **only on the framework**, **UI layout**, and **node-graph mechanics** for an Unreal-style shader node editor implemented in **Bevy 0.17 + bevy_egui**, without shader logic.

If you want, I can also generate the full architectural folder structure, a task graph for the agent, or a step-by-step implementation guide.

---

# Software Specification

## Feature: Unreal-Style Node Editor Framework (Bevy 0.17 + bevy_egui)

### Purpose

Implement a reusable UI framework that provides **floating node windows** inside an **editor canvas**, each window representing a “node” with **named inputs and outputs**. This system will serve as the foundation for a WGSL shader graph but must be delivered **independent of shader implementation**.

The initial deliverable is an operable in-editor node-graph framework that enables:

* Adding new nodes (each rendered as an **egui floating window**).
* Each node displaying:

  * **Input pins** (left side)
  * **Output pins** (right side)
* A **zoomable/pannable node canvas**.
* A **connection system** enabling visual representation of links between nodes (no computation required yet).

---

# 1. Technical Stack

### 1.1 Runtime

* **Rust 1.79+**
* **Bevy 0.17**
* **bevy_egui 0.27 or latest compatible**

### 1.2 Architectural Goals

* Pure Bevy ECS for state management.
* All UI is rendered using egui via bevy_egui.
* Separation between:

  * Node graph state (data model)
  * Node graph rendering (UI)
  * Node window logic
  * Connection rendering

---

# 2. Functional Requirements

## 2.1 Node Canvas

A dedicated fullscreen or partial-screen draggabble, pannable, zoomable canvas.

**Requirements:**

* User may **click and drag** with MMB or RMB to pan.
* User may use **Ctrl + Scroll** or just Scroll to zoom.
* All node windows must be positioned in world-space inside this canvas and move according to panning/zooming transforms.
* Canvas holds a list of node instances.

---

## 2.2 Node Windows (Floating Egui Windows)

A node window must behave like an Unreal Engine shader node:

### Required UI layout:

* Title bar with node name.
* Left column: **Input pins** (labeled).
* Right column: **Output pins** (labeled).
* Optional area for in-node parameter controls (empty for now).

### Behavior:

* Windows must be draggable **inside the canvas coordinate system**, not screen coordinates.
* Each window stores:

  * Global ID
  * Node type ID
  * Input pin list
  * Output pin list
  * Position (Vec2 in canvas space)
  * Size (Vec2) – determined by egui but stored for connection rendering

### Pin rendering:

* Each pin is represented by a small circle or square.
* Must be clickable to initiate connections.
* Pin layout uses fixed offsets relative to the window rectangle.

---

## 2.3 Node Graph Data Model

Implement ECS-friendly structs.

### `NodeGraph`

Contains:

* HashMap<NodeId, NodeInstance>
* Vec<Connection>
* Canvas state (zoom: f32, offset: Vec2)

### `NodeInstance`

Contains:

* node_id: NodeId
* position: Vec2
* inputs: Vec<InputPin>
* outputs: Vec<OutputPin>
* window title: String

### `InputPin` / `OutputPin`

* pin_id: PinId
* label: String
* parent_node: NodeId

### `Connection`

Visual and logical link between output pin → input pin.
Contains:

* from_pin: PinId
* to_pin: PinId

---

## 2.4 Connection Creation UX

### Behavior:

* User clicks an output pin → system enters “connecting mode”.
* A temporary bezier line is drawn from the originating pin to the cursor.
* When user clicks an input pin, a Connection is created.
* If user clicks empty space, the temporary link is canceled.

### Rendering:

* Connections are drawn as **curved bezier lines** in the canvas (egui paint API).
* Color and thickness configurable.

---

## 2.5 Node Creation Menu

A simple “Add Node” menu item or hotkey should create a test node.

Requirements:

* Pressing **N** adds a sample node at canvas origin.
* Node contains two example input pins and one output pin.
* Node window appears immediately and is draggable.

---

# 3. Implementation Requirements

## 3.1 Canvas System

Implement a system to manage:

* Panning
* Zoom
* Converting screen coordinates ↔ canvas coordinates
* Maintaining global transform for node windows

Canvas Transform:

```
canvas_pos = (screen_pos / zoom) + offset
```

## 3.2 Node Window Rendering

Use `egui::Area::new(id)` or `egui::Window::new(title)` with `.fixed_pos()`, but positions must be transformed into screen-space based on canvas transform.

Important:
egui works in screen space. Node positions must be stored in canvas space and transformed into screen space before rendering.

---

## 3.3 Connection Rendering

Use `egui::Painter::add(Shape::bezier_cubic(...))`.

Compute anchor points:

```
start = output_pin.screen_position
end   = input_pin.screen_position
control_offset = Vec2::new(80.0 * zoom, 0.0)
```

Draw at correct z-order: under windows.

---

## 3.4 Node Registry (Optional for Phase 1)

Define a simple static description of available node types.
Only required to spawn example nodes for now.

---

# 4. Deliverables for the Coding Agent

### 4.1 Source Code Structure (required)

Agent shall produce:

```
src/
  node_graph/
    mod.rs
    model.rs           // Node structs
    canvas.rs          // Pan/zoom + transform
    render.rs          // Node rendering + connections
    interactions.rs    // Dragging, pin clicking, linking
  systems/
    spawn_node.rs
  main.rs
  lib.rs
```

### 4.2 Systems to Implement

* `update_canvas_system`
* `render_canvas_background_system`
* `render_nodes_system`
* `render_connections_system`
* `handle_node_drag_system`
* `handle_pin_interactions_system`
* `spawn_test_node_system`

### 4.3 Must Build & Run

At the end of the build, user must be able to:

* Open the app.
* Pan/zoom canvas.
* Press "N" to add nodes.
* Drag nodes around.
* Click output pin → click input pin → create a connection.
* See connections drawn visually.

No shader logic is required.
No WGSL generation is required.
Only the framework.

---

# 5. Non-Functional Requirements

### 5.1 Performance

* Must maintain 60+ FPS with 200 nodes and 500 connections.
* Avoid allocating memory per frame.

### 5.2 Maintainability

* Strict separation of model vs UI.
* NodeGraph must be serializable (RON or JSON).

### 5.3 Extensibility

* New node types should be easy to define.
* Connecting logic must not depend on node content.
* Connection structure must future-proof for typed pin definitions.

---

# 6. Acceptance Criteria

The feature is complete when:

1. The Bevy project launches into a node editor screen.
2. Canvas is draggable/pannable/zoomable.
3. Nodes spawn on command and are draggable.
4. Each node displays its pins.
5. User can connect pins visually.
6. Connections follow node movement.
7. All elements persist within a coherent node graph model.
8. No shader-related logic exists yet.
