# Project Context: Miska Alexia's bevy game

## Tech Stack & Architecture
- A Bevy game

## Critical Rules & Guardrails
- **AI Constraints:** Never write placeholder code or comments like `// TODO: implement later`. Write complete, working functions.
- **Always Ask Questions:** if you are unsure of something, ask rather than guessing.

## Testing Instructions

### Test Organization
- **Unit tests** in `tests/*.rs` (flat structure, not subdirectories) - run with `cargo test`
- **Integration tests** in same location - share common infrastructure
- **Common test utilities** in `tests/common/mod.rs` - single module with `TestApp` builder

### Test Infrastructure
- **Headless Bevy app** via `TestApp::new()` with:
  - `MinimalPlugins` + `AssetPlugin` + `ScheduleRunnerPlugin` (no window/renderer)
  - `PhysicsPlugins`, `TnuaControllerPlugin`, `TnuaAvian3dPlugin` for physics
  - `StatesPlugin` for state machines
  - `Persistent` resources with temp dir isolation
- **Temp directory per test** using `tempfile::TempDir` for `bevy_persistent` isolation
- **Fixed timestep** via `app.world_mut().run_schedule(FixedUpdate)` for deterministic physics

### Public API for Testing
- Make modules public in `src/plugins/mod.rs` when tests need access
- Export constants (`CAMERA_DISTANCE`), components (`PlayerCamera`), and systems (`apply_controls`) as `pub`
- Add `Default` derive to resources used in test setup (`LocationChange`)

### Unit Test Patterns
- **Pure logic tests** without Bevy app when possible (serialization, state enums, math)
- **Minimal app** with `StatesPlugin` + required resources for system tests
- **Threshold assertions** for physics: `assert!(pos.x > 0.1)` not exact equality
- **`serial_test::serial`** for tests sharing global plugin state

### Integration Test Patterns
- **`TestApp` helper** for spawn/advance/query patterns
- **Collision events** constructed manually: `CollisionStart { collider1, collider2, body1: None, body2: None }`
- **Trigger via** `app.world_mut().trigger(event)` not `send_event`
- **State transitions** via `NextState` resource + `advance_frames(2)`

### Asset Handling
- **Test assets** in `tests/test_assets/` (Yarn files, minimal GLTF)
- **Copied to temp dir** in `TestApp::new()` for `AssetPlugin` loading
- **YarnProject** loaded automatically by `YarnSpinnerPlugin` from asset path

### Coverage Requirement
- New features must include component unit tests compatible with `cargo test`
- Integration tests for cross-system behavior (movement, level loading, dialogue, saving)

### CI Readiness
- All tests run with `cargo test`
- No external dependencies (display, network, etc.)
- Deterministic via fixed timestep
- Parallel-safe via unique temp dirs

## Git & PR Guidelines
- **Never use git** do not attempt to use git