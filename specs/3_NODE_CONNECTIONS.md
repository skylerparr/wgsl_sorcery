# Specification

## Feature: Window-Based Node Linking UI (bevy_egui)

### Objective

Implement a **visual node linking system** where each node is an `egui::Window`. Nodes expose **input pins (blue, left)** and **output pins (green, right)**. Users can click-drag from an output pin to create a temporary white link line, then release on a valid input pin to form a persistent connection. Links visually follow node windows as they move.

This spec defines **UI behavior, interaction flow, and rendering logic** only.

---

## 1. Core Concepts

### 1.1 Node Definition

A **Node** is rendered as a `bevy_egui::egui::Window` and contains:

* A title bar
* A vertical list of **input pins** on the left
* A vertical list of **output pins** on the right

Each pin has:

* A visual circle
* A unique `PinId`
* A parent `NodeId`

---

## 2. Data Structures

### 2.1 Identifiers

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinId(u64);
```

---

### 2.2 Pin Types

```rust
pub enum PinKind {
    Input,
    Output,
}

pub struct Pin {
    pub id: PinId,
    pub node: NodeId,
    pub kind: PinKind,
}
```

---

### 2.3 Node

```rust
pub struct Node {
    pub id: NodeId,
    pub title: String,
    pub position: egui::Pos2,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
}
```

---

### 2.4 Connection

```rust
pub struct Link {
    pub from: PinId, // output
    pub to: PinId,   // input
}
```

---

### 2.5 UI Interaction State

```rust
pub struct GraphUiState {
    pub dragging_link: Option<DraggingLink>,
}

pub struct DraggingLink {
    pub from_pin: PinId,
    pub start_pos: egui::Pos2,
    pub current_pos: egui::Pos2,
}
```

---

## 3. Node Rendering (egui::Window)

### 3.1 Window Rendering Pattern

Each node is rendered using:

```rust
egui::Window::new(&node.title)
    .default_pos(node.position)
    .resizable(false)
    .show(ctx, |ui| {
        render_node_contents(ui, node, ui_state);
    });
```

After rendering, update `node.position` using:

```rust
node.position = ui.ctx().memory(|m| {
    m.area_rect(node.id).map(|r| r.min).unwrap_or(node.position)
});
```

(This ensures links follow window movement.)

---

## 4. Pin Rendering

### 4.1 Visual Style

* **Input pins**: Blue (`Color32::from_rgb(80, 120, 255)`)
* **Output pins**: Green (`Color32::from_rgb(80, 200, 120)`)
* Radius: `5.0`

---

### 4.2 Pin Widget Implementation

```rust
fn pin_widget(
    ui: &mut egui::Ui,
    pin: &Pin,
    color: egui::Color32,
    ui_state: &mut GraphUiState,
) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::click_and_drag());

    ui.painter().circle_filled(rect.center(), 5.0, color);

    // Output pin: mouse down starts link
    if pin.kind == PinKind::Output && response.drag_started() {
        ui_state.dragging_link = Some(DraggingLink {
            from_pin: pin.id,
            start_pos: rect.center(),
            current_pos: rect.center(),
        });
    }

    response
}
```

---

## 5. Interaction Rules

### 5.1 Dragging a Link

* Mouse **down on output pin**:

  * Begin dragging a temporary white line
* Mouse **move**:

  * Update `current_pos` to cursor position
* Mouse **up**:

  * Check if released over a **valid input pin**

---

### 5.2 Valid Connection Conditions

A link is created **only if**:

1. Drag started from an **output pin**
2. Mouse released on an **input pin**
3. Input pin belongs to a **different node**

If any condition fails:

* Temporary link is discarded
* No connection is created

---

### 5.3 Mouse Up Detection on Input Pins

```rust
if let Some(drag) = &ui_state.dragging_link {
    if response.hovered() && response.mouse_released() {
        if pin.kind == PinKind::Input && pin.node != output_node {
            links.push(Link {
                from: drag.from_pin,
                to: pin.id,
            });
        }
        ui_state.dragging_link = None;
    }
}
```

---

## 6. Link Rendering

### 6.1 Persistent Links

All established links are rendered every frame.

```rust
fn draw_link(
    painter: &egui::Painter,
    start: egui::Pos2,
    end: egui::Pos2,
) {
    painter.line_segment(
        [start, end],
        egui::Stroke::new(2.0, egui::Color32::WHITE),
    );
}
```

> The start position is the **output pin center**,
> the end position is the **input pin center**.

---

### 6.2 Dragging (Temporary) Link

While dragging:

```rust
if let Some(link) = &ui_state.dragging_link {
    painter.line_segment(
        [link.start_pos, link.current_pos],
        egui::Stroke::new(2.0, egui::Color32::WHITE),
    );
}
```

---

## 7. Updating Link Positions on Window Move

Each frame:

1. Node windows update their `position`
2. Pin world positions are recomputed
3. Links re-render using updated positions

This guarantees:

* Links always follow nodes
* No stale geometry

---

## 8. Rendering Order

1. Background (optional)
2. Persistent links
3. Temporary dragging link
4. Node windows (topmost)

Use a full-screen painter:

```rust
let painter = ctx.layer_painter(egui::LayerId::background());
```

---

## 9. Acceptance Criteria

The implementation is correct when:

* Nodes render as egui Windows
* Inputs appear left, outputs right
* Output pins are green, input pins blue
* Clicking an output pin starts a white link
* Releasing on a valid input pin creates a connection
* Releasing elsewhere cancels the link
* Links visually attach under pins
* Links move correctly when windows are dragged
* Links cannot connect a node to itself

---

## 10. Explicit Non-Goals

* No type checking
* No multiple connections per pin validation
* No bezier curves (straight lines only)
* No serialization
* No shader logic

