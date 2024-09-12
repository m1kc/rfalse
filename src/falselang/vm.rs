use super::tokenizer::{Token, Tokenizer};

use std::{collections::HashMap, io::{self, Read, Write}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
	OK,
	End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackElement {
	Number(i64),
	Lambda(usize),  // contains function index in storage
	Variable(char),
}

impl StackElement {
	pub fn expect_number(&self) -> i64 {
		if let StackElement::Number(n) = self {
			return *n;
		}
		panic!("Expected number, got {:?}", self);
	}

	pub fn expect_lambda(&self) -> usize {
		if let StackElement::Lambda(l) = self {
			return *l;
		}
		panic!("Expected lambda, got {:?}", self);
	}

	pub fn expect_variable(&self) -> char {
		if let StackElement::Variable(v) = self {
			return *v;
		}
		panic!("Expected variable, got {:?}", self);
	}
}

#[derive(Debug)]
pub struct FalseVM {
	pub stack: Vec<StackElement>,
	pub variables: HashMap<char, StackElement>,
	pub functions: Vec<Vec<Token>>,

	pub fn_index: usize,
	pub cursor: usize,

	pub verbose: bool,

}

impl FalseVM {
	pub fn new() -> FalseVM {
		FalseVM {
			stack: Vec::new(),
			variables: HashMap::new(),
			fn_index: 0,
			cursor: 0,
			verbose: false,
			functions: Vec::new(),
		}
	}

	pub fn load(&mut self, code: &str) {
		let t = Tokenizer::new(code);
		let mut parser = super::parser::Parser::new(t);
		self.fn_index = match parser.parse() {
			Token::LambdaPointer(l) => l,
			_ => panic!("Expected lambda"),
		};
		self.functions = parser.lambda_storage;
		self.cursor = 0;
	}

	pub fn peek_instruction(&self) -> Option<&Token> {
		self.functions.get(self.fn_index).and_then(|v| v.get(self.cursor))
	}

	pub fn gosub(&mut self, lambda_index: usize) {
		// save
		let tmph = self.cursor;
		let tmp = self.fn_index.clone();
		// replace & run
		self.fn_index = lambda_index;
		self.cursor = 0;
		self.run();
		// restore
		self.fn_index = tmp;
		self.cursor = tmph;
	}

	pub fn step(&mut self) -> StepResult {
		assert!(self.functions.len() > 0, "invalid VM state (did you call load()?)");
		let curr = self.functions.get(self.fn_index).expect("wrong curr");
		if self.cursor >= curr.len() {
			return StepResult::End;
		}
		match &curr[self.cursor] {
			Token::Number(n) => self.stack.push(StackElement::Number(*n)),

			Token::Dup => {
				let a = self.stack.last().expect("Stack underflow").clone();
				self.stack.push(a);
			}
			Token::Drop => {
				self.stack.pop().expect("Stack underflow");
			}
			Token::Swap => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a);
				self.stack.push(b);
			}
			Token::Rot => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				let c = self.stack.pop().expect("Stack underflow");
				self.stack.push(b);
				self.stack.push(a);
				self.stack.push(c);
			}
			Token::Pick => {
				let n = self.stack.pop().expect("Stack underflow");
				if let StackElement::Number(idx) = n {
					let idx = self.stack.len() - 1 - idx as usize;
					let v = self.stack.get(idx).expect("Stack underflow").clone();
					self.stack.push(v);
				} else {
					panic!("Invalid index");
				}
			}

			Token::Plus => {
				let a: i64 = self.stack.pop().expect("Stack underflow").expect_number();
				let b: i64 = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(b + a));
			}
			Token::Minus => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(b - a));
			}
			Token::Mul => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(b * a));
			}
			Token::Div => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(b / a));
			}
			Token::Negate => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(-a));
			}
			Token::BitAnd => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(a & b));
			}
			Token::BitOr => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(a | b));
			}
			Token::BitNot => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(!a));
			}

			Token::GreaterThan => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(if a < b { !0 } else { 0 }));
			}
			Token::Equal => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(if a == b { !0 } else { 0 }));
			}
			Token::LessThan => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(if a > b { !0 } else { 0 }));
			}

			Token::LambdaPointer(v) => {
				self.stack.push(StackElement::Lambda(*v));
			}
			Token::LambdaExecute => {
				let l = self.stack.pop().expect("Stack underflow");
				let l = l.expect_lambda();
				self.gosub(l);
			}
			Token::LambdaIf => {
				let l = self.stack.pop().expect("Stack underflow");
				let l = l.expect_lambda();
				let cond = self.stack.pop().expect("Stack underflow").expect_number();
				if cond != 0 {
					self.gosub(l);
				}
			}
			Token::LambdaWhile => {
				let body = self.stack.pop().expect("Stack underflow");
				let body = body.expect_lambda();
				let cond = self.stack.pop().expect("Stack underflow");
				let cond = cond.expect_lambda();

				loop {
					self.gosub(cond);
					let cond = self.stack.pop().expect("Stack underflow").expect_number();
					if cond == 0 {
						break;
					}
					self.gosub(body);
				}
			}
			Token::LambdaStart => {
				panic!("LambdaStart must not be output by parser")
			}
			Token::LambdaEnd => {
				panic!("LambdaEnd must not be output by parser")
			}

			Token::Variable(x) => {
				self.stack.push(StackElement::Variable(*x))
			}
			Token::VarWrite => {
				let var = self.stack.pop().expect("Stack underflow").expect_variable();
				let val = self.stack.pop().expect("Stack underflow");
				self.variables.insert(var, val);
			}
			Token::VarRead => {
				let var = self.stack.pop().expect("Stack underflow").expect_variable();
				let val = self.variables.get(&var).expect("Variable not initialized").clone();
				self.stack.push(val);
			}

			Token::ReadChar => {
				// read char from stdin
				let mut buf = [0u8; 1];
				io::stdin().read_exact(&mut buf).unwrap();
				let c = buf[0] as i64;
				self.stack.push(StackElement::Number(c));
			}
			Token::WriteChar => {
				let c = self.stack.pop().expect("Stack underflow").expect_number();
				let c = std::char::from_u32(c as u32).expect("Invalid char");
				print!("{}", c);
			}
			Token::PrintString(s) => {
				print!("{}", s);
			}
			Token::WriteInt => {
				let n = self.stack.pop().expect("Stack underflow").expect_number();
				print!("{}", n);
			}
			Token::FlushIO => {
				std::io::stdout().flush().unwrap();
			}
		}
		self.cursor += 1;
		return StepResult::OK;
	}

	#[allow(dead_code)]
	pub fn run(&mut self) {
		loop {
			if self.verbose {
				println!("\n==> Doing instr: {:?}", self.peek_instruction());
			}
			let r = self.step();
			// wait for keystroke
			// std::io::stdin().read_line(&mut String::new()).unwrap();
			if r == StepResult::End {
				break;
			}
			if self.verbose {
				println!("\n==> Stack: {:?}", self.stack);
				println!("    Vars: {:?}", self.variables);
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_empty() {
		let mut vm = FalseVM::new();
		vm.load("");
		vm.run();
		assert_eq!(vm.stack.len(), 0);
	}

	#[test]
	fn test_put_i() {
		let mut vm = FalseVM::new();
		vm.load("1 2 3 4 5");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(1),
			StackElement::Number(2),
			StackElement::Number(3),
			StackElement::Number(4),
			StackElement::Number(5),
		]);
	}

	#[test]
	fn test_2plus2() {
		let mut vm = FalseVM::new();
		vm.load("2 2+");
		vm.run();
		assert_eq!(vm.stack, vec![ StackElement::Number(4) ]);
	}

	#[test]
	fn test_charcode() {
		let mut vm = FalseVM::new();
		vm.load("'a    'b         'c'd");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(97),
			StackElement::Number(98),
			StackElement::Number(99),
			StackElement::Number(100),
		]);
	}

	#[test]
	fn test_dup() {
		let mut vm = FalseVM::new();
		vm.load("2 4$");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(2),
			StackElement::Number(4),
			StackElement::Number(4),
		]);
	}

	#[test]
	fn test_drop() {
		let mut vm = FalseVM::new();
		vm.load("1 2 3%");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(1),
			StackElement::Number(2),
		]);
	}

	#[test]
	fn test_swap() {
		let mut vm = FalseVM::new();
		vm.load("1 2 \\");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(2),
			StackElement::Number(1),
		]);
	}

	#[test]
	fn test_rot() {
		let mut vm = FalseVM::new();
		vm.load("0 1 2 3 @");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(0),
			StackElement::Number(2),
			StackElement::Number(3),
			StackElement::Number(1),
		]);
	}

	#[test]
	fn test_pick_1() {
		let mut vm = FalseVM::new();
		vm.load("7 8 9 2 ø");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(7),
			StackElement::Number(8),
			StackElement::Number(9),
			StackElement::Number(7),
		]);
	}

	#[test]
	fn test_pick_2() {
		let mut vm = FalseVM::new();
		vm.load("7 8 9 2P");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(7),
			StackElement::Number(8),
			StackElement::Number(9),
			StackElement::Number(7),
		]);
	}

	#[test]
	fn test_negate() {
		let mut vm = FalseVM::new();
		vm.load("1920_");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(-1920),
		]);
	}

	#[test]
	fn test_bit_and() {
		let mut vm = FalseVM::new();
		vm.load("3 1 &");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(1),
		]);
	}

	#[test]
	fn test_bit_or() {
		let mut vm = FalseVM::new();
		vm.load("3   1|");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(3),
		]);
	}

	#[test]
	fn test_bit_not() {
		let mut vm = FalseVM::new();
		vm.load("5~");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(-6),
		]);
	}

	#[test]
	fn test_fn_factorial() {
		let mut vm = FalseVM::new();
		vm.load("[$1=$[\\%1\\]?~[$1-f;!*]?]f:    6 f;!");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(720),
		]);
	}

	#[test]
	fn test_fn_fibonacci() {
		let mut vm = FalseVM::new();
		vm.load("[$ 1 > [1- $ f;! \\ 1- f;! +]?]f: 12 f;!");
		vm.run();
		assert_eq!(vm.stack, vec![
			StackElement::Number(144),
		]);
	}

	#[test]
	fn test_fn_primes() {
		let mut vm = FalseVM::new();
		vm.load("50 9[1-$][\\$@$@$@$@\\/*=[1-$$[%\\1-$@]?0=[\\' ,\\]?]?]#");
		vm.run();
		assert_eq!(vm.stack, vec![
			// 47 43 41 37 31 29 23 19 17 13 11 7 5 3 2
			StackElement::Number(1),
			StackElement::Number(0),
		]);
	}
}
