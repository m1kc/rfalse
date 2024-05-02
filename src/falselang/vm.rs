use super::Token;
use super::parser::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
	OK,
	End,
}

pub struct FalseVM {
	pub stack: Vec<i64>,

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
		let mut p = Parser::new(code);
		self.instructions = p.parse_all();
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
			Token::Number(n) => self.stack.push(n),

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
				let idx = self.stack.len() - 1 - n as usize;
				let v = self.stack.get(idx).expect("Stack underflow").clone();
				self.stack.push(v);
			}

			Token::Plus => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a + b);
			}
			Token::Minus => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a - b);
			}
			Token::Mul => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a * b);
			}
			Token::Div => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a / b);
			}
			Token::Negate => {
				let a = self.stack.pop().expect("Stack underflow");
				self.stack.push(-a);
			}
			Token::BitAnd => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a & b);
			}
			Token::BitOr => {
				let a = self.stack.pop().expect("Stack underflow");
				let b = self.stack.pop().expect("Stack underflow");
				self.stack.push(a | b);
			}
			Token::BitNot => {
				let a = self.stack.pop().expect("Stack underflow");
				self.stack.push(!a);
			}
		}
		self.head += 1;
		return StepResult::OK;
	}

	#[allow(dead_code)]
	pub fn run(&mut self) {
		loop {
			let r = self.step();
			if r == StepResult::End {
				break;
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
		assert_eq!(vm.stack, vec![1, 2, 3, 4, 5]);
	}

	#[test]
	fn test_2plus2() {
		let mut vm = FalseVM::new();
		vm.load("2 2+");
		vm.run();
		assert_eq!(vm.stack, vec![4]);
	}

	#[test]
	fn test_charcode() {
		let mut vm = FalseVM::new();
		vm.load("'a    'b         'c'd");
		vm.run();
		assert_eq!(vm.stack, vec![97, 98, 99, 100]);
	}

	#[test]
	fn test_dup() {
		let mut vm = FalseVM::new();
		vm.load("2 4$");
		vm.run();
		assert_eq!(vm.stack, vec![2, 4, 4]);
	}

	#[test]
	fn test_drop() {
		let mut vm = FalseVM::new();
		vm.load("1 2 3%");
		vm.run();
		assert_eq!(vm.stack, vec![1, 2]);
	}

	#[test]
	fn test_swap() {
		let mut vm = FalseVM::new();
		vm.load("1 2 \\");
		vm.run();
		assert_eq!(vm.stack, vec![2, 1]);
	}

	#[test]
	fn test_rot() {
		let mut vm = FalseVM::new();
		vm.load("0 1 2 3 @");
		vm.run();
		assert_eq!(vm.stack, vec![0, 2, 3, 1]);
	}

	#[test]
	fn test_pick_1() {
		let mut vm = FalseVM::new();
		vm.load("7 8 9 2 ø");
		vm.run();
		assert_eq!(vm.stack, vec![7, 8, 9, 7]);
	}

	#[test]
	fn test_pick_2() {
		let mut vm = FalseVM::new();
		vm.load("7 8 9 2p");
		vm.run();
		assert_eq!(vm.stack, vec![7, 8, 9, 7]);
	}

	#[test]
	fn test_negate() {
		let mut vm = FalseVM::new();
		vm.load("1920_");
		vm.run();
		assert_eq!(vm.stack, vec![-1920]);
	}

	#[test]
	fn test_bit_and() {
		let mut vm = FalseVM::new();
		vm.load("3 1 &");
		vm.run();
		assert_eq!(vm.stack, vec![1]);
	}

	#[test]
	fn test_bit_or() {
		let mut vm = FalseVM::new();
		vm.load("3   1|");
		vm.run();
		assert_eq!(vm.stack, vec![3]);
	}

	#[test]
	fn test_bit_not() {
		let mut vm = FalseVM::new();
		vm.load("5~");
		vm.run();
		assert_eq!(vm.stack, vec![-6]);
	}
}
