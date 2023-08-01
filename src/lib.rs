use proc_macro::{TokenStream};
use quote::{quote, __private::TokenStream as TokenStream2};
use syn::{parse_macro_input, DeriveInput, Field, Data, Fields, Ident};

#[proc_macro_derive(Rexcel)]
pub fn derive_rexcel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    if let Data::Struct(r#struct) = input.data {
        let fields = r#struct.fields;
        if matches!(fields, Fields::Named(_)) {
            let print_field = TokenStream2::from_iter(
                fields.iter()
                    .map(|field: &Field| field.ident.as_ref().unwrap())
                    .map(|ident: &Ident| quote!(
                        fields.push_str(&format!("{}\n", stringify!(#ident)));
                    )),
            );
            quote!(
                impl #ident {
                    pub fn print_field() {
                        let mut fields = String::new();
                        #print_field
                        println!("{}", fields)
                    }
                }
            ).into()
        } else {
            quote!().into()
        }
    } else {
        quote!().into()
    }
}