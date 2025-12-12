# Node Graph Framework

This is a Bevy-based Unreal-style node editor framework that provides the foundational components for building shader node editors with WGSL support. The framework implements all core functionality required for managing nodes, connections, and UI interactions.

## Overview

The framework consists of several key components:

1. **Data Model** - ECS-friendly structs for representing nodes, pins, and connections
2. **Canvas System** - Panning, zooming, and coordinate transformation between screen and canvas space
3. **Node Rendering** - Visual representation of nodes using egui
4. **Connection System** - Creating and rendering visual connections between nodes
5. **Interaction System** - Handling node dragging, pin clicking, and connection creation

## Core Components

### Data Model

#### Node Graph Structure
- `NodeId` - Unique identifier for nodes
- `PinId` - Unique identifier for pins
- `InputPin` - Represents an input pin with label and parent node reference
- `OutputPin` - Represents an output pin with label and parent node reference  
- `NodeInstance` - Complete node representation with position, inputs, outputs, and title
- `Connection` - Visual link between output pin → input pin
- `NodeGraph` - Container holding all nodes, connections, and canvas state

### Canvas System

The canvas system provides:
- Panning functionality (MMB/RMB drag)
- Zooming functionality (scroll wheel)
- Coordinate transformation between screen space and canvas space
- Canvas state management (zoom level and offset)

### Node Rendering

Nodes are rendered as floating egui windows with:
- Title bar showing node name
- Input pins on the left side
- Output pins on the right side
- Draggable positioning within canvas space

### Connection System

Visual connections are drawn as curved lines between nodes, with:
- Proper anchor point calculation 
- Z-order rendering behind node windows
- Connection updates when nodes move

## Usage

### Node Creation

Press 'N' key to spawn a test node at canvas origin. The system creates nodes with:
- Two input pins labeled "A" and "B"
- One output pin labeled "Out"
- Position at (0,0) in canvas space

### Canvas Navigation

- **Panning**: Click and drag with MMB or RMB
- **Zooming**: Ctrl + Scroll or regular scroll wheel
- **Zoom Center**: Zooms centered on cursor position

## Implementation Details

All systems are implemented as Bevy ECS systems and registered in the main application loop. The framework follows a clear separation of:
- Data model (NodeGraph resource)
- UI rendering (egui-based)
- System logic (node creation, interaction handling)

## Future Extensions

This framework can be extended to support:
- Multiple node types with custom properties
- Full pin type checking and connection validation  
- Serialization of node graphs to RON/JSON
- Advanced connection styling and visual effects
- Custom UI elements within nodes
- Node grouping and organization features

The framework is designed to be extensible while maintaining clean separation between core functionality and node-specific implementations.
