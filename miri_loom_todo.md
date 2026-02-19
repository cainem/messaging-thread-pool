# Miri / Loom TODO

This document captures what to do **later** to add Miri and Loom coverage, and why each item matters.

## Why do this?

We already have deterministic failing tests for several concrete bugs. Miri/Loom are still valuable because they can reveal additional concurrency/soundness issues that normal tests often miss:

- Undefined behavior in executed paths (Miri)
- Data race / aliasing violations hidden behind `unsafe` (Miri)
- Rare interleavings and schedule-dependent bugs (Loom)
- Incorrect assumptions around channel close, shutdown ordering, and lock behavior (Loom)

## Current Risk Areas to Target First

1. `messaging-thread-pool/src/id_based_blocking/cloneable_id_based_writer.rs`
   - `Arc<Mutex<IdBasedWriter>>` coordination and cloning semantics
   - Concurrency hotspot; prioritize a Loom model around writer access patterns and lock behavior

2. `messaging-thread-pool/src/pool_thread/message_loop.rs`
   - Shutdown/abort/channel-closed state transitions
   - Good Loom candidate for schedule exploration

3. `messaging-thread-pool/src/send.rs` + `src/pool_item/mod.rs`
   - Routing assumptions (`thread_count > 0`) and post-shutdown behavior
   - Good candidate for invariant checks in modeled tests

## Miri Plan (Phase 1)

### 1) Toolchain and command setup

- Install/add Miri component for nightly toolchain
- Record exact local commands in this repo (README/CONTRIBUTING or dedicated script)

Why:
- Miri requires nightly + component and is slower/specialized; setup should be repeatable.

### 2) Add a dedicated Miri test target strategy

- Select a narrow set of tests that exercise risky code paths first
- Avoid broad full-suite runs initially (too slow/noisy)

Why:
- Faster feedback and better signal while integrating.

### 3) Enable strict checking flags

- Configure Miri run flags to maximize UB detection
- Document any required env vars

Why:
- Default runs can miss classes of problems or produce inconsistent local behavior.

### 4) Add CI job (non-blocking initially)

- Add separate workflow job for Miri smoke tests
- Start as informational/non-blocking; promote to required once stable

Why:
- Keeps confidence high without blocking all PRs during early tuning.

## Loom Plan (Phase 2)

### 1) Add Loom as dev-dependency and test gating

- Add Loom behind `cfg(loom)` or feature gating
- Keep production code unchanged by default path

Why:
- Loom requires modeled primitives and should not impact normal builds/tests.

### 2) Introduce small model wrappers/adapters

- Add minimal abstraction points for synchronization/channel components used in target path(s)
- Keep adapters local and focused to avoid broad refactor

Why:
- Loom tests need controllable primitives; full-project conversion is high-cost.

### 3) First modeled scenario: channel closed + message loop exit

- Model send/recv/drop interleavings around thread message loop shutdown
- Assert no panic on expected closure paths and no stuck states

Why:
- This area has known behavioral issues and is highly interleaving-sensitive.

### 4) Second modeled scenario: shared writer mutation ordering

- Model concurrent access assumptions around cloneable writer behavior
- Verify invariants around switching active id and writing

Why:
- Highest risk area for hidden unsoundness and order-dependent bugs.

### 5) Tune model bounds and execution budget

- Keep tests tiny (few threads/steps) with explicit bounds
- Add guidance for local vs CI limits

Why:
- Prevents state-space explosion and flaky CI runtimes.

## Definition of Done

Miri/Loom effort is complete when:

- Reproducible local commands are documented
- CI runs Miri smoke checks on targeted tests
- At least 1 Loom model test exists for message loop closure behavior
- At least 1 Loom model (or justified alternative) covers writer/shared-mutation risk
- Found issues are converted into deterministic regression tests where possible

## Suggested Execution Order

1. Stabilize current deterministic regression failures (already added)
2. Add Miri setup + targeted runs
3. Fix any Miri findings and lock in tests
4. Add first Loom model (message loop closure)
5. Add second Loom model (writer/shared mutation)
6. Promote CI checks from informational to required (once stable)
