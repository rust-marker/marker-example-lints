use marker_api::{
    ast::{AstPath, AstPathSegment},
    prelude::*,
    span::SpanSource,
};

marker_api::declare_lint! {
    /// # What it does
    /// It detects `use` items that could be combined.
    ///
    /// # Example
    /// ```
    /// use std::fs::File;
    /// use std::vec::Vec;
    /// ```
    ///
    /// Use instead:
    /// ```
    /// use std::{fs::File, vec::Vec};
    /// ```
    COMBINABLE_IMPORTS,
    Warn,
}

pub fn check_item<'ast>(cx: &'ast MarkerContext<'ast>, item: ast::ItemKind<'ast>) {
    if let ast::ItemKind::Mod(module) = item
        && !item.span().is_from_expansion()
    {
        check_items(cx, module.items());
    }
}

pub fn check_crate<'ast>(cx: &'ast MarkerContext<'ast>, krate: ast::Crate<'ast>) {
    check_items(cx, krate.items());
}

pub fn check_items<'ast>(cx: &'ast MarkerContext<'ast>, items: &[ast::ItemKind<'ast>]) {
    let mut namespaces: Vec<UseItemInfo<'_>> = vec![];

    for item in items {
        // Only select imports
        let import = if let ast::ItemKind::Use(import) = item
            && let span = import.span()
            && !span.is_from_expansion()
        {
            import
        } else {
            continue;
        };

        // Find merge candidate
        let segs = stringify_path(import.use_path());
        let import_info = UseItemInfo { segs, import };
        let candidate = namespaces
            .iter()
            .fold((None, 0), |prev_selection, candidate| {
                compare_candidate(&import_info, prev_selection, candidate)
            });

        // Emit lint
        if let (Some(other), _) = candidate {
            cx.emit_lint(COMBINABLE_IMPORTS, import, "this import can be merged...")
                .span_note("...with this one", other.import.span());
        }

        // Insert as a potential merge candidate
        namespaces.push(import_info);
    }
}

fn stringify_path(path: &AstPath<'_>) -> Vec<String> {
    path.segments()
        .iter()
        .map(|seg| seg.ident().name().to_string())
        .collect()
}

fn compare_candidate<'a, 'ast>(
    import: &UseItemInfo<'ast>,
    best: (Option<&'a UseItemInfo<'ast>>, usize),
    candidate: &'a UseItemInfo<'ast>,
) -> (Option<&'a UseItemInfo<'ast>>, usize) {
    let (best_candidate, best_count) = best;
    let ctn = count_match(&import.segs, &candidate.segs);
    if ctn > best_count
        && !are_merged(
            &import.import.use_path().segments()[ctn - 1],
            &candidate.import.use_path().segments()[ctn - 1],
        )
    {
        (Some(candidate), ctn)
    } else {
        (best_candidate, best_count)
    }
}

fn count_match(a: &[String], b: &[String]) -> usize {
    a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count()
}

/// Checks if the two [`AstPathSegment`]s are the same or different.
fn are_merged(a: &AstPathSegment<'_>, b: &AstPathSegment<'_>) -> bool {
    let a_span = a.ident().span();
    if let SpanSource::File(file) = a_span.source() {
        let a_start = file.to_file_pos(a_span.start());
        let b_start = file.to_file_pos(b.ident().span().start());
        a_start.column() == b_start.column() && a_start.line() == b_start.line()
    } else {
        false
    }
}

struct UseItemInfo<'ast> {
    segs: Vec<String>,
    import: &'ast ast::UseItem<'ast>,
}
