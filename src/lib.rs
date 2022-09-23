#![warn(
	unused,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string,
	//clippy::arithmetic_side_effects
)]
#![deny(clippy::unwrap_used)]
#![forbid(
	unsafe_code,
	clippy::exit,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

//!Provives fns to calculate the Ackermann Function with arbitrary precision
//!
//!More precisely, the Ackermann-Péter fn.
//!
//!Input args of these fns can be stack-allocated (fixed-size ints) or heap-allocated (`num_bigint`)

use num_bigint::BigUint;
use num_traits::{One, Zero};

///Calculates `base` ^ `exp`.
///
///It uses [binary exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) algorithm
///
///This helper is necessary because the `pow` trait only supports `u32` as `exp`,
///but we need **truly arbitrary** precision, for mathematical correctness.
fn pow(base: &BigUint, exp: &BigUint) -> BigUint {
	let b = base;
	let e = exp;

	//this patches issue #1 (not a fix)
	if *e <= BigUint::from(core::u32::MAX) {
		return b.pow(e.to_u32_digits()[0]);
	}

	if b.is_zero() {
		return b.clone(); //should this be `BigUint::zero()`?
	}

	let out = BigUint::one();

	if b.is_one() {
		return out;
	}

	let mut out = out;

	let mut b = b.clone();
	let mut e = e.clone();
	//do-while
	while {
		if e.bit(0) {
			out *= &b;
		}
		e >>= 1;
		b = &b * &b;

		!e.is_one() //condition
	} {}
	out
}

///Calculates the [Hyper-Operation function](https://en.wikipedia.org/wiki/Hyperoperation#Definition)
///
///`n` is `n` ("order"), `base` is `a`, `exp` is `b`
///
///This helper is necessary because it's **WAY BETTER** than the Ackermann fn.
///It's faster, uses less memory, and it's more readable, than the optimized ack fn with explicit stack.
///Also, it doesn't need memoization!
fn hyper_op(n: &BigUint, base: &BigUint, exp: &BigUint) -> BigUint {
	let n0 = BigUint::zero();
	let n1 = BigUint::one();

	if n == &n0 {
		return exp + n1;
	}
	if n == &n1 {
		return base + exp;
	}
	let n2 = &n1 + &n1;
	if n == &n2 {
		return base * exp;
	}
	let n3 = &n2 + &n1;
	if n == &n3 {
		return pow(base, exp);
	}
	if exp.is_zero() {
		return n1;
	}

	let n = n - &n1;
	let mut exp = exp.clone();

	let mut out = base.clone();
	while {
		exp -= &n1;
		!exp.is_zero()
	} {
		out = hyper_op(&n, base, &out);
	}
	out
}

#[allow(
	non_snake_case,
	clippy::must_use_candidate,
	clippy::needless_pass_by_value
	/*
	reason = "
		It's the std name, and the crate-name already provides context,
		OOM-panics are sort-of a side effect,
		I'm considering on making it pass-by-ref, but not now
	"
	*/
)]
///The Ackermann-Péter function
///
///For performance, this implementation is defined
///[like so](https://en.wikipedia.org/wiki/Ackermann_function#TRS,_based_on_hyperoperators)
pub fn A(m: BigUint, n: BigUint) -> BigUint {
	let n3 = BigUint::from(3_u8);
	hyper_op(&m, &BigUint::from(2_u8), &(n + &n3)) - n3
}

#[cfg(test)]
mod tests {

	use crate::A;
	use num_bigint::BigUint;

	#[test]
	fn it_works() {
		let result = A(BigUint::from(2_u8), BigUint::from(3_u8));
		assert_eq!(result, BigUint::from(9_u8));
	}
}
