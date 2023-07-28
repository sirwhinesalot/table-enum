#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, parse_quote, Expr, ExprMatch, Field, Fields, Ident, ImplItemFn, Token, Variant, Visibility, Attribute};

#[derive(Debug)]
struct TableEnum {
    attrs: Vec<Attribute>,
    visibility: Visibility,
    ident: Ident,
    types: Punctuated<Field, Comma>,
    members: Punctuated<TableEnumVariant, Comma>,
}

#[derive(Debug)]
struct TableEnumVariant {
    ident: Ident,
    values: Punctuated<Expr, Comma>,
}

impl Parse for TableEnumVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let values = content.parse_terminated(Expr::parse, Token![,])?;
        return Ok(TableEnumVariant {
            ident: name,
            values,
        });
    }
}

impl Parse for TableEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        let visibility: Visibility = if lookahead.peek(Token![pub]) {
            input.parse()?
        } else {
            Visibility::Inherited
        };
        input.parse::<Token![enum]>()?;
        let name: Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let types = content.parse_terminated(syn::Field::parse_named, Token![,])?;
        let content;
        syn::braced!(content in input);
        let members = content.parse_terminated(TableEnumVariant::parse, Token![,])?;
        Ok(TableEnum {
            attrs,
            visibility,
            ident: name,
            types,
            members,
        })
    }
}

pub fn table_enum_core(input: TokenStream) -> TokenStream {
    let table_enum = match parse2::<TableEnum>(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };
    let enum_attrs = table_enum.attrs;
    let enum_visibility = table_enum.visibility;
    let enum_name = table_enum.ident;
    let mut enum_variants = Punctuated::<Variant, Comma>::new();
    for m in &table_enum.members {
        enum_variants.push(Variant {
            attrs: Vec::new(),
            ident: m.ident.clone(),
            fields: Fields::Unit,
            discriminant: None,
        });
    }
    if !enum_variants.trailing_punct() {
        enum_variants.push_punct(parse_quote!(,))
    }
    let variant_names = enum_variants
        .iter()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();
    let mut getters = Vec::<ImplItemFn>::new();
    for i in 0..table_enum.types.len() {
        let t = &table_enum.types[i];
        let getter_name = t.ident.clone().unwrap();
        let getter_type = &t.ty;
        let variant_values = table_enum.members.iter().map(|v| v.values[i].clone());
        let match_block: ExprMatch = parse_quote!(
            match self {
                #(#enum_name::#variant_names => #variant_values,)*
            }
        );
        let getter: ImplItemFn = parse_quote!(
            #enum_visibility const fn #getter_name(&self) -> #getter_type {
                #match_block
            }
        );
        getters.push(getter);
    }
    parse_quote!(
        #(#enum_attrs)*
        #enum_visibility enum #enum_name {
            #enum_variants
        }
        impl #enum_name {
            #(#getters)*
        }
    )
}
