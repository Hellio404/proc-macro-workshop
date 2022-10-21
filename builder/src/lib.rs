use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};
 
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let builder_name = format_ident!("{}Builder", name);
    let builder_fields = get_builder_fields(&input.data);
    let builder_methods = generate_builder_methods(&input.data);
    let builder_check_is_set = check_fields_are_set(&input.data);

    let assignement_to_builder = assign_builder_fields(&input.data);
    let build_command_fields = builder_build_command_fields(&input.data);

    let tokens = quote! {

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #assignement_to_builder
                }
            }
        }

        pub struct #builder_name {
            #builder_fields

        }

        impl #builder_name {
            #builder_methods
            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                #builder_check_is_set
                Ok(#name {
                    #build_command_fields
                })
            }
        }
    };

    proc_macro::TokenStream::from(tokens)
}


fn get_builder_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        #name: Option<#ty>
                    }
                });
                quote!(#(#recurse),*)
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn assign_builder_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name: None
                    }
                });
                quote!(#(#recurse),*)
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn generate_builder_methods(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        fn #name (&mut self, #name: #ty) -> &mut Self {
                            self.#name = Some(#name);
                            self
                        }

                    }
                });
                quote!(#(#recurse)*)
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn check_fields_are_set(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        if self.#name.is_none() {
                            return Err(Box::<dyn std::error::Error>::from(concat!(stringify!(#name), " was not set").to_string()));
                        }

                    }
                });
                quote!(#(#recurse)*)
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn builder_build_command_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name: self.#name.as_ref().unwrap().to_owned()
                    }
                });
                quote!(#(#recurse),*)
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
