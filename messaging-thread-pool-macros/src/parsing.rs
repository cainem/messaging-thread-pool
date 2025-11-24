use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
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
