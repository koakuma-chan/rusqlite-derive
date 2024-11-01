use proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(FromSql)]
pub fn from_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let inner_type = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                    &fields.unnamed.first().unwrap().ty
                }

                _ => {
                    return syn::Error::new_spanned(
                        //
                        struct_name,
                        //
                        "FromSql can only be derived for tuple structs",
                    )
                    //
                    .to_compile_error()
                    //
                    .into();
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(
                //
                struct_name,
                //
                "FromSql can only be derived for tuple structs",
            )
            //
            .to_compile_error()
            //
            .into();
        }
    };

    quote! {
        impl ::rusqlite::types::FromSql for #struct_name {
            fn column_result(value: ::rusqlite::types::ValueRef<'_>) -> ::rusqlite::types::FromSqlResult<Self> {
                <#inner_type as ::rusqlite::types::FromSql>::column_result(value).map(Self)
            }
        }
    }
    //
    .into()
}

#[proc_macro_derive(ToSql)]
pub fn to_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => (),

            _ => {
                return syn::Error::new_spanned(
                    //
                    struct_name,
                    //
                    "ToSql can only be derived for tuple structs",
                )
                //
                .to_compile_error()
                //
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                //
                struct_name,
                //
                "ToSql can only be derived for tuple structs",
            )
            //
            .to_compile_error()
            //
            .into();
        }
    }

    quote! {
        impl ::rusqlite::types::ToSql for #struct_name {
            fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput<'_>> {
                self.0.to_sql()
            }
        }
    }
    //
    .into()
}
