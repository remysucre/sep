# Simple Equational Prover

This program allows you to check if an equality follows from a set of axioms.
It's a simple instantiation of [egg](https://github.com/egraphs-good/egg).
To use this program: 

1. Install rust with [rustup](https://rustup.rs/).
2. At the root of this project, run `cargo test --release` to see the tests pass.
3. Edit `src/lib.rs` to define the language (`define_language!`) and the axioms.
4. To check if an equality follows from the axioms,
follow the examples in `tests/test.rs` and add a test case.
5. Run `cargo test --release` again to check if the equality holds.

Note that expressions are in prefix (lisp) notation; 
pattern variables in a rewrite rule is prefixed with `?`.
