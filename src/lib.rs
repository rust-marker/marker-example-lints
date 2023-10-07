#![feature(let_chains)]
#![warn(clippy::pedantic)]

use marker_api::prelude::*;
use marker_api::{LintPass, LintPassInfo, LintPassInfoBuilder};

mod clippy;

#[derive(Default)]
struct MyLintPass {}
marker_api::export_lint_pass!(MyLintPass);

impl LintPass for MyLintPass {
    fn info(&self) -> LintPassInfo {
        LintPassInfoBuilder::new(clippy::lints().into_boxed_slice()).build()
    }

    fn check_expr<'ast>(&mut self, cx: &'ast MarkerContext<'ast>, expr: ast::ExprKind<'ast>) {
        clippy::check_expr(cx, expr);
    }
}
