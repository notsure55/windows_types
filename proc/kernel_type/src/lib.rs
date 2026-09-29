//#![no_std]
#![allow(dead_code)]
use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::*;

extern crate alloc;

use crate::alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;

fn from_snake_to_pascal_case(s: impl AsRef<str>) -> String {
    let mut capitalize = true;

    let mut string = String::new();

    for c in s.as_ref().chars() {
        if c == '_' {
            capitalize = true;
            continue;
        }

        let c: char = if capitalize {
            capitalize = false;
            c.to_uppercase().next().unwrap()
        } else {
            c.to_lowercase().next().unwrap()
        };

        string.push(c);
    }

    string
}

#[proc_macro_derive(KernelType)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);

    let struct_name = parsed.ident.clone();

    generate_wrapper(struct_name)
}

fn generate_wrapper(name: Ident) -> TokenStream {
    let pascal_name_string = from_snake_to_pascal_case(name.to_string());
    let pascal_name = format_ident!("{}", pascal_name_string);
    eprintln!("pascal = {:?} windows = {:?}", pascal_name, name);

    quote! {
        pub struct #pascal_name {
            pub raw: *mut #name,
        }

        impl core::ops::Deref for #pascal_name {
            type Target = #name;

            fn deref(&self) -> &Self::Target {
                if let Some(raw) = unsafe { self.raw.as_ref() } {
                    raw
                } else {
                    panic!("Pointer was null when trying to auto deref {:?}", #pascal_name_string);
                }
            }
        }

        impl core::ops::DerefMut for #pascal_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                if let Some(raw) = unsafe { self.raw.as_mut() } {
                    raw
                } else {
                    panic!("Pointer was null when trying to auto derefmut {:?}", #pascal_name_string);
                }
            }
        }
    }
    .into()
}

fn clear_attributes(
    attrs: impl IntoIterator<Item = Attribute>,
    idents: &[Ident],
) -> Vec<Attribute> {
    attrs
        .into_iter()
        .filter(|attr| idents.contains(attr.path().get_ident().unwrap()) != true)
        .collect()
}

fn find_attribute(attrs: &[Attribute], ident: &Ident) -> bool {
    for attr in attrs.iter() {
        if attr.path().get_ident().unwrap() == ident {
            return true;
        }
    }

    false
}

fn replace_field_type(field: &mut Field, new_type: &Ident) {
    if let Type::Path(ref mut typ) = field.ty {
        typ.path.segments[0].ident = new_type.clone()
    } else {
        todo!("Implement more types for replacing type for field")
    }
}

fn append_field_type(field: &mut Field, addon: &str) {
    if let Type::Path(ref mut typ) = field.ty {
        typ.path.segments[0].ident = format_ident!("{}{}", typ.path.segments[0].ident, addon);
    } else {
        todo!("Implement more types for replacing type for field")
    }
}
