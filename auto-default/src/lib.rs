extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote};
use syn::{Expr, ExprStruct, Token, ExprPath, Path};

fn add_default_bases(expr: &mut Expr) {
    match expr {
        Expr::Struct(expr) => {
            expr.rest = match &expr.rest {
                None => Some(Box::new(parse_quote!(Default::default()))),
                Some(rest) => {
                    match &**rest {
                        Expr::Path(ExprPath{path, ..}) if path.is_ident("__") => None,
                        rest => Some(Box::new(rest.clone())),
                    }
                },
            };
            for field in expr.fields.iter_mut() {
                add_default_bases(&mut field.expr)
            }
        }
        Expr::Call(expr) => {
            for arg in expr.args.iter_mut() {
                add_default_bases(arg)
            }
        }
        _ => {}
    }
}

#[proc_macro]
pub fn auto_default(input: TokenStream) -> TokenStream {
    let mut expr = parse_macro_input!(input as Expr);
    add_default_bases(&mut expr);
    expr.into_token_stream().into()
}
