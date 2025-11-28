//! # Messaging Thread Pool Macros
//!
//! This crate provides the `#[pool_item]` attribute macro for the
//! [`messaging_thread_pool`](https://docs.rs/messaging_thread_pool) crate.
//!
//! The macro dramatically reduces boilerplate when creating pool items by automatically
//! generating:
//! - The `PoolItem` trait implementation
//! - Request and response structs for each messaging method
//! - The API enum that routes messages to methods
//! - Type conversions between messages and the thread request/response types
//!
//! ## Basic Usage
//!
//! ```rust,ignore
//! use messaging_thread_pool::{IdTargeted, pool_item};
//!
//! #[derive(Debug)]
//! pub struct MyItem {
//!     id: u64,
//!     data: String,
//! }
//!
//! impl IdTargeted for MyItem {
//!     fn id(&self) -> u64 { self.id }
//! }
//!
//! #[pool_item]
//! impl MyItem {
//!     // Constructor - called when MyItemInit(id) is received
//!     pub fn new(id: u64) -> Self {
//!         Self { id, data: String::new() }
//!     }
//!
//!     // Message handler - generates SetDataRequest and SetDataResponse
//!     #[messaging(SetDataRequest, SetDataResponse)]
//!     pub fn set_data(&mut self, value: String) {
//!         self.data = value;
//!     }
//!
//!     // Message handler with return value
//!     #[messaging(GetDataRequest, GetDataResponse)]
//!     pub fn get_data(&self) -> String {
//!         self.data.clone()
//!     }
//! }
//! ```
//!
//! ## What Gets Generated
//!
//! For a struct named `MyItem` with the above implementation, the macro generates:
//!
//! ### Initialization
//! - `MyItemInit(u64)` - Request struct to create a new pool item
//! - Implementation of `PoolItem::new_pool_item` that calls your `new` function
//!
//! ### For each `#[messaging]` method
//! - Request struct: `SetDataRequest(u64, String)` - ID + method parameters
//! - Response struct: `SetDataResponse { id: u64, result: () }` - ID + return value
//! - Conversions to/from `ThreadRequestResponse<MyItem>`
//!
//! ### API Enum
//! - `MyItemApi` - Enum with variants for each message type, used for routing
//!
//! ## The `#[messaging]` Attribute
//!
//! Place `#[messaging(RequestType, ResponseType)]` on methods that should be callable
//! via messages:
//!
//! ```rust,ignore
//! #[messaging(CalculateRequest, CalculateResponse)]
//! pub fn calculate(&self, x: i32, y: i32) -> i64 {
//!     (x as i64) + (y as i64)
//! }
//! ```
//!
//! This generates:
//! - `CalculateRequest(u64, i32, i32)` - tuple struct with (id, x, y)
//! - `CalculateResponse { id: u64, result: i64 }` - struct with id and return value
//!
//! ### Method Requirements
//! - Must take `&self` or `&mut self` as first parameter
//! - Additional parameters become fields in the request struct
//! - Return type (or `()`) becomes the `result` field in the response struct
//!
//! ## Optional Parameters
//!
//! ### Custom Initialization Type
//!
//! For complex constructors that need more than just an ID:
//!
//! ```rust,ignore
//! // Your custom init request
//! #[derive(Debug)]
//! pub struct MyComplexInit {
//!     pub id: u64,
//!     pub config: Config,
//!     pub options: Options,
//! }
//!
//! impl IdTargeted for MyComplexInit {
//!     fn id(&self) -> u64 { self.id }
//! }
//!
//! impl RequestWithResponse<MyItem> for MyComplexInit {
//!     type Response = AddResponse;
//! }
//!
//! // Tell the macro to use your custom type
//! #[pool_item(Init = "MyComplexInit")]
//! impl MyItem {
//!     // Constructor receives your custom type
//!     pub fn new(init: MyComplexInit) -> Self {
//!         Self {
//!             id: init.id,
//!             config: init.config,
//!             // ...
//!         }
//!     }
//! }
//! ```
//!
//! ### Custom Shutdown Handler
//!
//! To perform cleanup when the pool shuts down:
//!
//! ```rust,ignore
//! #[pool_item(Shutdown = "cleanup")]
//! impl MyItem {
//!     pub fn new(id: u64) -> Self { /* ... */ }
//!
//!     // Called during pool shutdown
//!     pub fn cleanup(&self) -> Vec<ThreadShutdownResponse> {
//!         // Perform cleanup, return any shutdown responses
//!         vec![ThreadShutdownResponse::new(self.id, vec![])]
//!     }
//!
//!     #[messaging(DoWorkRequest, DoWorkResponse)]
//!     pub fn do_work(&self) { /* ... */ }
//! }
//! ```
//!
//! ### Combining Parameters
//!
//! ```rust,ignore
//! #[pool_item(Init = "MyComplexInit", Shutdown = "cleanup")]
//! impl MyItem {
//!     // ...
//! }
//! ```
//!
//! ## Generic Pool Items
//!
//! The macro supports generic types:
//!
//! ```rust,ignore
//! pub trait MyTrait: Debug + Send { /* ... */ }
//!
//! #[derive(Debug)]
//! pub struct GenericItem<T: MyTrait> {
//!     id: u64,
//!     inner: T,
//! }
//!
//! impl<T: MyTrait> IdTargeted for GenericItem<T> {
//!     fn id(&self) -> u64 { self.id }
//! }
//!
//! #[pool_item(Init = "GenericItemInit<T>")]
//! impl<T: MyTrait> GenericItem<T> {
//!     pub fn new(init: GenericItemInit<T>) -> Self {
//!         Self { id: init.id, inner: init.inner }
//!     }
//!
//!     #[messaging(ProcessRequest, ProcessResponse)]
//!     pub fn process(&self) -> String {
//!         format!("{:?}", self.inner)
//!     }
//! }
//! ```
//!
//! Note: Generic types typically require custom `Init` types because the generated
//! `{StructName}Init` would need the generic parameters.
//!
//! ## Complete Example
//!
//! See `messaging_thread_pool::samples::UserSession` for a full working example that
//! demonstrates using `Rc<RefCell<T>>` with the pool item pattern.

mod generation;
mod parsing;

use parsing::PoolItemArgs;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

/// Attribute macro that generates `PoolItem` implementation and message types.
///
/// # Usage
///
/// Apply to an `impl` block for a struct that implements `IdTargeted`:
///
/// ```rust,ignore
/// #[pool_item]
/// impl MyStruct {
///     pub fn new(id: u64) -> Self { /* ... */ }
///
///     #[messaging(MyRequest, MyResponse)]
///     pub fn my_method(&self, arg: String) -> i32 { /* ... */ }
/// }
/// ```
///
/// # Parameters
///
/// - `Init = "TypeName"` - Use a custom initialization request type instead of
///   generating `{StructName}Init`
/// - `Shutdown = "method_name"` - Specify a method to call during pool shutdown
///
/// # Generated Types
///
/// For a struct `Foo` with method `bar`:
/// - `FooInit(u64)` - Initialization request (unless custom `Init` specified)
/// - `FooApi` - Enum containing all message variants
/// - `BarRequest(...)` - Request struct for the method
/// - `BarResponse { id, result }` - Response struct for the method
///
/// # Requirements
///
/// - The struct must implement `IdTargeted`
/// - The impl block must have a `pub fn new(...)` method
/// - Methods marked with `#[messaging]` must take `&self` or `&mut self`
#[proc_macro_attribute]
pub fn pool_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as PoolItemArgs);
    let input = parse_macro_input!(item as ItemImpl);
    generation::generate_pool_item_impl(input, args).into()
}
