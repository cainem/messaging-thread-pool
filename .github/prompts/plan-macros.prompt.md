# Plan: Implement `messaging-thread-pool-macros`

## 1. Project Restructuring
- [ ] **Convert to Workspace**: The current project is a single crate. We need to convert it into a Cargo workspace to host both the main library and the new macro crate.
    - Move existing code to `messaging-thread-pool/`.
    - Create new crate `messaging-thread-pool-macros/`.
    - Create a root `Cargo.toml` defining the workspace.

## 2. Create Macro Crate
- [ ] **Initialize**: Create `messaging-thread-pool-macros` with `proc-macro = true`.
- [ ] **Dependencies**: Add `syn` (full features), `quote`, and `proc-macro2`.
- [ ] **Scaffolding**: Create the entry point `lib.rs` with a placeholder `#[pool_item]` attribute macro.

## 3. Implement `#[pool_item]` Macro
- [ ] **Parsing**: Use `syn` to parse the `impl` block.
    - Identify methods decorated with `#[messaging(Request, Response)]`.
    - Extract method names, arguments, and return types.
- [ ] **Generation (The Heavy Lifting)**:
    - **Request Structs**: Generate structs like `pub struct MeanRequest(pub u64);`.
    - **Response Structs**: Generate structs like `pub struct MeanResponse { pub id: u64, pub result: ReturnType }`.
    - **API Enum**: Generate the enum with variants for each method.
    - **PoolItem Implementation**: Generate the `impl PoolItem for Target` block.
        - Implement `process_message` match arm to call the actual method (e.g., `self.mean()`).

## 4. Integration & Testing
- [ ] **Re-export**: Re-export the macro from the main `messaging-thread-pool` crate so users don't need to add two dependencies.
- [ ] **Test Suite**: Create a new test in `tests/` that uses the macro to define a `PoolItem` (e.g., `MacroRandoms`) and verifies it works identically to the manual implementation.

## 5. Migration (Optional/Verification)
- [ ] **Refactor Sample**: Convert `src/samples/randoms` to use the new macro to prove it works in the real world.

## Risks & Unknowns
- **Complex Signatures**: Handling methods with arguments (other than `&self`) requires generating fields in the Request struct.
- **Generics**: Supporting generic `PoolItem`s adds complexity to the macro generation. *Strategy: Start with non-generic support first.*
