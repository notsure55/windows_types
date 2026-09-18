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
                    let typ = &field.ty;

                    quote! {
                        fn #get_ident(&self) -> #typ;
                        fn #set_ident(&mut self, #ident: #typ);
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
        //tokens.extend(generate_x64(&parsed));
    }

    tokens
}

fn generate_x64(parsed: &DeriveInput, trait_name: &Ident) -> TokenStream {
    let name = format_ident!("{}64", trait_name);
    let mut parsed = parsed.clone();

    if let Data::Struct(ref mut data) = parsed.data {
        let new_fields: Vec<_> = data
            .fields
            .clone()
            .into_iter()
            .filter_map(|field| {
                for attr in field.attrs.iter() {
                    if attr.path().get_ident().unwrap() == &format_ident!("x32") {
                        return None;
                    }
                }

                Some(field)
            })
            .collect();

        let methods: Vec<_> = new_fields
            .iter()
            .map(|field| {
                if let Some(ref ident) = field.ident {
                    let get_ident = format_ident!("get_{}", ident);
                    let set_ident = format_ident!("set_{}", ident);
                    let typ = &field.ty;

                    quote! {
                        fn #get_ident(&self, #ident: #typ) -> #typ;
                        fn #set_ident(&mut self, #ident: #typ)
                    }
                } else {
                    todo!("Handle non named fields");
                }
            })
            .collect();

        parsed.attrs = Vec::new();

        let token_stream: TokenStream = quote! {
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
            .map(|field| {
                field.attrs = clear_attributes(field.attrs.clone(), &format_ident!("x32"));

                if let Some(ref ident) = field.ident {
                    let get_ident = format_ident!("get_{}", ident);
                    let set_ident = format_ident!("set_{}", ident);
                    let ty = field.ty.clone();

                    let tokens = if let Type::Path(ref mut typ) = field.ty {
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
                    } else if let Type::Array(ref mut typ) = field.ty {
                        quote! {
                            fn #get_ident(&self) -> #ty {
                                self.#ident as _
                            }
                            fn #set_ident(&mut self, #ident: #ty) {
                                self.#ident = #ident;
                            }
                        }
                    } else {
                        todo!("implement checks for other types windows_type")
                    };

                    tokens
                } else {
                    todo!("Handle non named fields");
                }
            })
            .collect();

        let fields = data.fields.iter();

        let token_stream: TokenStream = quote! {
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

fn clear_attributes(attrs: impl IntoIterator<Item = Attribute>, ident: &Ident) -> Vec<Attribute> {
    attrs
        .into_iter()
        .filter(|attr| attr.path().get_ident().unwrap() != ident)
        .collect()
}
