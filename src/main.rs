#![warn(
	unused,
	future_incompatible,
	clippy::unwrap_used,
	clippy::cargo,
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
#![forbid(
	unsafe_code,
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

fn print_help() {
	println!(
		"usage: ack m n\n\
		where `n` and `m` are integer decimal numerals\n\
		only the 1st 2 arguments are used, everyting else is ignored"
	);
}

fn main() {
	use ackermann::A;
	use num_bigint::BigUint;
	use std::str::FromStr;

	let args: Vec<String> = std::env::args().skip(1).take(2).collect();

	if args.is_empty() {
		return print_help();
	};
	let m = &args[0].to_ascii_lowercase();
	if m == "help" || m == "/?" {
		return print_help();
	};

	let m = BigUint::from_str(m).expect("Cannot parse `m`");

	let n = &args[1];

	let n = BigUint::from_str(n).expect("Cannot parse `n`");

	//we need as much memory as possible for the next step
	drop(args);

	println!("{}", A(m, n));
}
