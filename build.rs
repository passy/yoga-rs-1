extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;
use gcc::Config;

fn main() {
	let mut c = Config::new();

	c.flag("-std=c99");
	c.file("src/c/YGEnums.c");
	c.file("src/c/YGNodeList.c");
	c.file("src/c/Yoga.c");
	c.compile("libyoga.a");

	let bindings = bindgen::Builder::default()
		.hide_type("max_align_t") // This fails `cargo test` so disable for now
		.hide_type("FP_INFINITE")
		.hide_type("FP_NAN")
		.hide_type("FP_NORMAL")
		.hide_type("FP_SUBNORMAL")
		.hide_type("FP_ZERO")
		.header("src/c/wrapper.h")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings!");
}
