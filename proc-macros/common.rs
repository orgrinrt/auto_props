use std::collections::btree_map::OccupiedEntry;

use proc_macro::TokenStream;
use proc_macro_misc_helpers::parse_utils::{parse_peekables_until, parse_until};
use proc_macro_misc_helpers::{format_ident_if, quote_if, token_name, unwrap, TokenStream2};
use quote::{format_ident, quote};
use syn::parse::{Error, Parse, ParseStream};
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, Result, ReturnType, Token};

struct PropertyDslInput {
    name:              syn::Ident,
    ty:                syn::Type,
    return_type:       syn::Type,
    is_into_variant:   bool,
    use_getter_prefix: bool,
    impl_with:         bool,
    param_prefix:      TokenStream2,
    param_postfix:     TokenStream2,
    ret_prefix:        TokenStream2,
    ret_postfix:       TokenStream2,
    where_clause:      Option<TokenStream2>,
}

impl Parse for PropertyDslInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: syn::Ident = input
            .parse()
            .expect("Expected a valid ident as property name");
        input
            .parse::<Token![:]>()
            .expect("Expected : token after property name");
        let mut ty: syn::Type = input.parse().expect("Expected a type after property name");
        let ty_name: String = token_name!(parsable ty ty);
        let is_into_variant = ty_name.starts_with("Into");
        if is_into_variant {
            let ty_name_inner = ty_name
                .replace("Into", "")
                .replace("<", "")
                .replace(">", "");
            ty = syn::parse_str::<syn::Type>(ty_name_inner.as_str()).expect(
                format!(
                    "Expected inner to be valid type, but got `{}`",
                    ty_name_inner
                )
                .as_str(),
            );
        }
        input.parse::<Token![=]>()?;
        let param_prefix = parse_peekables_until(input, Token![_]).ok();
        let _: Token![_] = input.parse()?;
        let param_postfix = parse_peekables_until(input, Token![->]).ok();
        input.parse::<Token![->]>().expect("Expected the -> token");
        let mut ret_prefix: TokenStream2 = TokenStream2::new();
        let mut ret_postfix: TokenStream2 = TokenStream2::new();
        let mut return_type: Option<syn::Type> = None;
        if input.to_string().contains("_") {
            ret_prefix = parse_peekables_until(input, Token![_]).expect(
                "Expected the type \
            placeholder token in the correct place",
            );
            let _: Token![_] = input.parse()?;
            let maybe_ret_postfix = parse_peekables_until(input, Token![where]).ok();
            if let Some(postfix) = maybe_ret_postfix {
                ret_postfix = postfix;
            }
            return_type = Some(parse_quote!(#ret_prefix #ty #ret_postfix));
        } else {
            return_type = input.parse().ok();
        }
        let mut where_clause: Option<TokenStream2> = None;
        if !input.is_empty() {
            let w: Option<Token![where]> = input.parse().ok();
            if let Some(w) = w {
                let body;
                unwrap!(braces body in input);
                let body_stream: TokenStream2 = body.parse().expect(
                    "Expected valid token stream \
                from the body",
                );
                where_clause = Some(body_stream);
            }
        }

        #[cfg(feature = "getter_prefix")]
        let use_getter_prefix = true;
        #[cfg(not(feature = "getter_prefix"))]
        let use_getter_prefix = false;

        #[cfg(feature = "impl_with")]
        let impl_with = true;
        #[cfg(not(feature = "impl_with"))]
        let impl_with = false;

        Ok(PropertyDslInput {
            name,
            ty,
            return_type: return_type.expect("Expected a valid return type for the property"),
            is_into_variant,
            use_getter_prefix,
            impl_with,
            param_prefix: quote_if!(some param_prefix),
            param_postfix: quote_if!(some param_postfix),
            ret_prefix,
            ret_postfix,
            where_clause,
        })
    }
}

#[proc_macro]
pub fn common(input: TokenStream) -> TokenStream {
    let PropertyDslInput {
        name,
        ty,
        return_type,
        is_into_variant,
        use_getter_prefix,
        impl_with,
        param_prefix,
        param_postfix,
        ret_prefix,
        ret_postfix,
        where_clause,
    } = parse_macro_input!(input as PropertyDslInput);
    let setter = format_ident!("set_{}", name);
    let getter = format_ident_if!(use_getter_prefix, "get_{}", name);
    let with = format_ident!("with_{}", name);
    let func_signature = quote_if!(is_into_variant, {<T: Into<#ty >>});
    let value_ty = if is_into_variant {
        quote!(#param_prefix T #param_postfix)
    } else {
        quote!
    (#param_prefix #ty #param_postfix)
    };
    let where_block = quote_if!(where_clause.is_some(), {
        where #where_clause
    });
    let with_block = quote_if!(impl_with, {
        fn #with #func_signature (mut self, value: #value_ty) -> Self #where_block {
                self. #setter (value);
                self
            }
    });
    let output = quote! {
        fn #getter #func_signature (&self) -> #return_type #where_block;
        fn #setter #func_signature (&mut self, value: #value_ty) #where_block;
        #with_block
    };

    output.into()
}
