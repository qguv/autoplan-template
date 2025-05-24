# Automated Planning

Rust project template (using `cargo`) for programming exercises from the class "Automated Planning" taught at the University of Amsterdam (2025 edition)

## How to build/test/run locally

    cargo build
    cargo test
    cargo run --bin 01-parsing < tests/samples/01-parsing/1.in

Note that it's not usually necessary to run the binary directly, because the sample files are integrated nicely into cargo's own test runner. See `tests/01-parsing.rs` for an example.

## How to submit to DOMjudge

Suppose you want to build the binary whose entry point is `src/bin/01-parsing.rs`; then you'd run:

    scripts/prepare.sh 01-parsing

This will fetch necessary dependencies (the `nom` parsing library), remove source for unused entry points, and create a bundle that the `cargo` platform script on DOMjudge knows how to build. The output will be a new `.tar.xz` file in the project root.

Then just log into DOMjudge and submit the resulting `.tar.xz` file as a source file of type `cargo`.

Tip: there's no need to commit the `.tar.xz` files. To keep track of your submissions, just create a git tag with the problem number, attempt number, and result:

    git tag -m p01-a1-wronganswer{,}
