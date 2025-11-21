# Messaging Thread Pool - GitHub Copilot Instructions

> **Read `AGENTS.md` for comprehensive project guidance.**

## Project Architecture
Rust library crate for typed thread pools with message-based communication:
- `src/` - Core library implementation
- `tests/` - Integration tests
- `benches/` - Performance benchmarks using Criterion
- Single crate structure (not a workspace)

## Essential Initialization Pattern
```rust
use messaging_thread_pool::{ThreadPool, samples::Randoms};

// Create a thread pool with 4 threads
// The lifetime of the elements created (e.g. Randoms) will be tied to the life of this struct
let thread_pool = ThreadPool::<Randoms>::new(4);

// Send requests to create objects in the pool
thread_pool
    .send_and_receive((0..1000u64).map(RandomsAddRequest))
    .expect("thread pool to be available")
    .for_each(|response| assert!(response.result().is_ok()));
```

## Critical Code Organization
- **File naming**: `src/module/struct_name.rs` (filename = struct name)
- **Fields**: Private only - use getters/setters, never public fields
- **Function size**: Split at >40 lines into separate files
- **Tests**: `given_xxx_when_yyy_then_zzz` naming
- **Module structure**: Follows the folder structure, typically `mod.rs` exports structs defined in sibling files.

## Development Workflow
```powershell
cargo fmt && cargo test && cargo clippy   # Pre-commit checks
cargo bench                               # Run benchmarks
cargo test                                # Run all tests
```

## Key Domain Types
- `ThreadPool<T>` - The main entry point, managing threads and lifetime of items.
- `PoolItem` trait - Must be implemented by objects managed by the pool.
- `SenderAndReceiver` trait - Abstraction for sending/receiving messages (allows mocking).
- `id_provider` - Modules for generating unique IDs for pool items.

## Integration Points
- **Tracing**: Uses `tracing` crate for logging.
- **Crossbeam**: Uses `crossbeam-channel` for internal communication.

## Testing Requirements  
- Full branch/condition coverage per function
- Test boundary conditions
- Use `SenderAndReceiverMock` for testing interactions without spawning threads where possible.
- Integration tests in `tests/` directory for complete workflows

## Key Behaviors
- **Always read `AGENTS.md`** for comprehensive context at conversation start
- **Ask before changing** - clarify requirements vs assumptions
- **Use existing patterns** - examine similar functions before creating new approaches