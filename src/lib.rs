use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path, Type, TypePath};

#[proc_macro_derive(Parser, attributes(arg))]
pub fn parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let help = generate_help(&input);
    let ident = input.ident;

    let mut matched_fields = Vec::new();
    let mut fields_constructor = Vec::new();
    let mut fields_checker = Vec::new();
    let mut init = Vec::new();

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            for field in fields_named.named {
                let name = field.ident.unwrap();
                let matcher = name.to_string();
                matched_fields.push(quote! {
                    Long(#matcher) => {#name = Some(parser.value()?.parse()?)}
                });

                fields_constructor.push(quote! {
                    let mut #name = None;
                });

                if !type_is_option(field.ty) {
                    fields_checker.push(quote! {
                        let #name = #name.ok_or(::lexopt::Error::MissingValue {
                            option: Some(#matcher.to_string()),
                        })?;
                    });
                }

                init.push(quote! {
                    #name,
                })
            }
        } else {
            unimplemented!() // TODO: Handle this as an error
        }
    } else {
        unimplemented!() // TODO: Handle this as an error
    }

    quote! {
        impl #ident {
            const HELP: &str = #help;

            pub fn parse<A>(args: A, help: &str) -> Result<Self, ::lexopt::Error>
            where
                A: IntoIterator,
                A::Item: Into<::std::ffi::OsString>,
            {
                #(#fields_constructor)*

                use ::lexopt::prelude::*;
                let mut parser = ::lexopt::Parser::from_iter(args);

                while let Some(arg) = parser.next()? {
                    match arg {
                        Long("help") | Short('h') => {
                            // println!("{}", Self::HELP);
                            println!("{help}"); // FIXME: This should use a generate help message
                            ::std::process::exit(0)
                        }
                        #(#matched_fields)*
                        _ => return Err(arg.unexpected()),
                    }
                }

                #(#fields_checker)*

                Ok(Self { #(#init)* })
            }
        }
    }
    .into()
}

fn generate_help(input: &DeriveInput) -> String {
    "This is a sample generated help message".to_string() // TODO: Need to actually generate an help message
}

fn type_is_option(path: Type) -> bool {
    if let Type::Path(TypePath { qself: _, path }) = path {
        path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Option"
    } else {
        false
    }
}
