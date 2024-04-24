#![warn(
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
	clippy::format_push_string
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
		"usage: hyper_op n base exp\n\
		where all are Natural decimal numerals\n"
	);
}

fn main() {
	use hyper_op::H;
	use num_bigint::BigUint;
	use std::str::FromStr;

	let args: Vec<String> = std::env::args().skip(1).take(3).collect();

	if args.is_empty() {
		return print_help();
	};
	let a0 = &args[0].to_ascii_lowercase();
	if a0 == "help" || a0 == "?" {
		return print_help();
	};

	let a0 = BigUint::from_str(a0).expect("Cannot parse `n`");
	let a1 = BigUint::from_str(&args[1]).expect("Cannot parse `base`");
	let a2 = BigUint::from_str(&args[2]).expect("Cannot parse `exp`");

	// we need as much memory as possible for the next step
	drop(args);

	println!("{}", H(&a0, a1, &a2));
}
