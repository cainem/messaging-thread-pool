use crate::parsing::MessagingArgs;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, Ident, ImplItem, ItemImpl, ReturnType, Type};

pub fn generate_pool_item_impl(mut input: ItemImpl) -> TokenStream {
    let self_ty = &input.self_ty;

    let struct_name = if let Type::Path(type_path) = self_ty.as_ref() {
        &type_path.path.segments.last().unwrap().ident
    } else {
        return quote! { compile_error!("Expected struct type"); };
    };

    let api_name = format_ident!("{}Api", struct_name);
    let init_name = format_ident!("{}Init", struct_name);

    let mut generated_items = Vec::new();
    let mut api_variants = Vec::new();
    let mut request_names = Vec::new();
    let mut process_message_arms = Vec::new();
    let mut type_aliases = Vec::new();

    for item in &mut input.items {
        if let ImplItem::Fn(method) = item {
            let mut messaging_attr_index = None;
            for (i, attr) in method.attrs.iter().enumerate() {
                if attr.path().is_ident("messaging") {
                    messaging_attr_index = Some(i);
                    break;
                }
            }

            if let Some(index) = messaging_attr_index {
                let attr = method.attrs.remove(index);
                let args: MessagingArgs = match attr.parse_args() {
                    Ok(args) => args,
                    Err(e) => return e.to_compile_error(),
                };

                let request_name = args.request_type;
                let response_name = args.response_type;
                let method_name = &method.sig.ident;

                // Parse arguments
                let mut request_fields: Vec<Type> = Vec::new();
                // Always add ID as first field
                request_fields.push(syn::parse_quote!(u64));

                for input in &method.sig.inputs {
                    if let FnArg::Typed(pat_type) = input {
                        let ty = &pat_type.ty;
                        request_fields.push(*ty.clone());
                    }
                }

                // Parse return type
                let return_type = match &method.sig.output {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => Some(ty.clone()),
                };

                generated_items.push(generate_request_struct(
                    &request_name,
                    &request_fields,
                    struct_name,
                    &response_name,
                    &api_name,
                ));

                let result_type = return_type.unwrap_or_else(|| syn::parse_quote!(()));
                generated_items.push(generate_response_struct(
                    &response_name,
                    &result_type,
                    struct_name,
                    &api_name,
                    &request_name,
                ));

                generated_items.push(generate_from_response_impl(
                    &response_name,
                    struct_name,
                    &api_name,
                    &request_name,
                ));

                let alias_name = format_ident!("{}_{}_RequestResponse", struct_name, request_name);
                type_aliases.push(quote! {
                    #[allow(non_camel_case_types)]
                    pub type #alias_name = messaging_thread_pool::request_response::RequestResponse<#struct_name, #request_name>;
                });

                api_variants.push(quote! {
                    #request_name(#alias_name)
                });
                request_names.push(request_name.clone());

                process_message_arms.push(generate_process_message_arm(
                    &api_name,
                    &request_name,
                    method_name,
                    &request_fields,
                    &response_name,
                ));
            }
        }
    }

    generated_items.push(generate_api_enum(
        &api_name,
        &type_aliases,
        &api_variants,
        &request_names,
    ));

    generated_items.push(generate_init_struct(&init_name, struct_name));

    generated_items.push(generate_pool_item_trait_impl(
        self_ty,
        &init_name,
        &api_name,
        &process_message_arms,
    ));

    quote! {
        #input
        #(#generated_items)*
    }
}

fn generate_request_struct(
    request_name: &Ident,
    request_fields: &[Type],
    struct_name: &Ident,
    response_name: &Ident,
    api_name: &Ident,
) -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #request_name( #(pub #request_fields),* );

        impl messaging_thread_pool::IdTargeted for #request_name {
            fn id(&self) -> u64 {
                self.0
            }
        }

        impl messaging_thread_pool::RequestWithResponse<#struct_name> for #request_name {
            type Response = #response_name;
        }

        impl From<#request_name> for messaging_thread_pool::ThreadRequestResponse<#struct_name> {
            fn from(request: #request_name) -> Self {
                messaging_thread_pool::ThreadRequestResponse::MessagePoolItem(
                    #api_name::#request_name(
                        messaging_thread_pool::request_response::RequestResponse::Request(request)
                    )
                )
            }
        }
    }
}

fn generate_response_struct(
    response_name: &Ident,
    result_type: &Type,
    struct_name: &Ident,
    api_name: &Ident,
    request_name: &Ident,
) -> TokenStream {
    quote! {
        #[derive(Debug, Clone)]
        pub struct #response_name {
            pub id: u64,
            pub result: #result_type,
        }

        impl From<messaging_thread_pool::ThreadRequestResponse<#struct_name>> for #response_name {
            fn from(response: messaging_thread_pool::ThreadRequestResponse<#struct_name>) -> Self {
                if let messaging_thread_pool::ThreadRequestResponse::MessagePoolItem(
                    #api_name::#request_name(
                        messaging_thread_pool::request_response::RequestResponse::Response(res)
                    )
                ) = response {
                    res
                } else {
                    panic!("Unexpected response type")
                }
            }
        }
    }
}

fn generate_from_response_impl(
    response_name: &Ident,
    struct_name: &Ident,
    api_name: &Ident,
    request_name: &Ident,
) -> TokenStream {
    quote! {
        impl From<#response_name> for messaging_thread_pool::ThreadRequestResponse<#struct_name> {
            fn from(response: #response_name) -> Self {
                messaging_thread_pool::ThreadRequestResponse::MessagePoolItem(
                    #api_name::#request_name(
                        messaging_thread_pool::request_response::RequestResponse::Response(response)
                    )
                )
            }
        }
    }
}

fn generate_process_message_arm(
    api_name: &Ident,
    request_name: &Ident,
    method_name: &Ident,
    request_fields: &[Type],
    response_name: &Ident,
) -> TokenStream {
    let call_args = if request_fields.len() > 1 {
        let indices = (1..request_fields.len()).map(syn::Index::from);
        quote! { #(request.#indices),* }
    } else {
        quote! {}
    };

    quote! {
        #api_name::#request_name(request_response) => {
            let request = match request_response {
                messaging_thread_pool::request_response::RequestResponse::Request(r) => r,
                _ => panic!("Unexpected message in process_message (expected Request)"),
            };
            let result = self.#method_name(#call_args);
            #response_name { id: messaging_thread_pool::IdTargeted::id(&request), result }.into()
        }
    }
}

fn generate_api_enum(
    api_name: &Ident,
    type_aliases: &[TokenStream],
    api_variants: &[TokenStream],
    request_names: &[Ident],
) -> TokenStream {
    quote! {
        #(#type_aliases)*

        #[derive(Debug)]
        pub enum #api_name {
            #(#api_variants),*
        }

        impl messaging_thread_pool::IdTargeted for #api_name {
            fn id(&self) -> u64 {
                match self {
                    #(
                        #api_name::#request_names(req) => req.id(),
                    )*
                }
            }
        }
    }
}

fn generate_init_struct(init_name: &Ident, struct_name: &Ident) -> TokenStream {
    quote! {
        #[derive(Debug)]
        pub struct #init_name(pub u64);

        impl messaging_thread_pool::IdTargeted for #init_name {
            fn id(&self) -> u64 {
                self.0
            }
        }

        impl messaging_thread_pool::RequestWithResponse<#struct_name> for #init_name {
            type Response = messaging_thread_pool::thread_request_response::AddResponse;
        }

        impl From<#init_name> for messaging_thread_pool::ThreadRequestResponse<#struct_name> {
            fn from(request: #init_name) -> Self {
                messaging_thread_pool::ThreadRequestResponse::AddPoolItem(
                    messaging_thread_pool::request_response::RequestResponse::Request(request)
                )
            }
        }
    }
}

fn generate_pool_item_trait_impl(
    self_ty: &Type,
    init_name: &Ident,
    api_name: &Ident,
    process_message_arms: &[TokenStream],
) -> TokenStream {
    quote! {
        impl messaging_thread_pool::PoolItem for #self_ty {
            type Init = #init_name;
            type Api = #api_name;
            type ThreadStartInfo = ();

            fn process_message(&mut self, request: Self::Api) -> messaging_thread_pool::ThreadRequestResponse<Self> {
                match request {
                    #(#process_message_arms)*
                    _ => panic!("Unexpected message or response in process_message"),
                }
            }

            fn name() -> &'static str {
                stringify!(#self_ty)
            }

            fn new_pool_item(request: Self::Init) -> Result<Self, messaging_thread_pool::pool_item::NewPoolItemError> {
                Ok(Self::new(request.0))
            }

            fn shutdown_pool(&self) -> Vec<messaging_thread_pool::thread_request_response::ThreadShutdownResponse> {
                Vec::default()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_generate_pool_item_impl_basic() {
        let input: ItemImpl = parse_quote! {
            impl MyStruct {
                #[messaging(MyRequest, MyResponse)]
                pub fn my_method(&self, arg1: u32) -> bool {
                    true
                }
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();

        // Check for key generated components
        assert!(output_str.contains("struct MyRequest"));
        assert!(output_str.contains("struct MyResponse"));
        assert!(output_str.contains("enum MyStructApi"));
        assert!(output_str.contains("impl messaging_thread_pool :: PoolItem for MyStruct"));
        assert!(output_str.contains("MyStruct_MyRequest_RequestResponse"));
    }

    #[test]
    fn test_generate_pool_item_impl_multiple_methods() {
        let input: ItemImpl = parse_quote! {
            impl MyStruct {
                #[messaging(Req1, Resp1)]
                pub fn method1(&self) {}

                #[messaging(Req2, Resp2)]
                pub fn method2(&self) {}
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();

        assert!(output_str.contains("struct Req1"));
        assert!(output_str.contains("struct Resp1"));
        assert!(output_str.contains("struct Req2"));
        assert!(output_str.contains("struct Resp2"));
        assert!(output_str.contains("Req1 (MyStruct_Req1_RequestResponse)"));
        assert!(output_str.contains("Req2 (MyStruct_Req2_RequestResponse)"));
        // Check that method call is generated
        assert!(output_str.contains("self . method1"));
    }

    #[test]
    fn test_generate_pool_item_impl_no_return_type() {
        let input: ItemImpl = parse_quote! {
            impl MyStruct {
                #[messaging(Req, Resp)]
                pub fn method(&self) {}
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();

        assert!(output_str.contains("pub result : ()"));
    }

    #[test]
    fn test_generate_pool_item_impl_error_not_struct() {
        // This is hard to test directly because parse_macro_input! expects ItemImpl which has a Type.
        // But we can pass an ItemImpl where self_ty is not a path.
        // However, syn::parse_quote! usually produces valid types.
        // Let's try to construct one manually or just skip this edge case if it's hard to trigger with parse_quote.
        // Actually, `impl &str` is valid syntax but not a struct path.
        let input: ItemImpl = parse_quote! {
            impl &str {
                #[messaging(Req, Resp)]
                fn method(&self) {}
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();
        assert!(output_str.contains("compile_error ! (\"Expected struct type\")"));
    }

    #[test]
    fn test_generate_pool_item_impl_invalid_attr() {
        let input: ItemImpl = parse_quote! {
            impl MyStruct {
                #[messaging(OnlyOneArg)]
                pub fn method(&self) {}
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();
        assert!(output_str.contains("compile_error"));
    }

    #[test]
    fn test_generate_pool_item_impl_ignore_const() {
        let input: ItemImpl = parse_quote! {
            impl MyStruct {
                const MY_CONST: u32 = 0;

                #[messaging(Req, Resp)]
                pub fn method(&self) {}
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();
        // Should still generate for method
        assert!(output_str.contains("struct Req"));
    }

    #[test]
    fn test_generate_pool_item_impl_no_attr() {
        let input: ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn method(&self) {}
            }
        };

        let output = generate_pool_item_impl(input);
        let output_str = output.to_string();
        // Should generate empty Api enum and PoolItem impl
        assert!(output_str.contains("enum MyStructApi"));
        assert!(output_str.contains("impl messaging_thread_pool :: PoolItem for MyStruct"));
    }
}
