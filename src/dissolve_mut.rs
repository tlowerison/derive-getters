//! DissolveMut internals
use std::{convert::TryFrom, iter::Extend};

use proc_macro2::{Delimiter, Group, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    AttrStyle, Attribute, DeriveInput, Error, Ident, LitStr, Result, Type, TypeTuple,
};

use crate::{
    dissolve::{Field, IndexOrName},
    extract::named_struct,
    faultmsg::Problem,
};

struct Rename {
    name: Ident,
}

impl Parse for Rename {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(rename);

        if input.peek(rename) {
            let _ = input.parse::<rename>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowNewName))
            } else {
                let name = Ident::new(name.value().as_str(), Span::call_site());
                Ok(Rename { name })
            }
        } else {
            Err(Error::new(Span::call_site(), Problem::InvalidAttribute))
        }
    }
}

fn dissolve_mut_rename_from(attributes: &[Attribute]) -> Result<Option<Ident>> {
    let mut current: Option<Ident> = None;

    for attr in attributes {
        if attr.style != AttrStyle::Outer {
            continue;
        }

        if attr.path().is_ident("dissolve_mut") {
            let rename = attr.parse_args::<Rename>()?;
            current = Some(rename.name);
        }
    }

    Ok(current)
}

pub struct NamedStruct<'a> {
    original: &'a DeriveInput,
    name: Ident,
    fields: Vec<Field>,
    dissolve_mut_rename: Option<Ident>,
}

impl<'a> NamedStruct<'a> {
    pub fn emit(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.name;

        let types: Punctuated<Type, syn::Token![,]> =
            self.fields.iter().fold(Punctuated::new(), |mut p, field| {
                p.push(syn::Type::Reference(syn::TypeReference {
                    and_token: Default::default(),
                    lifetime: None,
                    mutability: Some(Default::default()),
                    elem: Box::new(field.ty.clone()),
                }));
                p
            });

        let types_len = types.len();

        let return_type = if types_len > 1 {
            let tup_group = Group::new(Delimiter::Parenthesis, quote!(#types));
            let type_tuple = TypeTuple {
                paren_token: Paren {
                    span: tup_group.delim_span(),
                },
                elems: types,
            };

            quote!(#type_tuple)
        } else {
            if let Some(elem) = types.first() {
                quote!(#elem)
            } else {
                quote!(())
            }
        };

        let fields: TokenStream =
            self.fields
                .iter()
                .enumerate()
                .fold(TokenStream::new(), |mut ts, (count, field)| {
                    if count > 0 {
                        ts.extend(quote!(,));
                    }

                    let field_name = &field.name;
                    let field_expr = match field_name {
                        IndexOrName::Name(name) => {
                            quote!(
                                &mut self.#name
                            )
                        }
                        IndexOrName::Index(i) => {
                            quote!(
                                &mut self.#i
                            )
                        }
                    };

                    ts.extend(field_expr);

                    ts
                });

        let dissolve_mut = Ident::new("dissolve_mut", Span::call_site());
        let fn_name = self.dissolve_mut_rename.as_ref().unwrap_or(&dissolve_mut);

        quote!(
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                pub fn #fn_name(&mut self) -> #return_type {
                    (
                        #fields
                    )
                }
            }
        )
    }
}

impl<'a> TryFrom<&'a DeriveInput> for NamedStruct<'a> {
    type Error = Error;

    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = named_struct(node)?;
        let fields = Field::from_struct(struct_data)?;
        let rename = dissolve_mut_rename_from(node.attrs.as_slice())?;

        Ok(NamedStruct {
            original: node,
            name: node.ident.clone(),
            fields,
            dissolve_mut_rename: rename,
        })
    }
}
