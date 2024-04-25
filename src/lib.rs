#![deny(clippy::unwrap_used)]
#![forbid(clippy::exit)]

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Calculates `b` ^ `e` (unbounded).
///
/// It uses [binary exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) algorithm.
///
/// This helper is necessary because the `pow` method only supports `u32` as `exp`,
/// but we need **truly arbitrary** precision, for mathematical correctness.
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

/// Calculates the [Hyper-Operation function](https://en.wikipedia.org/wiki/Hyperoperation#Definition)
///
/// `n` is "order" or "degree", `base` is `a`, `exp` is `b`
#[allow(non_snake_case)]
pub fn H(n: &BigUint, base: BigUint, exp: &BigUint) -> BigUint {
	if n.is_zero() {
		return exp + 1_u8;
	}
	if n.is_one() {
		return base + exp;
	}
	{
		let n0 = BigUint::zero();
		let n1 = BigUint::one();
		let n2 = &n1 + &n1;

		if *n == n2 {
			drop([n1, n2]);
			return base * exp;
		}
		let n3 = n2.clone() + &n1;
		if *n == n3 {
			drop([n1, n3]);
			return big_pow(base, exp);
		}
		let n4 = n3 + &n1;
		debug_assert!(n >= &n4);

		if base.is_zero() {
			return if (exp % 2u8).is_zero() { n1 } else { n0 };
		}
		if base.is_one() {
			return n1;
		}
		debug_assert!(base >= n2.clone());

		if exp.is_zero() {
			return n1;
		}
		if exp.is_one() {
			return base;
		}
		debug_assert!(exp >= &n2);

		if base == n2 && exp == &n2 {
			return n4;
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
		out = H(&n, base.clone(), &out);
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
/// The Ackermann-Péter function
///
/// For performance, this implementation is defined
/// [like so](https://en.wikipedia.org/wiki/Ackermann_function#TRS,_based_on_hyperoperators)
pub fn A(m: BigUint, n: BigUint) -> BigUint {
	let n2 = BigUint::from(2u8);
	H(&m, n2, &(n + 3_u8)) - 3_u8
}

#[allow(non_snake_case)]
/// https://en.wikipedia.org/wiki/Graham%27s_number
pub fn Graham(mut n: BigUint) -> BigUint {
	let n3 = BigUint::from(3u8);

	let mut x = BigUint::from(4u8);
	while !n.is_zero() {
		n -= 1u8;
		x = H(&(x + BigUint::from(2u8)), n3.clone(), &n3);
	}
	x
}

#[cfg(test)]
mod tests {
	#[allow(clippy::wildcard_imports)]
	use super::*;
	use num_bigint::BigUint;
	use num_traits::One;

	#[test]
	fn table_cmp() {
		let mut m = BigUint::zero();
		for n in 0..core::u8::MAX {
			assert_eq!(A(m.clone(), BigUint::from(n)), BigUint::from(n + 1));
		}

		m = BigUint::one();
		for n in 0..(core::u8::MAX - 1) {
			assert_eq!(A(m.clone(), BigUint::from(n)), BigUint::from(n + 2));
		}

		m = BigUint::from(2u8);
		for n in 0..(core::u8::MAX >> 2) {
			assert_eq!(A(m.clone(), BigUint::from(n)), BigUint::from(2 * n + 3));
		}

		m = BigUint::from(3u8);
		for n in 0..0x10u8 {
			assert_eq!(
				A(m.clone(), BigUint::from(n)),
				BigUint::from(2_u32.pow(u32::from(n) + 3) - 3)
			);
		}

		m = BigUint::from(4u8);
		assert_eq!(A(m.clone(), BigUint::zero()), BigUint::from(13_u8));
		assert_eq!(A(m.clone(), BigUint::one()), BigUint::from(0xFFFD_u16));
		assert_eq!(
			A(m.clone(), BigUint::from(2u8)),
			(BigUint::one() << 0x1_00_00) - 3_u8
		);
	}
}
