//! This is the parent module for custom lints

use marker_api::prelude::*;

mod combinable_imports;

pub fn lints() -> Vec<&'static Lint> {
    vec![combinable_imports::COMBINABLE_IMPORTS]
}

pub fn check_item<'ast>(cx: &'ast MarkerContext<'ast>, item: ast::ItemKind<'ast>) {
    combinable_imports::check_item(cx, item);
}

/// Currently unused until the crate object is available after:
/// <https://github.com/rust-marker/marker/issues/279>
#[allow(dead_code)]
pub fn check_crate<'ast>(cx: &'ast MarkerContext<'ast>, krate: ast::Crate<'ast>) {
    combinable_imports::check_crate(cx, krate);
}
