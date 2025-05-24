# Automated Planning

Rust project template (using `cargo`) for programming exercises from the class "Automated Planning" taught at the University of Amsterdam (2025 edition)

## How to build/test/run locally

TL;DR: Run `cargo test`

Unit tests in Rust are typically defined close to the code they test; see `src/bfs.rs` for an example. These can be run using `cargo test`, or by running a particular test, e.g. `cargo test bfs::test_bfs::test_placeholder`.

Integration tests (e.g. running your code on the provided sample problems and checking against the expected output) are defined in their own files in the `tests/` directory and are also run with `cargo test`. You can also run a specific test file, e.g. `cargo test p01_parsing::test_1`. The `test_each_file!` macro (from the crate of the same name) is used to automatically discover test files in the `tests/samples/` directory, avoiding some boilerplate, though it's still necessary to write a test runner function for each exercise. You can follow the example provided in `tests/01-parsing.rs`.

Note that it's not usually necessary to run the built binary directly, because of the test functionality described above. If needed, though, you can do so by running:

    cargo run --bin 01-parsing < tests/samples/01-parsing/1.in

or by building and executing manually:

    cargo build
    target/debug/01-parsing < tests/samples/01-parsing/1.in

Alternatively, to build in release mode:

    cargo build --release
    target/release/01-parsing < tests/samples/01-parsing/1.in

## How to submit to DOMjudge

Suppose you want to build the binary whose entry point is `src/bin/01-parsing.rs`; then you'd run:

    scripts/prepare.sh 01-parsing

This will fetch necessary dependencies (the `nom` parsing library), remove source for unused entry points, and create a bundle that the `cargo` platform script on DOMjudge knows how to build. The output will be a new `.tar.xz` file in the project root.

Then just log into DOMjudge and submit the resulting `.tar.xz` file as a source file of type `cargo`.

Tip: there's no need to commit the `.tar.xz` files. To keep track of your submissions, just create a git tag with the problem number, attempt number, and result:

    git tag -m p01-a1-wronganswer{,}
