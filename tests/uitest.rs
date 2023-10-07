use marker_uitest::ui_test::*;
use std::env;

fn main() -> color_eyre::Result<()> {
    // FIXME: Use `marker_uitest::simple_ui_test_config!` after v0.3.0
    // cc: https://github.com/rust-marker/marker/issues/276
    let config: Config = marker_uitest::create_ui_test_config(
        std::path::PathBuf::from_iter(std::path::Path::new("tests/ui")),
        &std::path::PathBuf::from_iter(std::path::Path::new(
            &std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "./target".into()),
        )),
        env!("CARGO_PKG_NAME"),
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")),
        marker_api::MARKER_API_VERSION,
    )?;

    // To use external crates in ui tests, they need to be defined in a `Cargo.toml`
    // of a valid crate. The line below will use the `Cargo.toml` if this lint crate.
    // config.dependencies_crate_manifest_path = Some("./Cargo.toml".into());

    run_tests_generic(
        vec![config],
        default_file_filter,
        default_per_file_config,
        status_emitter::Text::verbose(),
    )
}
