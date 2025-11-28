//! # Legacy API Specification Macro
//!
//! This module contains the `api_specification!` macro, which is the **legacy** way
//! to define pool items.
//!
//! ## ⚠️ Prefer `#[pool_item]` for New Code
//!
//! The `#[pool_item]` attribute macro is the recommended approach for new code.
//! It generates the same types with less boilerplate:
//!
//! ```rust,ignore
//! // OLD WAY (api_specification! macro)
//! api_specification!(
//!     pool_item: Counter,
//!     api_name: CounterApi,
//!     add_request: CounterAddRequest,
//!     calls: [
//!         { call_name: Increment, request: IncrementRequest, response: IncrementResponse },
//!     ]
//! );
//!
//! // NEW WAY (#[pool_item] macro)
//! #[pool_item]
//! impl Counter {
//!     pub fn new(id: u64) -> Self { /* ... */ }
//!
//!     #[messaging(IncrementRequest, IncrementResponse)]
//!     pub fn increment(&mut self, amount: i32) -> i32 { /* ... */ }
//! }
//! ```
//!
//! ## When You Might Still Need `api_specification!`
//!
//! - Complex generic bounds not supported by `#[pool_item]`
//! - Gradual migration of existing codebases
//! - Edge cases where more control over generated types is needed
//!
//! For new projects, start with `#[pool_item]` and only fall back to
//! `api_specification!` if you encounter limitations.

/// Generates an API enum and trait implementations for a pool item.
///
/// ## ⚠️ Legacy Macro
///
/// **This is the legacy approach.** For new code, prefer the [`pool_item`](macro@crate::pool_item)
/// attribute macro, which provides the same functionality with less boilerplate.
///
/// ## Migration Example
///
/// Before (with `api_specification!`):
/// ```rust,ignore
/// api_specification!(
///     pool_item: MyItem,
///     api_name: MyItemApi,
///     add_request: MyItemAddRequest,
///     calls: [
///         { call_name: DoWork, request: DoWorkRequest, response: DoWorkResponse },
///     ]
/// );
/// ```
///
/// After (with `#[pool_item]`):
/// ```rust,ignore
/// #[pool_item]
/// impl MyItem {
///     pub fn new(id: u64) -> Self { /* ... */ }
///
///     #[messaging(DoWorkRequest, DoWorkResponse)]
///     pub fn do_work(&self) { /* ... */ }
/// }
/// ```
///
/// # Parameters
///
/// - `pool_item`: The type of the pool item.
/// - `api_name`: The name of the generated API enum.
/// - `add_request`: The type for the add request.
/// - `calls`: A list of call definitions where each call consists of:
///     - `call_name`: The name of the call.
///     - `request`: The type of the request for the call.
///     - `response`: The type of the response for the call.
/// - `generics` (optional): The name of a generic type and its bound. Only required for the generic case.
///
/// # Example
///
/// ```ignore
/// // Non-generic case
/// api_specification!(
///     pool_item: Randoms,
///     api_name: RandomsApi,
///     add_request: RandomsAddRequest,
///     calls: [
///         { call_name: Mean, request: MeanRequest, response: MeanResponse },
///         { call_name: Sum, request: SumRequest, response: SumResponse },
///         { call_name: Panic, request: PanicRequest, response: PanicResponse },
///     ]
/// );
///
/// // Generic case
/// api_specification!(
///     pool_item: RandomsBatch<T>,
///     api_name: RandomsBatchApi,
///     add_request: RandomsBatchAddRequest<T>,
///     calls: [
///         { call_name: SumOfSums, request: SumOfSumsRequest, response: SumOfSumsResponse },
///     ],
///     generics: T: InnerThreadPool
/// );
/// ```
///
/// This will generate an enum `RandomsApi` with variants `Mean`, `Sum`, and `Panic` for the non-generic case,
/// and `RandomsBatchApi` with the `SumOfSums` variant for the generic case, along with various generics implementations
/// and conversion functions for the provided request and response types.
#[macro_export]
macro_rules! api_specification {
    // Match for the generic case with a trailing comma and a generics bound
    (
        pool_item: $pool_item:ty,
        api_name: $api:ident,
        add_request: $add_request:ty,
        calls: [
            $(
                {
                    call_name: $call:ident,
                    request: $request:ty,
                    response: $response:ty
                }
            ),* $(,)?
        ],
        generics: $t:ident: $generics:ident
    ) => {
        #[derive(Debug, PartialEq)]
        pub enum $api<$t: $generics> {
            $(
                $call(RequestResponse<$pool_item, $request>),
            )*
        }

        impl<$t: $generics> IdTargeted for $api<$t> {
            fn id(&self) -> u64 {
                match self {
                    $(
                        $api::$call(request) => request.id(),
                    )*
                }
            }
        }

        impl<$t: $generics> RequestWithResponse<$pool_item> for $add_request {
            type Response = AddResponse;
        }

        impl<$t: $generics> From<$add_request> for ThreadRequestResponse<$pool_item> {
            fn from(add_request: $add_request) -> Self {
                ThreadRequestResponse::<$pool_item>::AddPoolItem(RequestResponse::Request(add_request))
            }
        }

        impl<$t: $generics> From<ThreadRequestResponse<$pool_item>> for $add_request {
            fn from(response: ThreadRequestResponse<$pool_item>) -> Self {
                let ThreadRequestResponse::AddPoolItem(RequestResponse::Request(result)) = response else {
                    panic!("not expected")
                };
                result
            }
        }

        $(
            impl<$t: $generics> RequestWithResponse<$pool_item> for $request {
                type Response = $response;
            }

            impl<$t: $generics> From<$request> for ThreadRequestResponse<$pool_item> {
                fn from(request: $request) -> Self {
                    ThreadRequestResponse::MessagePoolItem($api::$call(RequestResponse::Request(
                        request,
                    )))
                }
            }

            impl<$t: $generics> From<ThreadRequestResponse<$pool_item>> for $request {
                fn from(request: ThreadRequestResponse<$pool_item>) -> Self {
                    let ThreadRequestResponse::MessagePoolItem($api::$call(
                        RequestResponse::Request(result),
                    )) = request else {
                        panic!("not expected")
                    };
                    result
                }
            }

            impl<$t: $generics> From<$response> for ThreadRequestResponse<$pool_item> {
                fn from(response: $response) -> Self {
                    ThreadRequestResponse::MessagePoolItem($api::$call(RequestResponse::Response(
                        response,
                    )))
                }
            }

            impl<$t: $generics> From<ThreadRequestResponse<$pool_item>> for $response {
                fn from(response: ThreadRequestResponse<$pool_item>) -> Self {
                    let ThreadRequestResponse::MessagePoolItem($api::$call(
                        RequestResponse::Response(result),
                    )) = response else {
                        panic!("not expected")
                    };
                    result
                }
            }
        )*
    };

    // Match for the non-generic case with a trailing comma
    (
        pool_item: $pool_item:ty,
        api_name: $api:ident,
        add_request: $add_request:ty,
        calls: [
            $(
                {
                    call_name: $call:ident,
                    request: $request:ty,
                    response: $response:ty
                }
            ),* $(,)?
        ]
    ) => {
        #[derive(Debug, PartialEq)]
        pub enum $api {
            $(
                $call(RequestResponse<$pool_item, $request>),
            )*
        }

        impl IdTargeted for $api {
            fn id(&self) -> u64 {
                match self {
                    $(
                        $api::$call(request) => request.id(),
                    )*
                }
            }
        }

        impl RequestWithResponse<$pool_item> for $add_request {
            type Response = AddResponse;
        }

        impl From<$add_request> for ThreadRequestResponse<$pool_item> {
            fn from(add_request: $add_request) -> Self {
                ThreadRequestResponse::<$pool_item>::AddPoolItem(RequestResponse::Request(add_request))
            }
        }

        impl From<ThreadRequestResponse<$pool_item>> for $add_request {
            fn from(response: ThreadRequestResponse<$pool_item>) -> Self {
                let ThreadRequestResponse::AddPoolItem(RequestResponse::Request(result)) = response else {
                    panic!("not expected")
                };
                result
            }
        }

        $(
            impl RequestWithResponse<$pool_item> for $request {
                type Response = $response;
            }

            impl From<$request> for ThreadRequestResponse<$pool_item> {
                fn from(request: $request) -> Self {
                    ThreadRequestResponse::MessagePoolItem($api::$call(RequestResponse::Request(
                        request,
                    )))
                }
            }

            impl From<ThreadRequestResponse<$pool_item>> for $request {
                fn from(request: ThreadRequestResponse<$pool_item>) -> Self {
                    let ThreadRequestResponse::MessagePoolItem($api::$call(
                        RequestResponse::Request(result),
                    )) = request else {
                        panic!("not expected")
                    };
                    result
                }
            }

            impl From<$response> for ThreadRequestResponse<$pool_item> {
                fn from(response: $response) -> Self {
                    ThreadRequestResponse::MessagePoolItem($api::$call(RequestResponse::Response(
                        response,
                    )))
                }
            }

            impl From<ThreadRequestResponse<$pool_item>> for $response {
                fn from(response: ThreadRequestResponse<$pool_item>) -> Self {
                    let ThreadRequestResponse::MessagePoolItem($api::$call(
                        RequestResponse::Response(result),
                    )) = response else {
                        panic!("not expected")
                    };
                    result
                }
            }
        )*
    };
}
