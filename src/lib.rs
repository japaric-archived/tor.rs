#![deny(warnings)]
#![feature(plugin_registrar, slicing_syntax)]

extern crate rustc;
extern crate syntax;

use rustc::plugin::registry::Registry;
use syntax::ast::{ExprBox, Inherited, TTTok, TokenTree};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacExpr, MacResult, NormalTT};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::{mod, LBRACKET, RBRACKET};

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(r: &mut Registry) {
    r.register_syntax_extension(token::intern("tor"), NormalTT(box expand_tor, None));
}

/// Expands `tor!($SOMETHING)` into:
///
/// ``` ignore
/// {
///     use std::slice::BoxedSlice;
///     use std::boxed::HEAP;
///     let xs = box (HEAP) [$SOMETHING];
///     xs.into_vec()
/// }
/// ```
fn expand_tor<'cx>(
    cx: &'cx mut ExtCtxt,
    sp: Span,
    tts: &[TokenTree],
) -> Box<MacResult + 'cx> {
    // The original `tts` doesn't include delimiters, here I'll manually add brackets
    let tts = {
        let mut new_tts = Vec::with_capacity(tts.len() + 2);
        // XXX The span `sp` is likely to be wrong!
        new_tts.push(TTTok(sp, LBRACKET));
        new_tts.push_all(tts);
        // XXX Same here!
        new_tts.push(TTTok(sp, RBRACKET));
        new_tts
    };

    // This syntax extension expands into a block, composed of:
    let block = {
        // - Two imports:
        let imports = vec![{
            // `use std::slice::BoxedSlice`
            let segments = vec![
                token::str_to_ident("std"),
                token::str_to_ident("slice"),
                token::str_to_ident("BoxedSlice"),
            ];

            cx.view_use_simple(sp, Inherited, cx.path_global(sp, segments))
        }, {
            // `use std::boxed::HEAP`
            let segments = vec![
                token::str_to_ident("std"),
                token::str_to_ident("boxed"),
                token::str_to_ident("HEAP"),
            ];

            cx.view_use_simple(sp, Inherited, cx.path_global(sp, segments))
        }];

        // - One let statement:
        let stmts = vec![{
            // `let xs = box (HEAP) $ARRAY`
            let immutable = false;
            let ident = token::str_to_ident("xs");
            let expr = {
                // (NB No idea how to elide the `HEAP` in the box expression)
                let heap = cx.expr_ident(sp, token::str_to_ident("HEAP"));
                // Where the array is parsed from the modified `tts`
                let array = cx.new_parser_from_tts(tts[]).parse_expr();

                cx.expr(sp, ExprBox(heap, array))
            };

            cx.stmt_let(sp, immutable, ident, expr)
        }];

        // - And the return expression:
        let expr = {
            // `xs.into_vec()`
            let receiver = cx.expr_ident(sp, token::str_to_ident("xs"));
            let method = token::str_to_ident("into_vec");
            let args = vec![];

            cx.expr_method_call(sp, receiver, method, args)
        };

        cx.expr_block(cx.block_all(sp, imports, stmts, Some(expr)))
    };

    MacExpr::new(block)
}
