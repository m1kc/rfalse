#[cfg(target_arch = "wasm32")]
mod falselang;
#[cfg(target_arch = "wasm32")]
use falselang::vm::FalseVM;
// use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use log::*;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use std::panic;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_main() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	let _ = console_log::init_with_level(Level::Debug);
	info!("Hello from wasm_main!");

	let mut vm = FalseVM::new();
	info!("Parsing... ");
	// let start = Instant::now();
	// vm.load("   2 2 +  ");
	// vm.load("[2 2+]![2 2+]!");
	// vm.load("1[\"hello\"]? 0[\"hello\"]?");
	// vm.load("[$1=$[\\%1\\]?~[$1-f;!*]?]f:    \"factorial of \" 5 $. \" is \" f;!.");  // factorial example
	// vm.load("123.");
	// vm.load("99b:\n[b;0=[\"No more bottles of beer\"]?b;1=[\"1 more bottle of beer\"]?b;1>[b;.\" bottles of beer\"]?]a:\n[b;0>][a;!\" on the wall\"10,a;!10,\"Take one down, pass it around\"10,b;1-b:a;!\" on the wall\n\"]#");
	// vm.load("^^^,,,");
	vm.load("[$ 1 > [1- $ f;! \\ 1- f;! +]?]f:       33 f;! $. {compute & print 33th fibonacci number}");
	// info!("ok, {:?}", start.elapsed());
	info!("ok");

	info!("Running...");
	// let start = Instant::now();
	// vm.verbose = true;
	vm.run();
	// info!("\nRun complete, {:?}", start.elapsed());
	info!("Run complete");
	info!("{:?}", vm.stack);
}
