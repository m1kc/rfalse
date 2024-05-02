mod falselang;
use falselang::vm::FalseVM;
use std::time::Instant;


fn main() {
	let mut vm = FalseVM::new();
	print!("Parsing... ");
	let start = Instant::now();
	// vm.load("   2 2 +  ");
	// vm.load("[2 2+]![2 2+]!");
	// vm.load("1[\"hello\"]? 0[\"hello\"]?");
	vm.load("[$1=$[\\%1\\]?~[$1-f;!*]?]f:    5 f;!");
	println!("ok, {:?}", start.elapsed());

	println!("Running...");
	vm.runv(true);
	println!("Run complete.");
}
