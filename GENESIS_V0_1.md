# Genesis Engine — Technical Scope v0.1

## Purpose
Genesis Engine v0.1 aims to establish a **functional, clear, and extensible core** for an open-source game engine focused on **systems, simulation, and creator control**.

This version is not intended to be a full-featured or production-ready engine.
Its purpose is to demonstrate Genesis’ architecture, philosophy, and technical viability.

---

## v0.1 Goals

Genesis v0.1 must be able to:

- Compile and run on desktop platforms
- Open a window
- Load a scene from a data file
- Render simple entities
- Update game logic through systems
- Receive basic user input

All of this **without relying on a visual editor**.

---

## Supported Platforms

- Linux (primary)
- Windows
- macOS

> Mobile platforms (Android / iOS) are explicitly out of scope for v0.1.

---

## Language and Technical Stack

- Primary language: **Rust**
- Windowing and input: **winit**
- Initial render backend: **wgpu**
- Data formats: JSON or RON
- Build system: Cargo

---

## Included Components (IN SCOPE)

### 1. Core Loop
- Explicit and readable game loop
- Clear separation between:
  - update
  - render
- Configurable fixed timestep

---

### 2. ECS (Entity Component System)
- Entities as identifiers
- Components as plain data structures
- Systems as decoupled logic
- No inheritance
- No implicit state

The ECS must prioritize:
- clarity
- readability
- ease of extension

---

### 3. Scene System
- Scenes defined through data files
- Ability to:
  - create entities
  - assign components
  - define simple relationships

---

### 4. Asset Loading
- Asset loading by path
- No internal asset database
- Open file formats
- Architecture prepared for hot reload (not mandatory in v0.1)

---

### 5. Rendering
- Basic rendering of primitives (triangles / quads)
- Simple camera
- Transform support
- Render abstraction independent from the backend

wgpu is the initial backend, but the engine must not be tightly coupled to it.

---

### 6. Input
- Keyboard
- Mouse
- Gamepad (optional, not required)

---

### 7. Basic CLI
- Command to run a project:
genesis run <project_path>

- Clear console logging

---

## Out of Scope (NOT INCLUDED)

The following systems are **explicitly excluded from Genesis v0.1**:

- Visual editor
- Advanced physics
- Networking
- Advanced audio
- UI system
- Animation
- External scripting
- Marketplace
- Mobile support
- Advanced graphics (PBR, GI, complex shadows)

These may be evaluated in future versions.

---

## Completion Criteria

Genesis v0.1 is considered complete when:

- The project compiles successfully
- A minimal example can be executed
- A scene defined in a data file is rendered
- Input allows interaction with at least one entity
- The codebase is readable and documented
- The architecture reflects the engine’s philosophy

---

## Technical Philosophy

Genesis prioritizes:

- Simplicity over complexity
- Clarity over premature optimization
- Systems over visual effects
- Data over hidden configuration
- Creator freedom over imposed convenience

Genesis v0.1 is the **beginning**, not the destination.
