# VibeCoded Game MVP Task Catalogue

## Foundational Setup

### Repository Scaffolding
Establish the Cargo project, directory structure, licensing, and initial README so subsequent work shares a common baseline.

#### Subtasks
- ✅ **Create workspace skeleton** *(Completed)*: Run `cargo new vibecoded-game --bin`, add `README.md`, `.gitignore`, and initial license file.
- ✅ **Lay out directories** *(Completed)*: Add empty `src/`, `assets/`, `docs/`, `build/`, and `tests/` subfolders matching the architecture plan.
- ✅ **Baseline commit** *(Completed)*: Document architecture assumptions in `docs/` and push an initial commit to seed CI and collaboration.

### Toolchain Pinning
Keep contributors aligned on Rust versions and linting expectations.

#### Subtasks
- ✅ **Rust toolchain lock** *(Completed)*: Add `rust-toolchain.toml` specifying the desired stable channel and components.
- ✅ **Cargo configuration** *(Completed)*: Create `.cargo/config.toml` enabling `cargo fmt` and `cargo clippy` with project-wide flags.
- ✅ **Setup note** *(Completed)*: Document installation instructions and toolchain validation steps in `docs/setup.md`.

### Continuous Integration
Catch regressions early with automated formatting, linting, testing, and smoke builds.

#### Subtasks
- ✅ **Workflow skeleton** *(Completed)*: Add `.github/workflows/ci.yml` running `fmt`, `clippy`, and unit tests on push and PR.
- ✅ **Cache optimization** *(Completed)*: Configure Rust cache actions for `cargo` registry and build artifacts to reduce run time.
- ✅ **Cross-build smoke** *(Completed)*: Extend CI with a `cross` or target-specific job that produces Linux/macOS and Windows binaries.

## Rendering & Visual Atmosphere

### Rendering Bootstrap
Integrate Bevy app startup, window configuration, and asset hot reloading to accelerate iteration on visual features.

#### Subtasks
- ✅ **App entry point** *(Completed)*: Implement `src/main.rs` launching Bevy with default plugins and a placeholder scene.
- ✅ **Window tuning** *(Completed)*: Configure resolution, VSync, and resize policies through a dedicated window plugin.
- ✅ **Asset hot reload** *(Completed)*: Enable Bevy asset server settings for watching the `assets/` directory during development.

### HD-2D Pipeline
Implement sprite billboarding, tile mesh layering, depth sorting, and WGSL lighting shaders to produce the Octopath-inspired look.

#### Subtasks
- ✅ **Sprite3D integration** *(Completed)*: Added custom HD-2D billboarding and depth sorting via `Hd2dPipelinePlugin`.
- ✅ **Tile mesh staging** *(Completed)*: Terrain chunks now generate batched meshes driven by `TerrainChunkUpdated` events.
- **Lighting shader pass** *(Not Started)*: Author WGSL shader stages for normal-mapped sprites and ambient/point lighting mix.

### Post-Processing & Camera
Shape the final image with cinematic effects while maintaining Zelda-like responsiveness.

#### Subtasks
- **Camera controller** *(Completed)*: Create a camera plugin supporting orbit, zoom limits, and follow offset.
- **Post-process stack** *(Not Started)*: Implement depth-of-field, bloom, and color grading passes with tunable settings.
- **Performance guardrails** *(Not Started)*: Add instrumentation to monitor frame time and adjust default quality presets.

### Asset Import Workflow
Define conventions for sprites, normal maps, and materials; automate packing and versioning so artists can hand off assets smoothly.

#### Subtasks
- **Naming & folder schema** *(Not Started)*: Document how sprites, normal maps, and metadata files are organized under `assets/`.
- **Conversion scripts** *(Not Started)*: Add a `build/scripts/` tool (Rust or Python) to pack sprite sheets and generate texture atlases.
- **Validation checklist** *(Not Started)*: Create an asset QA markdown checklist that artists can follow before commits.

## World Generation & Streaming

### World Data Structures
Design ECS-friendly representations for tiles, chunks, biomes, and collision layers to support runtime generation and queries.

#### Subtasks
- ✅ **Component schema** *(Completed)*: Define structs/components for tile metadata, biome tags, and chunk coordinates.
- ✅ **Resource registries** *(Completed)*: Added biome definitions, chunk dimension resources, and navigation overlays wired into generation.
- ✅ **Terrain rendering link** *(Completed)*: Centered the camera/player over generated terrain and corrected world-space tile placement in `sync_terrain_tile_sprites()` so chunks now render as expected.
- **Unit tests** *(Not Started)*: Add targeted tests ensuring coordinate math and data serialization behave deterministically.

### Noise-Based Terrain Generator
Build configurable noise pipelines that output biome blends, elevation, and traversal blocking for each chunk.

#### Subtasks
- ✅ **Noise module** *(Completed)*: Integrate `noise` crate and expose seeded noise functions with adjustable parameters.
- **Biome classifier** *(Not Started)*: Map noise outputs to biome descriptors, ensuring transitions remain smooth.
- **Config surface** *(Not Started)*: Create RON/TOML config files allowing designers to tweak thresholds without code changes.

### Chunk Streaming System
Implement background generation, loading, and eviction of world chunks around the player while preserving determinism.

#### Subtasks
- ✅ **Chunk scheduler** *(Completed)*: Build a system that requests chunk generation based on camera/player position.
- **Async generation** *(Not Started)*: Use Bevy tasks or a lightweight thread pool to populate chunk data off the main thread.
- ✅ **Retention policy** *(Completed)*: Implement rules for caching and evicting chunks, plus debug visualization of active regions.

### Collision & Navigation Maps
Derive physics-ready colliders and navigation hints from generated data so gameplay systems have reliable spatial information.

#### Subtasks
- ✅ **Collider extraction** *(Completed)*: Convert terrain blocks into collider primitives compatible with the physics engine.
- **Nav data bake** *(Not Started)*: Generate lightweight navigation hints or flow fields for AI pathing.
- **Verification harness** *(In Progress)*: New navigation overlay toggle exposes collider coverage; expand with tests or analytics.

## Gameplay & Interaction

### Player Controller
Develop top-down movement, dash mechanics, and animation state management that replicate Zelda-like responsiveness.

#### Subtasks
- ✅ **Input mapping** *(Completed)*: Configure keyboard/gamepad bindings and a state machine for player intents.
- ✅ **Movement & dash** *(Completed)*: Implement acceleration curves, dash cooldown, and collision-resolving movement.
- **Animation states** *(Not Started)*: Wire sprite animations or blend trees to mirror facing, moving, and dashing.

### Combat Prototype
Implement a melee attack loop with hit detection, damage stubs, and basic feedback to demonstrate combat pacing.

#### Subtasks
- **Attack command** *(Not Started)*: Add attack input handling and wind-up/recovery timing.
- **Hitbox logic** *(Not Started)*: Spawn transient hitboxes, detect overlaps, and emit combat events.
- **Feedback pass** *(Not Started)*: Trigger placeholder sound, screen shake, and particle cues on impact.

### Enemy Dummy & AI Hooks
Introduce a simple enemy entity with placeholder behavior, providing a target for combat verification.

#### Subtasks
- ✅ **Dummy prefab** *(Completed)*: Create enemy sprites, stats, and spawn logic in the scene.
- **Behavior loop** *(In Progress)*: Implement a minimal state machine (idle, patrol, chase) reacting to player proximity.
- **Metrics logging** *(Not Started)*: Emit telemetry events when the dummy is hit or defeated for tuning.

### Camera Follow & Screen Shake
Ensure the camera tracks the player smoothly and supports effects that emphasize combat hits.

#### Subtasks
- ✅ **Follow rig** *(Completed)*: Add a smoothing follow system with configurable damping and bounds.
- ✅ **Shake module** *(Completed)*: Implement additive camera shake triggered by combat events.
- **User toggles** *(Completed)*: Expose sliders in the debug UI to tune camera offsets and shake intensity.

## Systems & Support Infrastructure

### Physics Integration
Configure collision layers, response handling, and optional kinematic bodies to support movement and combat interactions.

#### Subtasks
- **Physics crate selection** *(Completed)*: Integrate a physics plugin (Rapier) and set up foundational resources.
- **Layer matrix** *(Completed)*: Define collision layers/masks for player, enemies, terrain, and interactive objects.
- **Debug rendering** *(Completed)*: Add a toggleable overlay to visualize colliders at runtime.

### Debug & Tuning UI
Embed `bevy_egui` panels for runtime parameter tweaking (lighting, biome seeds, combat timings) to accelerate iteration.

#### Subtasks
- **UI bootstrap** *(Completed)*: Install `bevy_egui` plugin and create a root debug window accessible via hotkey.
- **Control panels** *(In Progress)*: Build tabs for rendering, worldgen, and gameplay parameters with live bindings.
- **Preset management** *(Not Started)*: Implement save/load routines for debug parameter profiles.

### Runtime Stability
Document graphics/runtime quirks encountered during development and track mitigations.

#### Subtasks
- **Wayland shutdown crash** *(Known Issue)*: Closing the window under KDE Wayland with NVIDIA drivers triggers a post-exit segfault despite clean runtime; consider testing with `WINIT_UNIX_BACKEND=x11` and updating `wgpu`/driver versions.

### Telemetry & Logging
Wire `tracing` spans and structured logs so performance and gameplay events can be analyzed post-session.

#### Subtasks
- **Tracing setup** *(Not Started)*: Configure `tracing-subscriber` with filtered levels and file output.
- **Instrumentation sweep** *(Not Started)*: Add spans/events around worldgen, rendering, and combat loops.
- **Log review script** *(Not Started)*: Provide a lightweight script to summarize timings or errors after a play session.

## Packaging & Documentation

### Cross-Platform Builds
Script reproducible builds for macOS/Linux and document Windows cross-compilation steps, including asset packaging.

#### Subtasks
- **Build scripts** *(Not Started)*: Add `build/` scripts wrapping `cargo build` with appropriate targets and feature flags.
- **Windows toolchain notes** *(Not Started)*: Document requirements for MinGW/MSVC cross compilation and asset bundling.
- **Artifact staging** *(Not Started)*: Create a packaging step that collects binaries, configs, and assets into distributable archives.

### Performance Validation
Create automated or scripted benchmarks to confirm frame pacing and loading thresholds meet MVP expectations.

#### Subtasks
- **Frame timing overlay** *(Not Started)*: Implement an in-game profiler HUD showing FPS and frame time percentiles.
- **Soak test harness** *(Not Started)*: Script a headless or automated run that exercises world streaming for a fixed duration.
- **Reporting template** *(Not Started)*: Provide a markdown template for recording benchmark results across hardware.

### User & Developer Docs
Produce setup guides, architecture summaries, and contribution checklists that prepare new collaborators for MVP development.

#### Subtasks
- ✅ **Setup guide** *(Completed)*: Write `docs/setup.md` covering tool installation and first run steps.
- **Architecture overview** *(Not Started)*: Author a living document summarizing systems, data flow, and extension points.
- **Contribution checklist** *(Not Started)*: Publish expectations for branches, testing, and code review in `CONTRIBUTING.md`.

### Future Roadmap Notes
Capture backlog items and stretch goals discovered during MVP work so they feed the post-MVP planning cycle.

#### Subtasks
- **Backlog doc** *(Not Started)*: Create `docs/ROADMAP.md` with categorized feature ideas and risk notes.
- **Review ritual** *(Not Started)*: Schedule a recurring review to triage new ideas into roadmap or MVP scope.
- **Prioritization rubric** *(Not Started)*: Draft simple criteria (impact vs effort) to guide future sequencing decisions.
