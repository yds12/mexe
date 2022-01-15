Mathematical Expression Evaluator.

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
- Having fun

## Similar project

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

