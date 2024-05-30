/// Facilitates the binding of response messages to API calls within a multi-threaded context.
///
/// This macro streamlines the process of:
///
/// 1. Converting a response message (`$msg_type`) into a `ThreadRequestResponse` that encapsulates the message and associates it with a specific API variant (`$api_variant`).
/// 2. Converting a `ThreadRequestResponse` back into the original response message.
///
/// It provides two distinct use cases:
///
/// - **Non-Generic Case:**  Handles scenarios where both the response message type and the pool item type are concrete (not generic).
/// - **Generic Case:**  Accommodates cases where the response message type is concrete, but the pool item type is generic, potentially with additional constraints on its type parameters.
///
/// By default, all generic type parameters in the generic case are automatically constrained to be `Send`, `Debug`, and `Sync` to ensure thread-safety.
/// Additional constraints can be specified by the user.
///
/// # Parameters
///
/// * `$msg_type:ty` - The type of the response message.
/// * `$pool_item:ty` (Non-Generic) / `$pool_item:ident<$($gen_param:ident),+>` (Generic) -
///   - **Non-Generic:** The concrete type representing a pool item, typically used for thread-safety.
///   - **Generic:**  The generic type representing a pool item. The `$gen_param` identifiers represent the type parameters of this generic type.
/// * `$api_variant:path` - The specific variant of the API enum (e.g., `RandomsApi::Mean`) that handles this response.
/// * `$($additional_constraints:tt)+` (Generic Only) -  Optional additional trait bounds (constraints) for the generic type parameters (`$gen_param`) of the pool item type.
///
/// # Examples
///
/// **Non-Generic:**
///
/// ```rust, ignore
/// bind_response_to_api!(MeanResponse, Randoms, RandomsApi::Mean);
/// ```
///
/// **Generic:**
///
/// ```rust, ignore
/// bind_response_to_api!(
///     SumOfSumsResponse,
///     RandomsBatch<P>,
///     RandomsBatchApi::SumOfSums,
///     P: SenderAndReceiver<Randoms>
/// );
/// ```
#[macro_export]
macro_rules! bind_response_to_api {
    // Non-generic case
    ($msg_type:ty, $pool_item:ty, $api_variant:path) => {
        impl From<$msg_type> for ThreadRequestResponse<$pool_item> {
            fn from(response: $msg_type) -> Self {
                ThreadRequestResponse::MessagePoolItem($api_variant(
                    RequestResponse::Response(response),
                ))
            }
        }

        impl From<ThreadRequestResponse<$pool_item>> for $msg_type {
            fn from(response: ThreadRequestResponse<$pool_item>) -> Self {
                if let ThreadRequestResponse::MessagePoolItem($api_variant(RequestResponse::Response(result))) = response {
                    result
                } else {
                    panic!("not expected")
                }
            }
        }
    };

    // Generic case with automatic Send, Debug, Sync constraints and user-defined additional constraints
    ($msg_type:ty, $pool_item:ident<$($gen_param:ident),+>, $api_variant:path, $($additional_constraints:tt)+) => {
        impl<$($gen_param),+> From<$msg_type> for ThreadRequestResponse<$pool_item<$($gen_param),+>>
        where
            $($gen_param: Send + Debug + Sync),+,
            $($additional_constraints)+
        {
            fn from(response: $msg_type) -> Self {
                ThreadRequestResponse::MessagePoolItem($api_variant(
                    RequestResponse::Response(response),
                ))
            }
        }

        impl<$($gen_param),+> From<ThreadRequestResponse<$pool_item<$($gen_param),+>>> for $msg_type
        where
            $($gen_param: Send + Debug + Sync),+,
            $($additional_constraints)+
        {
            fn from(response: ThreadRequestResponse<$pool_item<$($gen_param),+>>) -> Self {
                if let ThreadRequestResponse::MessagePoolItem($api_variant(RequestResponse::Response(result))) = response {
                    result
                } else {
                    panic!("unexpected")
                }
            }
        }
    };
}

/// This macro generates implementations for tying together request and response types,
/// and for enabling conversions between request types and `ThreadRequestResponse` types,
/// and vice versa.
///
/// # Parameters
/// - `request_type`: The type of the request.
/// - `pool_item_type`: The type of the pool item.
/// - `api_variant_type`: The type of the API variant.
/// - `response_type`: The type of the response.
/// - `generic_type`: The generic parameter (used in the generic case).
/// - `primary_constraint`: The primary constraint for the generic parameter (e.g., `SenderAndReceiver<Randoms>`).
///
/// # Constraints
/// For generic types, the macro automatically adds `Send + Debug + Sync` constraints in addition
/// to the specified primary constraint.
///
/// # Examples
///
/// ## Non-Generic Case
/// ```rust, ignore
/// bind_request_to_response!(
///     MeanRequest,
///     Randoms,
///     RandomsApi::Mean,
///     MeanResponse
/// );
/// ```
/// This generates the following code:
/// ```rust, ignore
/// /// ties together the request with a response
/// impl RequestWithResponse<Randoms> for MeanRequest {
///     type Response = MeanResponse;
/// }
///
/// // enable the conversion of the request to the required ThreadRequestResponse
/// impl From<MeanRequest> for ThreadRequestResponse<Randoms> {
///     fn from(request: MeanRequest) -> Self {
///         ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(request)))
///     }
/// }
///
/// // enable the conversion from the a ThreadRequestResponse
/// impl From<ThreadRequestResponse<Randoms>> for MeanRequest {
///     fn from(request: ThreadRequestResponse<Randoms>) -> Self {
///         let ThreadRequestResponse::MessagePoolItem(RandomsApi::Mean(RequestResponse::Request(
///             result,
///         ))) = request
///         else {
///             panic!("not expected")
///         };
///         result
///     }
/// }
/// ```
///
/// ## Generic Case
/// ```rust, ignore
/// bind_request_to_response!(
///     SumOfSumsRequest,
///     RandomsBatch<P>,
///     RandomsBatchApi::SumOfSums,
///     SumOfSumsResponse,
///     P: SenderAndReceiver<Randoms>
/// );
/// ```
/// This generates the following code:
/// ```rust, ignore
/// /// ties together the request with a response
/// impl<P: SenderAndReceiver<Randoms> + Send + Debug + Sync> RequestWithResponse<RandomsBatch<P>> for SumOfSumsRequest {
///     type Response = SumOfSumsResponse;
/// }
///
/// impl<P: SenderAndReceiver<Randoms> + Send + Debug + Sync> From<SumOfSumsRequest> for ThreadRequestResponse<RandomsBatch<P>> {
///     fn from(request: SumOfSumsRequest) -> Self {
///         ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
///             RequestResponse::Request(request),
///         ))
///     }
/// }
///
/// impl<P: SenderAndReceiver<Randoms> + Send + Debug + Sync> From<ThreadRequestResponse<RandomsBatch<P>>> for SumOfSumsRequest {
///     fn from(request: ThreadRequestResponse<RandomsBatch<P>>) -> Self {
///         let ThreadRequestResponse::MessagePoolItem(RandomsBatchApi::SumOfSums(
///             RequestResponse::Request(result),
///         )) = request
///         else {
///             panic!("not expected")
///         };
///         result
///     }
/// }
/// ```
#[macro_export]
macro_rules! bind_request_to_response {
    // Case for non-generic request
    ($request:ty, $pool_item:ty, $api_variant:path, $response:ty) => {
        /// ties together the request with a response
        impl RequestWithResponse<$pool_item> for $request {
            type Response = $response;
        }

        // enable the conversion of the request to the require ThreadRequestResponse
        impl From<$request> for ThreadRequestResponse<$pool_item> {
            fn from(request: $request) -> Self {
                ThreadRequestResponse::MessagePoolItem($api_variant(RequestResponse::Request(
                    request,
                )))
            }
        }

        // enable the conversion from the a ThreadRequestResponse
        impl From<ThreadRequestResponse<$pool_item>> for $request {
            fn from(request: ThreadRequestResponse<$pool_item>) -> Self {
                let ThreadRequestResponse::MessagePoolItem($api_variant(RequestResponse::Request(
                    result,
                ))) = request
                else {
                    panic!("not expected")
                };
                result
            }
        }
    };
   // Case for generic request
   ($request:ty, $pool_item:ty, $api_variant:path, $response:ty, $($generics:ident: $bound:path),+) => {
        /// ties together the request with a response
        impl<$($generics: $bound + Send + Debug + Sync),+> RequestWithResponse<$pool_item> for $request {
            type Response = $response;
        }

        impl<$($generics: $bound + Send + Debug + Sync),+> From<$request> for ThreadRequestResponse<$pool_item> {
            fn from(request: $request) -> Self {
                ThreadRequestResponse::MessagePoolItem($api_variant(
                    RequestResponse::Request(request),
                ))
            }
        }

        impl<$($generics: $bound + Send + Debug + Sync),+> From<ThreadRequestResponse<$pool_item>> for $request {
            fn from(request: ThreadRequestResponse<$pool_item>) -> Self {
                let ThreadRequestResponse::MessagePoolItem($api_variant(
                    RequestResponse::Request(result),
                )) = request
                else {
                    panic!("not expected")
                };
                result
            }
        }
    };

}
