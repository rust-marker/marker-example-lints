# Marker's Lint-Crate Template

This template contains everything to get you started with your first lint-crate for [Marker]. It contains all the required boilerplate and a working test setup.  

> **Note**
>
> [Marker] is in the early stages of development, some things are still missing and the API is still unstable. Documentation is still missing from major parts of the API.
>
> A list of limitations and planned features can be found in [Marker's Readme].

[Marker]: https://github.com/rust-marker/marker
[Marker's Readme]: https://github.com/rust-marker/marker/blob/master/README.md

## Getting Started

### Prerequisites

This template requires [Cargo], [rustup] and [cargo_marker] to be installed. Marker works on Linux, Windows, and macOS.

[Cargo]: https://github.com/rust-lang/cargo/
[rustup]: https://github.com/rust-lang/rustup/
[cargo_marker]: https://crates.io/crates/cargo_marker

### Development

1. Create a copy of this repository.
2. Update the name in the `Cargo.toml` file
3. Verify that everything is set up correctly by running `cargo test`
    * [rustup] might fetch a tool chain, required for the development of lint crates.
    * If the test fails, check the console output. It should include more information and tips on how to solve the problem.
4. The entry point of every lint is usually the [`LintPass`] implementation, located in the `./src/lib.rs` file. You can remove the example lint and implement different `check_*` functions depending on what you want to search for.
5. Now, it's just a lot of nested ifs and match statements.
    * You can just use print debugging to see the internal structure of nodes.

[`LintPass`]: https://docs.rs/marker_api/0.1.1/marker_api/trait.LintPass.html

### Testing

This template contains a working UI test setup. You can use `cargo uitest` to run the lint crate on the files inside the `.tests/ui/` directory. The corresponding `.stderr` and `.stdout` files can be updated automatically by running `cargo bless`. For more information on how these tests work and how to customize the setup, check out [marker_uitest]. 

## Using the Lint Crate

Once the lint crate is done, you can distribute it via git or a crate registry. To run the lint on a project, simply add it to the `[workspace.metadata.marker.lints]` section in the `Cargo.toml` file, like this:

```toml
[workspace.metadata.marker.lints]
# A lint local crate as a path
marker_lints = { path = './marker_lints' }
# An external lint crate via git
marker_lints = { git = "https://github.com/rust-marker/marker" }
# An external lint crate from a registry
marker_lints = "0.1.1"
```

You can take a look at [cargo_marker] for more options.

## Continuous Integration

This repository contains a workflow for GitHub. For every push, it will run all tests and check the code with [rustfmt] and [Clippy]. The workflow can be customized by editing the `./.github/rust.yml` file.

[Clippy]: https://github.com/rust-lang/rust-clippy
[rustfmt]: https://github.com/rust-lang/rustfmt

## Contributing

Contributions are highly appreciated! If you encounter any issues or have suggestions for improvements, please don't hesitate to open an issue or submit a pull request on [this template repository](https://github.com/rust-marker/lint-crate-template).

## License

Copyright (c) 2022-2023 Rust-Marker

Rust-marker is distributed under the terms of the MIT license or the Apache License (Version 2.0).

See [LICENSE-APACHE](https://github.com/rust-marker/marker/blob/master/LICENSE-APACHE), [LICENSE-MIT](https://github.com/rust-marker/marker/blob/master/LICENSE-MIT).
