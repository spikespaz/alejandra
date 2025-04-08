use std::path::PathBuf;

use heck::ToSnakeCase;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprLit, FieldValue, Lit, Member};

#[proc_macro]
pub fn alejandra_test_cases(input: TokenStream1) -> TokenStream1 {
    let cargo_manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let fields =
        Punctuated::<FieldValue, Comma>::parse_terminated.parse(input).unwrap();

    let (mut cases_dir, mut config) = (None, None);

    for FieldValue { member, expr, .. } in fields {
        let Member::Named(member) = member else { unimplemented!() };
        let member = member.to_string();

        match (member.as_bytes(), expr) {
            (b"cases_dir", Expr::Lit(ExprLit { lit: Lit::Str(lit), .. })) => {
                if cases_dir.is_none() {
                    cases_dir = Some(lit.value());
                } else {
                    unimplemented!()
                }
            }
            (b"config", expr) => {
                if config.is_none() {
                    config = Some(expr);
                } else {
                    unimplemented!()
                }
            }
            (_, _) => unimplemented!(),
        }
    }

    let cases_dir =
        cargo_manifest_dir.join(cases_dir.unwrap()).canonicalize().unwrap();
    let config = config.unwrap();

    let mut tokens = TokenStream::new();

    for case_dir in std::fs::read_dir(&cases_dir).unwrap() {
        let case_dir = case_dir.unwrap();
        let case_name =
            case_dir.file_name().into_string().unwrap().to_snake_case();
        let path_in = case_dir
            .path()
            .join("in.nix")
            .canonicalize()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let path_out = case_dir
            .path()
            .join("out.nix")
            .canonicalize()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        let case_ident = format_ident!("{case_name}");

        tokens.extend(quote! {
            #[test]
            fn #case_ident() {
                static BEFORE: &str = include_str!(#path_in);
                static AFTER: &str = include_str!(#path_out);

                let (_status, result) = ::alejandra::format::in_memory(
                    #path_in.to_owned(),
                    BEFORE.to_owned(),
                    #config,
                );

                assert_eq!(result, AFTER)
            }
        });
    }

    tokens.into()
}
