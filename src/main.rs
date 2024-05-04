mod falselang;
use falselang::vm::FalseVM;
use std::{io::Read, time::Instant};


fn main() {
	let args: Vec<String> = std::env::args().collect();
	// if a filename is provided, read from file
	let code = if args.len() == 2 {
		std::fs::read_to_string(&args[1]).expect("Failed to read file")
	} else {
		// otherwise, read from stdin until EOF
		let mut code = String::new();
		std::io::stdin().read_to_string(&mut code).expect("Failed to read stdin");
		code
	};

	let mut vm = FalseVM::new();
	print!("Parsing... ");
	let start = Instant::now();
	vm.load(&code);
	println!("ok, {:?}", start.elapsed());

	println!("Running...\n");
	let start = Instant::now();
	// vm.verbose = true;
	vm.run();
	println!("\n\nRun complete, {:?}", start.elapsed());
}
