![loc](https://sloc.xyz/github/yds12/mexe)
![tests](https://github.com/yds12/mexe/actions/workflows/unit.yml/badge.svg)

**m**athematical **ex**pression **e**valuator.

## How to Use

    use mexe::eval;

    fn main() {
        let forty_six = eval("(5 * 8) + 6").unwrap();
        let two = eval("1 + 1").unwrap();
        println!("{} & {}", forty_six, two);

        assert_eq!(forty_six, 46.0);
        assert_eq!(two, 2.0);
    }

Note: the above `assert_eq`s work, but for float comparison in general use a
crate such as `float-cmp`.

## Why?

If you need to evaluate simple arithmetic expressions, this crate offers a fast
and lightweight solution.

In our [current benchmarks](https://github.com/yds12/mexe/actions/workflows/bench.yml),
it's about 4-10x faster than `meval`, about 2x faster than `fasteval`, and
the fully-featured `evalexpr` is generally the slowest. Note that those crates
do much more than `mexe` -- especially `evalexpr`. Our focus on a very small
problem makes it easier for us to ship a fast and lean library.

## Includes

- sum
- subtraction
- multiplication
- division
- integers
- floats
- parentheses
- arbitrary whitespace

Floats are represented as `X.Y` where `X` and `Y` are non-empty sequences of
digits. The notation with exponents for floats or omitting either side of the
point is not accepted.

## Goals

- Minimal
- Fast: O(n)
- No dependencies
- Minimal allocations
- Thoroughly tested

## Run Tests and Benchmarks

Unit tests and integration tests:

    cargo test

We leverage the [`glc` crate](https://crates.io/crates/glc) to generate valid
random inputs for `mexe`. The command below will run an ignored integration test
that runs indefinitely and shows the output in the terminal until you stop it
with `CTRL+C`:

    cargo test --test integration without_bounds -- --nocapture --ignored

Benchmarks:

    cargo bench -- bench_cmp   # comparison with other crates
    cargo bench -- bench_mexe  # only mexe

### Running the fuzzer

Fuzz tests have been ran with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

To run it yourself, you need to install the nightly toolchain
(`rustup toolchain install nightly`) and the tool itself:
`cargo install cargo-fuzz` (check for more detailed instructions and
dependencies in the project's readme).

After that run:

    cargo fuzz init
    cargo fuzz add fn_eval

Go to `fuzz/fuzz_targets/fn_eval.rs` and paste this code:

    #![no_main]
    use libfuzzer_sys::fuzz_target;

    fuzz_target!(|data: &[u8]| {
        // fuzzed code goes here
        if let Ok(text) = std::str::from_utf8(data) {
            let _ = mexe::eval(text);
        }
    });

Now finally run:

    cargo +nightly fuzz run fn_eval

## Grammar

    E  -> T E'
    E' -> + T E'
    E' -> - T E'
    E' -> ε
    T  -> F T'
    T' -> * F T'
    T' -> / F T'
    T' -> ε
    F  -> ( E )
    F  -> n
    F  -> - ( E )
    F  -> - n

where `ε` is the empty string and `n` is a terminal number token. Grammar idea
adapted from [this post](https://stackoverflow.com/a/23845375).

Our first implementation uses an LL(1) parser.

## Similar Projects

- [evalexpr](https://crates.io/crates/evalexpr)
- [meval](https://crates.io/crates/meval)
- [fasteval](https://crates.io/crates/fasteval)
- [pmalmgren/rust-calculator](https://github.com/pmalmgren/rust-calculator)
- [adriaN/simple_rust_parser](https://github.com/adrianN/simple_rust_parser)

## Links

* Documentation: [docs.rs](https://docs.rs/mexe/latest)
* Crate: [crates.io](https://crates.io/crates/mexe) and [lib.rs](https://lib.rs/crates/mexe)
* Repository: [Github](https://github.com/yds12/mexe)
