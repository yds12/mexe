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
it's about 4-10x faster than `meval` and about 2x
faster than `fasteval`, but there are still optimisations to come, which will
make it even faster. Note that those crates do much more than `mexe`. Our focus
on a very small problem makes it easier for us to ship a fast and lean library.

## Includes

- sum
- subtraction
- multiplication
- division
- integers
- floats
- parentheses
- arbitrary whitespace

## Goals

- Minimal
- Fast: O(n)
- No allocations if possible
- We can assume the input is ASCII, and throw an error otherwise
- Thoroughly tested
- Maybe try to make it no-std

## Run Tests and Benchmarks

Unit tests:

    cargo test

Benchmarks:

    cargo bench -- bench_cmp   # comparison with other crates
    cargo bench -- bench_mexe  # only mexe

Fuzz tests have been ran with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

## Similar Projects

- [meval](https://crates.io/crates/meval)
- [fasteval](https://crates.io/crates/fasteval)
- [pmalmgren/rust-calculator](https://github.com/pmalmgren/rust-calculator)
- [adriaN/simple_rust_parser](https://github.com/adrianN/simple_rust_parser)

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

Our first (non-optimised) implementation will use an LL(1) parser.

## Links

* Documentation: [docs.rs](https://docs.rs/mexe/latest)
* Crate: [crates.io](https://crates.io/crates/mexe)
* Repository: [Github](https://github.com/yds12/mexe)

