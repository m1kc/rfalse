mod falselang;
use falselang::vm::{FalseVM, StepResult};


fn main() {
	let mut vm = FalseVM::new();
	println!("Parsing...");
	vm.load("   2 2 +  ");

	loop {
		println!("Next instr: {:?}", vm.peek_instruction());
		let r = vm.step();
		println!("Stack: {:?}", vm.stack);
		if r == StepResult::End {
			break;
		}
	}
	println!("Run complete.");
}
