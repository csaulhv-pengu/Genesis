# Genesis Engine

Genesis Engine is an open-source game engine focused on **systems-driven design,
simulation, and creator control**.

Genesis is not designed to compete with large, general-purpose engines.
Instead, it aims to provide a **clear, minimal, and extensible foundation**
for building games and simulations without hidden complexity or vendor lock-in.

---

## Philosophy

Genesis is built around a few core principles:

- The engine belongs to the creator
- Code should be readable and understandable
- Systems matter more than visual spectacle
- Data should drive behavior
- Tools should be optional, not mandatory
- Transparency is a technical requirement, not a promise

Genesis favors **clarity over convenience** and **simplicity over feature bloat**.

---

## What Genesis Is (and Is Not)

### Genesis is:
- Open-source and permissively licensed (MIT)
- Focused on systems, simulation, and strategy-oriented games
- Designed to run without a visual editor
- Modular and extensible by design
- Desktop-first (Linux, Windows, macOS)

### Genesis is not:
- A AAA graphics engine
- A Unity or Unreal replacement
- A drag-and-drop editor-centric tool
- A closed ecosystem or marketplace

---

## Current Status

Genesis is in **early development**.

The current goal is to complete **Genesis v0.1**, which establishes the engine’s
core architecture and technical foundations.

You can read the full technical scope here:
- [`GENESIS_V0_1.md`](GENESIS_V0_1.md)

---

## Technical Stack (v0.1)

- Language: Rust
- Windowing & Input: winit
- Rendering: wgpu (backend abstraction)
- Data formats: JSON / RON
- Platforms: Linux, Windows, macOS

---

## Project Structure (Planned)

The initial structure will look like:
genesis/
├── crates/
│ ├── genesis-core
│ ├── genesis-ecs
│ ├── genesis-render
│ └── genesis-cli
├── examples/
├── assets/
└── docs/


This structure may evolve as the project grows.

---

## Contributing

Genesis welcomes contributions of all kinds:
code, documentation, discussions, and ideas.

Before contributing, please read:
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

Genesis evolves deliberately. Large changes should be discussed before implementation.

---

## License

Genesis Engine is licensed under the **MIT License**.

You are free to use, modify, and distribute Genesis for both commercial
and non-commercial projects.

See the [`LICENSE`](LICENSE) file for details.

---

## Vision

Genesis exists to empower creators who want to **understand and control**
the systems they build.

It is a foundation — not a cage.
