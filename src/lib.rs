#![deny(clippy::unwrap_used)]
#![forbid(clippy::exit)]

//!Provives fns to calculate the Ackermann Function with arbitrary precision
//!
//!More precisely, the Ackermann-Péter fn.
//!
//!Input args of these fns can be stack-allocated (fixed-size ints) or heap-allocated (`num_bigint`)

use num_bigint::BigUint;
use num_traits::{One, Zero};

///Calculates `b` ^ `e` (unbounded).
///
///It uses [binary exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) algorithm.
///
///This helper is necessary because the `pow` trait only supports `u32` as `exp`,
///but we need **truly arbitrary** precision, for mathematical correctness.
fn big_pow(b: BigUint, e: &BigUint) -> BigUint {
	if *e <= BigUint::from(core::u32::MAX) {
		return b.pow(e.to_u32_digits()[0]);
	}

	if b.is_zero() || b.is_one() {
		return b;
	}

	let mut b = b;
	let mut e = e.clone();

	let mut out = BigUint::one();
	loop {
		if e.bit(0) {
			out *= &b;
		}
		e >>= 1;
		b = &b * &b;

		if e.is_one() {
			drop(e);
			break;
		}
	}
	out * b
}

///Calculates the [Hyper-Operation function](https://en.wikipedia.org/wiki/Hyperoperation#Definition)
///
///`n` is `n` ("order"), `base` is `a`, `exp` is `b`
///
///This helper is necessary because it's **way better** than the Ackermann fn.
///It's faster, uses less memory, and it's more readable, than the optimized ack fn with explicit stack.
///Also, it doesn't need memoization!
fn hyper_op(n: &BigUint, base: BigUint, exp: &BigUint) -> BigUint {
	if n.is_zero() {
		return exp + 1_u8;
	}
	if n.is_one() {
		return base + exp;
	}
	{
		let n1 = BigUint::one();
		let n2 = &n1 + &n1;
		if *n == n2 {
			drop([n1, n2]);
			return base * exp;
		}
		let n3 = n2 + &n1;
		if *n == n3 {
			drop([n1, n3]);
			return big_pow(base, exp);
		}
		if exp.is_zero() {
			return n1;
		}
	}

	let n = n - 1_u8;
	let mut exp = exp.clone();

	let mut out = base.clone();
	loop {
		exp -= 1_u8;
		if exp.is_zero() {
			break;
		}
		out = hyper_op(&n, base.clone(), &out);
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
pub fn A<T>(m: T, n: T) -> BigUint
where
	BigUint: std::convert::From<T>,
{
	let m = BigUint::from(m);
	let n = BigUint::from(n);

	let n2 = BigUint::from_slice(&[2]);

	hyper_op(&m, n2, &(n + 3_u8)) - 3_u8
}

#[cfg(test)]
mod tests {
	use super::A;
	use num_bigint::BigUint;
	use num_traits::One;

	#[test]
	fn table_cmp() {
		let mut m;

		m = 0;
		for n in 0..core::u8::MAX {
			assert_eq!(A(m, n), BigUint::from(n + 1));
		}

		m = 1;
		for n in 0..(core::u8::MAX - 1) {
			assert_eq!(A(m, n), BigUint::from(n + 2));
		}

		m = 2;
		for n in 0..(core::u8::MAX >> 2) {
			assert_eq!(A(m, n), BigUint::from(2 * n + 3));
		}

		m = 3;
		for n in 0..0x10 {
			assert_eq!(A(m, n), BigUint::from(2_u32.pow(u32::from(n) + 3) - 3));
		}

		m = 4;
		assert_eq!(A(m, 0), BigUint::from(13_u8));
		assert_eq!(A(m, 1), BigUint::from(0xFFFD_u16));
		assert_eq!(A(m, 2), (BigUint::one() << 0x1_00_00) - 3_u8);
	}
}
