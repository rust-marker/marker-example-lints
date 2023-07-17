use marker_uitest::ui_test::*;
use std::env;

fn main() -> color_eyre::Result<()> {
    let mut config = marker_uitest::simple_ui_test_config!()?;

    // To use external crates in ui tests, they need to be defined in a `Cargo.toml`
    // of a valid crate. The line below will use the `Cargo.toml` if this lint crate.
    // config.dependencies_crate_manifest_path = Some("./Cargo.toml".into());

    let bless = env::var_os("RUST_BLESS").is_some() || env::args().any(|arg| arg == "--bless");
    if bless {
        config.output_conflict_handling = OutputConflictHandling::Bless
    }

    config.stderr_filter(r"\\", "/");
    config.stdout_filter(r"\\", "/");

    run_tests_generic(
        config,
        default_file_filter,
        default_per_file_config,
        status_emitter::Text,
    )
}
