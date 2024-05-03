use rfalse::falselang::vm::{FalseVM, StackElement};
use criterion::{criterion_group, criterion_main, Criterion};

fn perf() {
	let mut vm = FalseVM::new();
	// vm.load("   2 2 +  ");
	// vm.load("[2 2+]![2 2+]!");
	// vm.load("1[\"hello\"]? 0[\"hello\"]?");
	// vm.load("[$1=$[\\%1\\]?~[$1-f;!*]?]f:    15 f;!");  // factorial example
	// vm.load("123.");
	// vm.load("99b:\n[b;0=[\"No more bottles of beer\"]?b;1=[\"1 more bottle of beer\"]?b;1>[b;.\" bottles of beer\"]?]a:\n[b;0>][a;!\" on the wall\"10,a;!10,\"Take one down, pass it around\"10,b;1-b:a;!\" on the wall\n\"]#");
	// vm.load("^^^,,,");
	// vm.load("[$ 1 > [1- $ f;! \\ 1- f;! +]?]f:       33 f;!  {compute 33th fibonacci number}");
	vm.load("[$ 1 > [1- $ f;! \\ 1- f;! +]?]f:       15 f;!  {compute 10th fibonacci number}");
	// vm.load("99 9[1-$][\\$@$@$@$@\\/*=[1-$$[%\\1-$@]?0=[\\$.' ,\\]?]?]#");

	// vm.verbose = true;
	vm.run();
	// assert_eq!(vm.stack, vec![StackElement::Number(3524578)]);
	// assert_eq!(vm.stack, vec![StackElement::Number(3628800)]);
	assert_eq!(vm.stack, vec![StackElement::Number(610)]);
}

fn perf_benchmark(c: &mut Criterion) {
	c.bench_function("perf", |b| b.iter(|| perf()));
}

criterion_group!(benches, perf_benchmark);
criterion_main!(benches);
