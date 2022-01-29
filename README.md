![tests](https://github.com/yds12/mexe/actions/workflows/unit.yml/badge.svg)

**m**athematical **ex**pression **e**valuator.

## How to Use

    use mexe::eval;

    fn main() {
        let forty_six = eval("(5 * 8) + 6").unwrap();
        let two = eval("1 + 1").unwrap();
        println!("{} & {}", forty_six, two);
    }

## Why?

If you need to evaluate simple arithmetic expressions, this crate offers a fast
and lightweight solution.

It's about 4-10x faster than `meval` and about 2x faster than `fasteval`, but
there are still optimisations to come, which will make it even faster. Note that
those crates do much more than `mexe`. Our focus on a very small problem makes
it easier for us to ship a fast and lean library.

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
- Having fun

## Similar Projects

- [pmalmgren/rust-calculator](https://github.com/pmalmgren/rust-calculator)
- [rekka/meval-rs](https://github.com/rekka/meval-rs)
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

