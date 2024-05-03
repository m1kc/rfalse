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
	// vm.load("[$1=$[\\%1\\]?~[$1-f;!*]?]f:    \"factorial of \" 5 $. \" is \" f;!.");  // factorial example
	// vm.load("123.");
	// vm.load("99b:\n[b;0=[\"No more bottles of beer\"]?b;1=[\"1 more bottle of beer\"]?b;1>[b;.\" bottles of beer\"]?]a:\n[b;0>][a;!\" on the wall\"10,a;!10,\"Take one down, pass it around\"10,b;1-b:a;!\" on the wall\n\"]#");
	// vm.load("^^^,,,");
	// vm.load("[$ 1 > [1- $ f;! \\ 1- f;! +]?]f:       33 f;! . {compute & print 33th fibonacci number}");
	vm.load("1999 9[1-$][\\$@$@$@$@\\/*=[1-$$[%\\1-$@]?0=[\\$.' ,\\]?]?]#");
	println!("ok, {:?}", start.elapsed());

	println!("Running...");
	let start = Instant::now();
	// vm.verbose = true;
	vm.run();
	println!("\nRun complete, {:?}", start.elapsed());
}
