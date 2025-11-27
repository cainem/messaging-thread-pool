/// This macro generates an API enum and implements various generics and conversions for provided types.
///
/// **Note:** For simple non-generic cases, consider using the `#[pool_item]` attribute macro instead.
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
