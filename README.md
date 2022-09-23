# ackermann.rs

This crate's purpose is to compute the [Ackermann-Péter function](https://en.wikipedia.org/wiki/Ackermann_function) with **truly** arbitrary-precision. It's intended to use as much memory as necessary to do the calculation. It's future-proof, so if 128bit-address-bus CPUs ever become a thing, it will take advantage of the extra memory to "unlock" more calculations.

## Usage

Install:

```sh
cargo install --git https://github.com/Rudxain/ackermann.rs.git
```

Argument syntax:

```sh
ackmn [m | help | /?] n
```

Example:

```sh
ackmn help #prints the help text
ackmn 2 3 #prints 9, because `A(2, 3) = 9`
```

## Disclaimer

[One of the doc-comments is false](https://github.com/Rudxain/ackermann.rs/issues/2) (currently, but not in the future)
