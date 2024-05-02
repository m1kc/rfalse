use super::tokenizer::{Token, Tokenizer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
	OK,
	End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackElement {
	Number(i64),
	Lambda(Vec<Token>),
}

impl StackElement {
	pub fn expect_number(&self) -> i64 {
		if let StackElement::Number(n) = self {
			return *n;
		}
		panic!("Expected number, got {:?}", self);
	}

	pub fn expect_lambda(&self) -> &Vec<Token> {
		if let StackElement::Lambda(l) = self {
			return l;
		}
		panic!("Expected lambda, got {:?}", self);
	}
}

pub struct FalseVM {
	pub stack: Vec<StackElement>,

	pub instructions: Vec<Token>,
	pub head: usize,
}

impl FalseVM {
	pub fn new() -> FalseVM {
		FalseVM {
			stack: Vec::new(),
			instructions: Vec::new(),
			head: 0,
		}
	}

	pub fn load(&mut self, code: &str) {
		let t = Tokenizer::new(code);
		let mut parser = super::parser::Parser::new(t);
		self.instructions = parser.all();
		self.head = 0;
	}

	pub fn peek_instruction(&self) -> Option<&Token> {
		self.instructions.get(self.head)
	}

	pub fn step(&mut self) -> StepResult {
		if self.head >= self.instructions.len() {
			return StepResult::End;
		}
		match self.instructions[self.head] {
			Token::Number(n) => self.stack.push(StackElement::Number(n)),

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
				self.stack.push(StackElement::Number(a + b));
			}
			Token::Minus => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(a - b));
			}
			Token::Mul => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(a * b));
			}
			Token::Div => {
				let a = self.stack.pop().expect("Stack underflow").expect_number();
				let b = self.stack.pop().expect("Stack underflow").expect_number();
				self.stack.push(StackElement::Number(a / b));
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
				todo!("GreaterThan not implemented")
			}
			Token::Equal => {
				todo!("Equal not implemented")
			}
			Token::LessThan => {
				todo!("LessThan not implemented")
			}

			Token::LambdaExecute => {
				todo!("LambdaExecute not implemented")
			}
			Token::LambdaIf => {
				todo!("LambdaIf not implemented")
			}
			Token::LambdaWhile => {
				todo!("LambdaWhile not implemented")
			}
			Token::ParsedLambda(_) => {
				todo!("ParsedLambda not implemented")
			}
			Token::LambdaStart => {
				panic!("LambdaStart must not be output by parser")
			}
			Token::LambdaEnd => {
				panic!("LambdaEnd must not be output by parser")
			}

			Token::Variable(_) => {
				todo!("Variable not implemented")
			}
			Token::VarWrite => {
				todo!("VarWrite not implemented")
			}
			Token::VarRead => {
				todo!("VarRead not implemented")
			}

			Token::ReadChar => {
				todo!("ReadChar not implemented")
			}
			Token::WriteChar => {
				todo!("WriteChar not implemented")
			}
			Token::PrintString(_) => {
				todo!("PrintString not implemented")
			}
			Token::WriteInt => {
				todo!("WriteInt not implemented")
			}
			Token::FlushIO => {
				todo!("FlushIO not implemented")
			}
		}
		self.head += 1;
		return StepResult::OK;
	}

	#[allow(dead_code)]
	pub fn run(&mut self) {
		return self.runv(false);
	}

	pub fn runv(&mut self, verbose: bool) {
		loop {
			if verbose {
				println!("Next instr: {:?}", self.peek_instruction());
			}
			let r = self.step();
			if r == StepResult::End {
				break;
			}
			if verbose {
				println!("Stack: {:?}", self.stack);
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
}
