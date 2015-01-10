#![feature(plugin_registrar)]

extern crate syntax;
extern crate rustc;

use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base;
use syntax::ext::base::{ExtCtxt, MacExpr};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::intern_and_get_ident;
use rustc::plugin::Registry;
use std::num::Float;
use std::str::FromStr;

macro_rules! expand_syntax_ext {
    ($method:ident -> $ret:ident) => {
        fn $method(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree]) -> Box<base::MacResult + 'static> {
            let p = &mut cx.new_parser_from_tts(tts);
            let expr = p.parse_expr();

            let (interned_x, ty_opt) = match expr.node {
                ast::ExprLit(ref lit) => match lit.node {
                    ast::LitFloat(ref s, ty) => {
                        (s.clone(), Some(ty))
                    },
                    ast::LitFloatUnsuffixed(ref s) =>  {
                        (s.clone(), None)
                    },
                    _ => {
                        cx.span_err(expr.span, "unsupported literal in sqrt!");
                        return base::DummyResult::expr(sp);
                    }
                },
                _ => {
                    cx.span_err(expr.span, "unsupported literal in sqrt!");
                    return base::DummyResult::expr(sp);
                }
            };

            let x: f32 = match FromStr::from_str(interned_x.get().as_slice()) {
                Some(o) => o,
                None => {
                    cx.span_err(expr.span, "unsupported literal in sqrt!");
                    return base::DummyResult::expr(sp);
                }
            };

            let sol: $ret = x.$method();
            let evs = intern_and_get_ident(sol.to_string().as_slice());
            let lit = match ty_opt {
                Some(ty) => ast::LitFloat(evs, ty),
                None => ast::LitFloatUnsuffixed(evs)
            };

            return MacExpr::new(cx.expr_lit(sp, lit));
        }
    }
}

expand_syntax_ext!(is_nan -> bool);
expand_syntax_ext!(is_infinite -> bool);
expand_syntax_ext!(is_finite -> bool);
expand_syntax_ext!(is_normal -> bool);
expand_syntax_ext!(floor -> f32);
expand_syntax_ext!(ceil -> f32);
expand_syntax_ext!(round -> f32);
expand_syntax_ext!(trunc -> f32);
expand_syntax_ext!(fract -> f32);
expand_syntax_ext!(abs -> f32);
expand_syntax_ext!(signum -> f32);
expand_syntax_ext!(is_positive -> bool);
expand_syntax_ext!(is_negative -> bool);
expand_syntax_ext!(recip -> f32);
expand_syntax_ext!(sqrt -> f32);
expand_syntax_ext!(rsqrt -> f32);
expand_syntax_ext!(exp -> f32);
expand_syntax_ext!(exp2 -> f32);
expand_syntax_ext!(ln -> f32);
expand_syntax_ext!(log2 -> f32);
expand_syntax_ext!(log10 -> f32);
expand_syntax_ext!(to_degrees -> f32);
expand_syntax_ext!(to_radians -> f32);
expand_syntax_ext!(cbrt -> f32);
expand_syntax_ext!(sin -> f32);
expand_syntax_ext!(cos -> f32);
expand_syntax_ext!(tan -> f32);
expand_syntax_ext!(asin -> f32);
expand_syntax_ext!(acos -> f32);
expand_syntax_ext!(atan -> f32);
expand_syntax_ext!(exp_m1 -> f32);
expand_syntax_ext!(ln_1p -> f32);
expand_syntax_ext!(sinh -> f32);
expand_syntax_ext!(cosh -> f32);
expand_syntax_ext!(tanh -> f32);
expand_syntax_ext!(asinh -> f32);
expand_syntax_ext!(acosh -> f32);
expand_syntax_ext!(atanh -> f32);

// TODO: do more

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("is_nan", is_nan);  
    reg.register_macro("is_infinite", is_infinite);  
    reg.register_macro("is_finite", is_finite);  
    reg.register_macro("is_normal", is_normal);  
    reg.register_macro("floor", floor);  
    reg.register_macro("ceil", ceil);  
    reg.register_macro("round", round);  
    reg.register_macro("trunc", trunc);  
    reg.register_macro("fract", fract);  
    reg.register_macro("abs", abs);  
    reg.register_macro("signum", signum);  
    reg.register_macro("is_positive", is_positive);  
    reg.register_macro("is_negative", is_negative);  
    reg.register_macro("recip", recip);  
    reg.register_macro("sqrt", sqrt);  
    reg.register_macro("rsqrt", rsqrt);  
    reg.register_macro("exp", exp);  
    reg.register_macro("exp2", exp2);  
    reg.register_macro("ln", ln);  
    reg.register_macro("log2", log2);  
    reg.register_macro("log10", log10);  
    reg.register_macro("to_degrees", to_degrees);  
    reg.register_macro("to_radians", to_radians);  
    reg.register_macro("cbrt", cbrt);  
    reg.register_macro("sin", sin);  
    reg.register_macro("cos", cos);  
    reg.register_macro("tan", tan);  
    reg.register_macro("asin", asin);  
    reg.register_macro("acos", acos);  
    reg.register_macro("atan", atan);  
    reg.register_macro("exp_m1", exp_m1);  
    reg.register_macro("ln_1p", ln_1p);  
    reg.register_macro("sinh", sinh);  
    reg.register_macro("cosh", cosh);  
    reg.register_macro("tanh", tanh);  
    reg.register_macro("asinh", asinh);  
    reg.register_macro("acosh", acosh);  
    reg.register_macro("atanh", atanh);  
}


// FIXME (10872): This is required to prevent an LLVM assert on Windows
// https://github.com/rust-lang/hexfloat/blob/f67d5fb9cf76986d122b6ede97319cee4071fb69/src/lib.rs#L175
#[test]
fn dummy_test() { }
