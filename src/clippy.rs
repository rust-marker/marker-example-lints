//! This is the parent module for lints, which are inspired by Clippy lints.
//! See [Clippy's lint list] for a full list.
//!
//! It can happen that some lints from Clippy are simply not reproducible as Clippy
//! has access to more compiler internals than Marker might ever expose via its public API.
//!
//! [Clippy's lint list]: <https://rust-lang.github.io/rust-clippy/master/index.html>

use marker_api::prelude::*;

mod almost_complete_range;

pub fn lints() -> Vec<&'static Lint> {
    vec![almost_complete_range::ALMOST_COMPLETE_RANGE]
}

pub(crate) fn check_expr<'ast>(cx: &'ast MarkerContext<'ast>, expr: ast::ExprKind<'ast>) {
    almost_complete_range::check(cx, expr);
}
