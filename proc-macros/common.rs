use ohelpers_proc_macros::parse_utils::parse_peekables_until;
use ohelpers_proc_macros::{format_ident_if, quote_if, unwrap, TokenStream2};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_quote, Result, Token};

/// The type `Into<T>` names, when that is what the property was declared as.
///
/// This used to be `ty_name.starts_with("Into")` over the type's printed form, followed by
/// `replace("Into", "")` and stripping every angle bracket. Three things went wrong with
/// that. A user type named `IntoThing` was silently rewritten to `Thing`, and so would
/// `IntoIterator` be. A nested argument such as `Into<Vec<u8>>` lost its inner brackets and
/// left `Vec u8`, which does not parse, so the macro panicked. And `Into<IntoThing>` had
/// both occurrences of `Into` removed, because `replace` is not `strip_prefix`.
///
/// Matching the parsed type instead: a path whose last segment is exactly `Into` and which
/// carries exactly one type argument.
fn into_argument(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(type_path) = strip_groups(ty) else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }

    let segment = type_path.path.segments.last()?;
    if segment.ident != "Into" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };
    if arguments.args.len() != 1 {
        return None;
    }

    match arguments.args.first()? {
        syn::GenericArgument::Type(inner) => Some(inner.clone()),
        _ => None,
    }
}

/// Looks through the invisible grouping a `$ty:ty` fragment arrives inside.
///
/// A matcher fragment reaches a proc macro wrapped in a `None`-delimited group, which syn
/// represents as `Type::Group`, so matching on `Type::Path` directly never fires for a type
/// that came from a `macro_rules` capture. Printing the type saw through it, which is why
/// the string version appeared to work.
fn strip_groups(ty: &syn::Type) -> &syn::Type {
    let mut current = ty;
    loop {
        current = match current {
            syn::Type::Group(group) => &group.elem,
            syn::Type::Paren(paren) => &paren.elem,
            other => return other,
        };
    }
}

/// Whether a standalone `_` placeholder comes before the end of the return specification.
///
/// This used to ask whether the remaining stream's printed form contained an underscore
/// anywhere, which is true of any identifier carrying one, so a return type spelled
/// `Bar_Baz` took the placeholder branch and was assembled as a prefix and a postfix around
/// the property type. A `_` placeholder is its own token; an identifier containing an
/// underscore is a single `Ident` token and is not one.
fn has_placeholder(input: ParseStream) -> bool {
    let fork = input.fork();
    while !fork.is_empty() {
        if fork.peek(Token![_]) {
            return true;
        }
        if fork.parse::<proc_macro2::TokenTree>().is_err() {
            return false;
        }
    }
    false
}

struct PropertyDslInput {
    name:              syn::Ident,
    ty:                syn::Type,
    return_type:       syn::Type,
    is_into_variant:   bool,
    use_getter_prefix: bool,
    impl_with:         bool,
    param_prefix:      TokenStream2,
    param_postfix:     TokenStream2,
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
        let is_into_variant = match into_argument(&ty) {
            Some(inner) => {
                ty = inner;
                true
            },
            None => false,
        };
        input.parse::<Token![=]>()?;
        let param_prefix = parse_peekables_until(input, Token![_]).ok();
        let _: Token![_] = input.parse()?;
        let param_postfix = parse_peekables_until(input, Token![->]).ok();
        input.parse::<Token![->]>().expect("Expected the -> token");
        // The return specification is either a `_` placeholder with optional tokens around
        // it, which is assembled around the property type, or a type written out in full.
        let return_type: Option<syn::Type> = if has_placeholder(input) {
            let ret_prefix = parse_peekables_until(input, Token![_])
                .expect("Expected the type placeholder token in the correct place");
            let _: Token![_] = input.parse()?;
            let ret_postfix =
                parse_peekables_until(input, Token![where]).unwrap_or_default();
            Some(parse_quote!(#ret_prefix #ty #ret_postfix))
        } else {
            input.parse().ok()
        };
        let mut where_clause: Option<TokenStream2> = None;
        if !input.is_empty() {
            if input.parse::<Token![where]>().is_ok() {
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
            where_clause,
        })
    }
}

// the `#[proc_macro]` entry point is generated in lib.rs; this is the implementation behind it
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
    // the builder takes `self` by value and returns it, which a trait method may only do when
    // the implementor is sized, so that bound is added to whatever the caller asked for
    let with_where = if where_clause.is_some() {
        quote!(where Self: Sized, #where_clause)
    } else {
        quote!(where Self: Sized)
    };
    let with_block = quote_if!(impl_with, {
        fn #with #func_signature (mut self, value: #value_ty) -> Self #with_where {
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
