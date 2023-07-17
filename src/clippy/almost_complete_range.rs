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
            diag.span_suggestion("try", expr.span(), format!("{a}..={b}"), app);
        });
    }
}

fn is_almost_complete(start: ExprKind<'_>, end: ExprKind<'_>) -> bool {
    if let (Ok(start), Ok(end)) = (
        TryInto::<LitExprKind>::try_into(start),
        TryInto::<LitExprKind>::try_into(end),
    ) {
        match (start, end) {
            (LitExprKind::Int(start), LitExprKind::Int(end)) => {
                matches!(
                    (u8::try_from(start.value()), u8::try_from(end.value())),
                    (Ok(b'a'), Ok(b'z')) | (Ok(b'A'), Ok(b'Z')) | (Ok(b'0'), Ok(b'9'))
                )
            }
            (LitExprKind::Char(start), LitExprKind::Char(end)) => {
                matches!(
                    (start.value(), end.value()),
                    ('a', 'z') | ('A', 'Z') | ('0', '9')
                )
            }
            _ => false,
        }
    } else {
        false
    }
}
