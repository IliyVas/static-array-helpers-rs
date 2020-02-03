#![feature(map_first_last, rustc_private)]

extern crate rustc;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

// Rust can't compile this code without this dependency on my machine
extern crate rustc_driver;

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_errors::registry::Registry;
use rustc_interface::{Config, run_compiler};
use rustc_session::config::Input;
use rustc_session::DiagnosticOutput;

use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;

macro_rules! macro_error {
    () => {
        return TokenStream::from(quote!(compile_error!("<ident> = <array> expected");))
    };
}

#[proc_macro]
pub fn static_array(ts: TokenStream) -> TokenStream {
    let mut ts_iter = ts.into_iter();

    // Get static array identifier
    let ident = match ts_iter.next() {
        Some(token) => token,
        None => macro_error!()
    };

    // Tiny check
    match ts_iter.next() {
        Some(equal) => {
            if equal.to_string().as_str() != "=" {
                macro_error!()
            }
        }
        None => macro_error!()
    };

    // Get array literal
    let mut array_ts = TokenStream::new();
    array_ts.extend(ts_iter);

    // Init internal rustc api
    let src = format!("fn main() {{ let arr = {} ; }}", array_ts);
    let filename = rustc_span::FileName::Custom("".to_string());
    let config = Config {
        opts: Default::default(),
        crate_cfg: FxHashSet::default(),
        input: Input::Str {
            name: filename,
            input: src
        },
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        diagnostic_output: DiagnosticOutput::Default,
        stderr: None,
        crate_name: None,
        lint_caps: FxHashMap::default(),
        register_lints: None,
        override_queries: None,
        registry: Registry::new(&[]),
    };

    let array_type_str = run_compiler(
        config,
        |compiler| {
            return compiler.enter(|queries| {
                let mut global_ctxt = queries.global_ctxt().unwrap().take();
                return global_ctxt.enter(|tcx| {
                    // Get main function body
                    let (_, body) = tcx.hir().krate().bodies.first_key_value().unwrap();

                    // Get "let" expression
                    let stmt_kind = match body.value.kind {
                        rustc_hir::ExprKind::Block(block, _) => &block.stmts[0].kind,
                        _ => unreachable!()
                    };
                    let let_expr = match stmt_kind {
                        rustc_hir::StmtKind::Local(l) => l.init.unwrap(),
                        _ => unreachable!()
                    };

                    let typeck_table = tcx.typeck_tables_of(let_expr.hir_id.owner_def_id());
                    let array_type = typeck_table.expr_ty(let_expr);
                    match array_type.kind {
                        rustc::ty::TyKind::Error => String::new(),
                        _ => array_type.to_string()
                    }
                });
            });
        }
    );

    if array_type_str.is_empty() {
        return TokenStream::from(quote!{
            compile_error!("Some not basic type is used in array or value after \"=\" isn't array");
        })
    }

    let array_def_str = format!("static {}: {} =", ident, array_type_str);
    let semicolon = TokenStream::from_str(";").unwrap();
    let mut array_def_ts = TokenStream::from_str(&array_def_str).unwrap();
    array_def_ts.extend(array_ts);
    array_def_ts.extend(semicolon);
    array_def_ts
}