# HYPER-OP.RS

The purpose is to compute [Hyper-Operation functions](https://en.wikipedia.org/wiki/Hyperoperation) with **truly** arbitrary-precision. It's intended to use as much memory as necessary to do the calculation. It's future-proof, so if 128bit-address-bus CPUs ever become a thing, it will take advantage of the extra memory to "unlock" more calculations.

## Usage

Install:

```sh
cargo install --git https://github.com/Rudxain/HYPER-OP.RS.git
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
