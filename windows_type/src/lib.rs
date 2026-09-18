use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::*;

#[derive(Debug, PartialEq, Eq)]
enum Arch {
    X32,
    X64,
}

impl Arch {
    pub fn new(arg: String) -> Option<Self> {
        if arg.contains("x32") {
            return Some(Self::X64);
        }
        if arg.contains("x64") {
            return Some(Self::X32);
        }

        None
    }
}

fn get_args(attr: TokenStream) -> Vec<Arch> {
    attr.into_iter()
        .filter_map(|attr| {
            if let TokenTree::Ident(ident) = attr {
                Arch::new(ident.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn generate_trait(parsed: &DeriveInput) -> TokenStream {
    let trait_name = &parsed.ident;

    if let Data::Struct(ref data) = parsed.data {
        let methods: Vec<_> = data
            .fields
            .iter()
            .map(|field| {
                if let Some(ref ident) = field.ident {
                    let get_ident = format_ident!("get_{}", ident);
                    let set_ident = format_ident!("set_{}", ident);
                    let ty = &field.ty;

                    let is_x32_only = find_attribute(&field.attrs, &format_ident!("x32"));
                    let is_obj = find_attribute(&field.attrs, &format_ident!("obj"));

                    // if the field is x32 only then we will make its getter optional return value
                    if is_x32_only == true && is_obj == true {
                        quote! {
                            fn #get_ident(&self) -> Option<&dyn #ty>;
                            fn #set_ident(&mut self, #ident: u32);
                        }
                    } else if is_x32_only == true {
                        quote! {
                            fn #get_ident(&self) -> Option<#ty>;
                            fn #set_ident(&mut self, #ident: #ty);
                        }
                    } else if is_obj == true {
                        quote! {
                            fn #get_ident(&self) -> &dyn #ty;
                            fn #set_ident(&mut self, #ident: u32);
                        }
                    } else {
                        quote! {
                            fn #get_ident(&self) -> #ty;
                            fn #set_ident(&mut self, #ident: #ty);
                        }
                    }
                } else {
                    todo!("Handle non named fields");
                }
            })
            .collect();

        let token_stream: TokenStream = quote! {
            pub trait #trait_name: std::fmt::Debug {
                #(#methods)*
            }
        }
        .into();

        return token_stream;
    } else {
        unreachable!("Gave a non struct data type")
    }
}

#[proc_macro_attribute]
pub fn windows_type(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);

    let trait_name = parsed.ident.clone();
    let args = get_args(attr);

    let mut tokens = generate_trait(&parsed);

    if args.contains(&Arch::X32) {
        tokens.extend(generate_x32(&parsed, &trait_name));
    }

    if args.contains(&Arch::X64) {
        tokens.extend(generate_x64(&parsed, &trait_name));
    }

    tokens
}

fn generate_x64(parsed: &DeriveInput, trait_name: &Ident) -> TokenStream {
    let name = format_ident!("{}64", trait_name);
    let mut parsed = parsed.clone();

    if let Data::Struct(ref mut data) = parsed.data {
        let methods: Vec<_> = data
            .fields
            .iter()
            .map(|field| {
                if let Some(ref ident) = field.ident {
                    let get_ident = format_ident!("get_{}", ident);
                    let set_ident = format_ident!("set_{}", ident);
                    let ty = &field.ty;

                    let is_x32_only = find_attribute(&field.attrs, &format_ident!("x32"));
                    let is_obj = find_attribute(&field.attrs, &format_ident!("obj"));

                    if is_x32_only == true && is_obj == true {
                        quote! {
                            fn #get_ident(&self) -> Option<&dyn #ty> {
                                None
                            }
                            fn #set_ident(&mut self, #ident: u32) {
                                // silenetly does nothing
                                let _ = #ident;
                            }
                        }
                    } else if is_x32_only == true {
                        quote! {
                            fn #get_ident(&self) -> Option<#ty> {
                                None
                            }
                            fn #set_ident(&mut self, #ident: #ty) {
                                // silenetly does nothing
                                let _ = #ident;
                            }
                        }
                    } else if is_obj == true {
                        quote! {
                            fn #get_ident(&self) -> &dyn #ty {
                                &self.#ident
                            }
                            fn #set_ident(&mut self, #ident: u32) {
                                let _ = #ident;
                            }
                        }
                    } else {
                        quote! {
                            fn #get_ident(&self) -> #ty {
                                self.#ident
                            }
                            fn #set_ident(&mut self, #ident: #ty) {
                                self.#ident = #ident;
                            }
                        }
                    }
                } else {
                    todo!("Handle non named fields");
                }
            })
            .collect();

        let new_fields: Vec<_> = data
            .fields
            .clone()
            .into_iter()
            .filter_map(|mut field| {
                for attr in field.attrs.iter() {
                    if attr.path().get_ident().unwrap() == &format_ident!("x32") {
                        return None;
                    }
                }

                if find_attribute(&field.attrs, &format_ident!("obj")) {
                    append_field_type(&mut field, "64");
                    field.attrs = clear_attributes(field.attrs.clone(), &[format_ident!("obj")]);
                }

                Some(field)
            })
            .collect();

        let token_stream: TokenStream = quote! {
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct #name {
                #(#new_fields),*
            }
            impl #trait_name for #name {
                #(#methods)*
            }
        }
        .into();

        token_stream
    } else {
        unreachable!()
    }
}

fn generate_x32(parsed: &DeriveInput, trait_name: &Ident) -> TokenStream {
    let name = format_ident!("{}32", trait_name);
    let mut parsed = parsed.clone();

    if let Data::Struct(ref mut data) = parsed.data {
        let methods: Vec<_> = data
            .fields
            .iter_mut()
            .map(|mut field| {
                if let Some(ident) = field.ident.clone() {
                    let get_ident = format_ident!("get_{}", ident);
                    let set_ident = format_ident!("set_{}", ident);
                    let ty = field.ty.clone();

                    let is_x32_only = find_attribute(&field.attrs, &format_ident!("x32"));
                    let is_obj = find_attribute(&field.attrs, &format_ident!("obj"));

                    let tokens = if is_x32_only == true && is_obj == true {
                        append_field_type(&mut field, "32");

                        field.attrs = clear_attributes(
                            field.attrs.clone(),
                            &[format_ident!("obj"), format_ident!("x32")],
                        );

                        quote! {
                            fn #get_ident(&self) -> Option<&dyn #ty> {
                                Some(&self.#ident as _)
                            }
                            fn #set_ident(&mut self, #ident: u32) {
                                let _ = #ident;
                            }
                        }
                    } else if is_x32_only == true {
                        field.attrs =
                            clear_attributes(field.attrs.clone(), &[format_ident!("x32")]);

                        quote! {
                            fn #get_ident(&self) -> Option<#ty> {
                                Some(self.#ident)
                            }
                            fn #set_ident(&mut self, #ident: #ty) {
                                self.#ident = #ident;
                            }
                        }
                    } else if is_obj == true {
                        append_field_type(&mut field, "32");
                        field.attrs =
                            clear_attributes(field.attrs.clone(), &[format_ident!("obj")]);

                        quote! {
                            fn #get_ident(&self) -> &dyn #ty {
                                &self.#ident as _
                            }
                            fn #set_ident(&mut self, #ident: u32) {
                                let _ = #ident;
                            }
                        }
                    } else if let Type::Path(ref mut typ) = field.ty {
                        if typ.path.segments[0].ident == format_ident!("u64") {
                            let tokens = quote! {
                                fn #get_ident(&self) -> #ty {
                                    self.#ident as _
                                }
                                fn #set_ident(&mut self, #ident: #ty) {
                                    self.#ident = #ident as _;
                                }
                            };

                            typ.path.segments[0].ident = format_ident!("u32");

                            tokens
                        } else {
                            quote! {
                                fn #get_ident(&self) -> #ty {
                                    self.#ident as _
                                }
                                fn #set_ident(&mut self, #ident: #ty) {
                                    self.#ident = #ident;
                                }
                            }
                        }
                    } else {
                        // TODO: make a good way to update arrays and other types that are bracketed <>
                        quote! {
                            fn #get_ident(&self) -> #ty {
                                self.#ident as _
                            }
                            fn #set_ident(&mut self, #ident: #ty) {
                                self.#ident = #ident;
                            }
                        }
                    };

                    tokens
                } else {
                    todo!("Handle non named fields");
                }
            })
            .collect();

        let fields = data.fields.iter();

        let token_stream: TokenStream = quote! {
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct #name {
                #(#fields),*
            }

            impl #trait_name for #name {
                #(#methods)*
            }
        }
        .into();

        return token_stream;
    } else {
        unreachable!("BLAKC");
    }
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
