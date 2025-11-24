# Messaging Thread Pool Project - Agent Instructions

## Project Context
This is a Rust-based library project for creating typed thread pools that communicate via messages.
The main library is in `src/`.
Benchmarks are located in `benches/`.
Integration tests are in `tests/`.

## Development Standards

### Code Organization
- **Simple solutions first** - Always prefer straightforward implementations
- **No code duplication** - Check existing codebase before adding new functionality
- **Clean structure** - Keep codebase organized and maintainable
- **Avoid unnecessary refactoring** - Stick to requested tasks only

### File Structure Rules
- **Structs in own files** - `src/module/struct_name.rs` where filename matches struct name
- **Size limits** - Functions >40 lines should be split; files >40-100 lines should be refactored
- **Private fields only** - Use getters/setters instead of public struct fields
- **No one-off scripts** - Avoid temporary or single-use files

### Testing Requirements
- **Full unit test coverage** - Every function needs comprehensive tests that test each branch and condition
- **Test naming** - Use `given_xxx_when_yyy_then_zzz` pattern
- **Mocking** - Use `SenderAndReceiverMock` where appropriate to test logic without threading overhead.
- **Performance** - Tests must run efficiently while maintaining coverage

### Change Management
- **Conservative changes** - Only implement requested features
- **Existing patterns first** - Exhaust current implementation before new patterns
- **Remove old code** - If introducing new patterns, clean up duplicates
- **Check comments for accuracy** - check that changes haven't left inaccurate constants
- **Add new comments** - for functions added or amended make sure the function is accompanied by appropriate comments
- **Check tests** - Make sure the tests covering changed code still cover all conditions and branches

### Environment Notes
- **Windows development** - Code targets Windows environment (but should be cross-platform compatible)
- **Never overwrite .env** - Always ask before modifying environment files

## Build & Test Commands
- Format: `cargo fmt`
- Build: `cargo build`
- Test: `cargo test`
- Benchmark: `cargo bench`
- Lint: `cargo clippy`
- Lint tests: `cargo clippy --tests`

## Before commit code to the repository
- Ensure `cargo fmt` has been run
- Ensure all tests pass: `cargo test`
- Ensure linting passes with zero warnings: `cargo clippy` and `cargo clippy --tests`. **All clippy warnings must be fixed** - either by correcting the code or by adding explicit `#[allow(...)]` annotations with justification comments for intentional exceptions.
- Ensure code adheres to all outlined standards and guidelines
- Ensure commit messages are clear and descriptive
- Ensure all changed functions and structs have appropriate tests with full coverage
- Ensure that all changed functions and structs have appropriate documentation comments


## Key Directories
- `src/` - Core library implementation
- `tests/` - Integration tests
- `benches/` - Performance benchmarks
