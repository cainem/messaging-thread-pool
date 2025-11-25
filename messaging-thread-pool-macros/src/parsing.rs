use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token, Type,
};

#[derive(Debug)]
pub struct MessagingArgs {
    pub request_type: Ident,
    pub response_type: Ident,
}

impl Parse for MessagingArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        if vars.len() != 2 {
            return Err(input.error("Expected exactly 2 arguments: (RequestType, ResponseType)"));
        }
        let mut iter = vars.into_iter();
        Ok(MessagingArgs {
            request_type: iter.next().unwrap(),
            response_type: iter.next().unwrap(),
        })
    }
}

#[derive(Default)]
pub struct PoolItemArgs {
    pub init_type: Option<Type>,
    pub shutdown_method: Option<Ident>,
}

impl std::fmt::Debug for PoolItemArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolItemArgs")
            .field("init_type", &self.init_type.as_ref().map(|_| "Some(Type)"))
            .field("shutdown_method", &self.shutdown_method)
            .finish()
    }
}

impl Parse for PoolItemArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = PoolItemArgs::default();
        if input.is_empty() {
            return Ok(args);
        }

        let vars = Punctuated::<syn::Meta, Token![,]>::parse_terminated(input)?;
        for meta in vars {
            if let syn::Meta::NameValue(nv) = meta {
                if nv.path.is_ident("Init") {
                    if let syn::Expr::Path(path) = nv.value {
                        args.init_type = Some(Type::Path(syn::TypePath {
                            qself: None,
                            path: path.path,
                        }));
                    } else if let syn::Expr::Lit(lit) = nv.value {
                        if let syn::Lit::Str(lit_str) = lit.lit {
                            let path: syn::Path = lit_str.parse()?;
                            args.init_type = Some(Type::Path(syn::TypePath {
                                qself: None,
                                path,
                            }));
                        } else {
                            return Err(syn::Error::new_spanned(
                                lit,
                                "Expected string literal for Init",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            nv.value,
                            "Expected a type path or string literal for Init",
                        ));
                    }
                } else if nv.path.is_ident("Shutdown") {
                    if let syn::Expr::Path(path) = nv.value {
                        if let Some(ident) = path.path.get_ident() {
                            args.shutdown_method = Some(ident.clone());
                        } else {
                            return Err(syn::Error::new_spanned(
                                path,
                                "Expected an identifier for Shutdown",
                            ));
                        }
                    } else if let syn::Expr::Lit(lit) = nv.value {
                        if let syn::Lit::Str(lit_str) = lit.lit {
                            args.shutdown_method = Some(lit_str.parse()?);
                        } else {
                            return Err(syn::Error::new_spanned(
                                lit,
                                "Expected string literal for Shutdown",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            nv.value,
                            "Expected an identifier or string literal for Shutdown",
                        ));
                    }
                }
            }
        }
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse::Parser;

    #[test]
    fn test_parse_valid_args() {
        let parser = |input: syn::parse::ParseStream| MessagingArgs::parse(input);
        let tokens = quote! { Request, Response };
        let args = parser.parse2(tokens).expect("Failed to parse valid args");
        assert_eq!(args.request_type.to_string(), "Request");
        assert_eq!(args.response_type.to_string(), "Response");
    }

    #[test]
    fn test_parse_too_few_args() {
        let parser = |input: syn::parse::ParseStream| MessagingArgs::parse(input);
        let tokens = quote! { Request };
        let err = parser
            .parse2(tokens)
            .expect_err("Should fail with too few args");
        assert!(err
            .to_string()
            .contains("Expected exactly 2 arguments: (RequestType, ResponseType)"));
    }

    #[test]
    fn test_parse_too_many_args() {
        let parser = |input: syn::parse::ParseStream| MessagingArgs::parse(input);
        let tokens = quote! { Request, Response, Extra };
        let err = parser
            .parse2(tokens)
            .expect_err("Should fail with too many args");
        assert!(err
            .to_string()
            .contains("Expected exactly 2 arguments: (RequestType, ResponseType)"));
    }

    #[test]
    fn test_parse_empty() {
        let parser = |input: syn::parse::ParseStream| MessagingArgs::parse(input);
        let tokens = quote! {};
        let err = parser
            .parse2(tokens)
            .expect_err("Should fail with empty args");
        assert!(err
            .to_string()
            .contains("Expected exactly 2 arguments: (RequestType, ResponseType)"));
    }
}
