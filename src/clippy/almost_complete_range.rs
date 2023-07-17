use marker_api::{ast::expr::LitExprKind, diagnostic::Applicability, prelude::*};

marker_api::declare_lint! {
    /// # What it does
    /// Checks for ranges that almost check an entire ascii range. The linted
    /// ranges are:
    /// * `'a'..'z'`
    /// * `'A'..'Z'`
    /// * `'0'..'9'`
    ///
    /// These ranges should probably be inclusive.
    ///
    /// # Example
    /// ```
    /// # let x = 0;
    /// ('a'..'z').contains(&c);
    /// ```
    ///
    /// Use instead
    /// ```
    /// # let x = 0;
    /// ('a'..='z').contains(&c);
    /// ```
    ///
    /// # Note
    /// This lint was inspired by the [clippy::almost_complete_range] lint.
    ///
    /// [clippy::almost_complete_range]: <https://rust-lang.github.io/rust-clippy/master/index.html#/almost_complete_range>
    ALMOST_COMPLETE_RANGE,
    Warn,
}

pub(crate) fn check<'ast>(cx: &AstContext<'ast>, expr: ExprKind<'ast>) {
    if let ExprKind::Range(range) = expr
        && !range.is_inclusive()
        && let Some(start) = range.start()
        && let Some(end) = range.end()
        && is_almost_complete(start, end)
    {
        cx.emit_lint(ALMOST_COMPLETE_RANGE, expr.id(), "almost complete ascii range", expr.span(), |diag| {
            let mut app = Applicability::MachineApplicable;
            let a = start.span().snippet_with_applicability("<start>", &mut app);
            let b = start.span().snippet_with_applicability("<end>", &mut app);
            diag.span_suggestion("try", expr.span(), format!("{a}..={b}"), app)
        })
    }
}

fn is_almost_complete(start: ExprKind<'_>, end: ExprKind<'_>) -> bool {
    if let (Ok(start), Ok(end)) = (
        TryInto::<LitExprKind>::try_into(start),
        TryInto::<LitExprKind>::try_into(end),
    ) {
        match (start, end) {
            (LitExprKind::Int(start), LitExprKind::Int(end)) => {
                if start.value() < u8::MAX.into() || end.value() < u8::MAX.into() {
                    match (start.value() as u8, end.value() as u8) {
                        (b'a', b'z') => true,
                        (b'A', b'Z') => true,
                        (b'0', b'9') => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            (LitExprKind::Char(start), LitExprKind::Char(end)) => {
                match (start.value(), end.value()) {
                    ('a', 'z') => true,
                    ('A', 'Z') => true,
                    ('0', '9') => true,
                    _ => false,
                }
            }
            _ => false,
        }
    } else {
        false
    }
}
