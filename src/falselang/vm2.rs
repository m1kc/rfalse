use core::panic;
use std::collections::HashMap;

use crate::falselang::tokenizer::*;
use num_enum::{TryFromPrimitive, IntoPrimitive};

const MEM_SIZE: usize = 131072;
const FIRST_VAR: usize = 0;
const CALL_STACK_START: usize = FIRST_VAR + 26;
const CALL_STACK_SIZE: usize = 640;
const FIRST_INSTR: usize = CALL_STACK_START + CALL_STACK_SIZE;

pub struct FalseVM {
	pub memory: Box<[i32; MEM_SIZE]>,
	pub cursor: usize,
	pub stack_pointer: usize,
	pub callstack_pointer: usize,
	pub fn_pointer: HashMap<usize, usize>, // stores function pointers, key = fn_index, value = memory pointer

	pub verbose: bool,
}

#[repr(i32)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
pub enum Instr {
	Noop = 0,
	Push = 1001, // 1 args, +1 stack
	Dup = 1002, // 0 args, +1 stack
	Drop = 1003, // 0 args, -1 stack
	Swap = 1004, // 0 args, 0 stack
	Rot = 1005, // 0 args, 0 stack
	Pick = 1006, // 0 args, +1 stack
	Plus = 1007, // 0 args, -1 stack
	Minus = 1008, // 0 args, -1 stack
	Mul = 1009, // 0 args, -1 stack
	Div = 1010, // 0 args, -1 stack
	Negate = 1011, // 0 args, 0 stack
	BitAnd = 1012, // 0 args, -1 stack
	BitOr = 1013, // 0 args, -1 stack
	BitNot = 1014, // 0 args, 0 stack
	Gt = 1015, // 0 args, -1 stack
	Eq = 1016, // 0 args, -1 stack
	// ReadChar
	WriteChar = 1018, // 0 args, -1 stack
	WriteInt = 1019, // 0 args, -1 stack
	WriteString = 1020, // 1+N args, 0 stack

	Call = 1025, // 0 args, -1 stack
	CallIf = 1026, // 0 args, -2 stack

	Return = 1030, // 0 args, -1 stack
	Halt = 1031,
	VarRead = 1032, // 0 args, 0 stack
	VarWrite = 1033, // 0 args, -2 stack
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
	OK,
	End,
}

impl FalseVM {
	pub fn new() -> Self {
		FalseVM {
			// Memory layout:
			// * variables start at 0x00
			// * call stack starts at 26 (0x1A)
			// * instructions start at call stack size + 26 (0x1A)
			// * data stack starts at the end (and grows backwards)
			memory: Box::new([0; MEM_SIZE]),
			cursor: FIRST_INSTR,
			stack_pointer: MEM_SIZE,
			callstack_pointer: CALL_STACK_START - 1,
			fn_pointer: HashMap::new(),

			verbose: false,
		}
	}

	/// Compiles a function and puts it into memory. Returns start addr.
	pub fn compile_fn(&mut self, code: &Vec<Token>, epilogue: Instr) -> usize {
		let ret = self.cursor;
		if self.verbose {
			println!("Compiling function: {:?}", code);
			println!("Function address: {}", ret);
		}
		for token in code {
			if self.verbose {
				println!(" Token: {:?}", token);
			}
			match token {
				Token::Number(x) => self.instr_push1(Instr::Push, *x as i32),

				Token::Dup => self.instr_push(Instr::Dup),
				Token::Drop => self.instr_push(Instr::Drop),
				Token::Swap => self.instr_push(Instr::Swap),
				Token::Rot => self.instr_push(Instr::Rot),
				Token::Pick => self.instr_push(Instr::Pick),

				Token::Plus => self.instr_push(Instr::Plus),
				Token::Minus => self.instr_push(Instr::Minus),
				Token::Mul => self.instr_push(Instr::Mul),
				Token::Div => self.instr_push(Instr::Div),
				Token::Negate => self.instr_push(Instr::Negate),
				Token::BitAnd => self.instr_push(Instr::BitAnd),
				Token::BitOr => self.instr_push(Instr::BitOr),
				Token::BitNot => self.instr_push(Instr::BitNot),

				Token::GreaterThan => self.instr_push(Instr::Gt),
				Token::Equal => self.instr_push(Instr::Eq),

				Token::Variable(x) => self.instr_push1(Instr::Push, (*x as i32) - ('a' as i32)),
				Token::VarRead => self.instr_push(Instr::VarRead),
				Token::VarWrite => self.instr_push(Instr::VarWrite),

				Token::LambdaExecute => self.instr_push(Instr::Call),
				Token::LambdaPointer(n) => self.instr_push1(Instr::Push, self.fn_pointer.get(&n).unwrap().clone() as i32),
				Token::LambdaIf => self.instr_push(Instr::CallIf),
				Token::LambdaWhile => {
					self.instr_push(Instr::CallIf);
				}

				Token::PrintString(s) => {
					self.instr_push1(Instr::WriteString, s.chars().count() as i32);
					for c in s.chars() {
						self.instr_push_raw(c as i32);
					}
				}
				Token::WriteInt => self.instr_push(Instr::WriteInt),
				Token::WriteChar => self.instr_push(Instr::WriteChar),

				unknown => panic!("compile_fn: Not implemented: {:?}", unknown),
			}
		}
		self.instr_push(epilogue);
		ret
	}

	pub fn load(&mut self, code: &str) {
		let t = Tokenizer::new(code);
		let mut parser = super::parser::Parser::new(t);

		let fn_index = match parser.parse() {
			Token::LambdaPointer(l) => l,
			_ => panic!("Expected lambda"),
		};
		let functions = parser.lambda_storage;

		let mut n = 0;
		let mut entrypoint: usize = 0;
		for function in functions.iter() {
			let is_main = n == functions.len() - 1;
			let addr = self.compile_fn(function, if is_main { Instr::Halt } else { Instr::Return });
			if self.verbose {
				println!("Saving function #{} as address {}", n, addr);
			}
			self.fn_pointer.insert(n, addr);
			n += 1;
			entrypoint = addr;
		}

		self.goto(entrypoint);
	}

	/// Returns the next memory cell under cursor, shifting cursor forward.
	pub fn instr_consume(&mut self) -> i32 {
		let ret = self.memory[self.cursor];
		self.cursor += 1;
		return ret;
	}

	pub fn instr_push(&mut self, x: Instr) {
		self.instr_push_raw(x as i32)
	}

	pub fn instr_push1(&mut self, x: Instr, arg1: i32) {
		self.instr_push_raw(x as i32);
		self.instr_push_raw(arg1 as i32);
	}

	pub fn instr_push2(&mut self, x: Instr, arg1: i32, arg2: i32) {
		self.instr_push_raw(x as i32);
		self.instr_push_raw(arg1 as i32);
		self.instr_push_raw(arg2 as i32);
	}

	pub fn instr_push_raw(&mut self, x: i32) {
		self.memory[self.cursor] = x;
		self.cursor += 1;
	}

	pub fn goto(&mut self, addr: usize) {
		self.cursor = addr
	}

	/// Pushes a new element onto the stack, shifting stack top pointer to the left.
	pub fn push(&mut self, x: i32) {
		self.stack_pointer -= 1;
		self.memory[self.stack_pointer] = x;
	}

	pub fn peek(&self) -> i32 {
		self.memory[self.stack_pointer]
	}

	pub fn pop(&mut self) -> i32 {
		let ret = self.memory[self.stack_pointer];
		self.memory[self.stack_pointer] = 0; // optional
		self.stack_pointer += 1;
		return ret;
	}

	pub fn stack_size(&self) -> usize {
		MEM_SIZE - self.stack_pointer
	}

	pub fn dump1(&self) {
		let mut zeroes = 0;
		for x in self.memory.iter() {
			if *x == 0 && zeroes > 10 {
				continue;
			}
			print!("{} ", x);
			if *x == 0 {
				zeroes += 1;
			} else {
				zeroes = 0
			}
			if zeroes > 10 {
				print!("[...] ")
			}
		}
		println!("\n");
	}

	pub fn dump2(&self) {
		println!("Cursor is at {}", self.cursor);
		let line_size = 10;
		let mut line_idx = 0;
		let mut addr = 0;
		let mut line: String = "".to_string();
		let mut all_zero = true;
		for x in self.memory.iter() {
			if *x != 0 {
				all_zero = false;
			}
			if line_idx == 0 {
				line.push_str(format!("{:6}| ", addr).as_str());
			}
			if addr == self.cursor || addr == self.stack_pointer {
				line.push_str(format!("[{:5}]", x).as_str());
			} else {
				line.push_str(format!("{:6} ", x).as_str());
			}

			addr += 1;
			line_idx += 1;

			if line_idx >= line_size {
				if !all_zero || addr < 30 {
					println!("{}", line);
				}
				line_idx = 0;
				all_zero = true;
				line = String::new();
			}
		}
		if !all_zero {
			println!("{}", line);
		}
	}

	pub fn callstack_push(&mut self, x: i32) {
		self.callstack_pointer += 1;
		self.memory[self.callstack_pointer] = x;
	}

	pub fn callstack_pop(&mut self) -> i32 {
		let ret = self.memory[self.callstack_pointer];
		self.memory[self.callstack_pointer] = 0;  // optional
		self.callstack_pointer -= 1;
		ret
	}

	pub fn step(&mut self) -> StepResult {
		if self.verbose {
			println!();
			println!();
			self.dump2();
		}

		let opcode = Instr::try_from(self.instr_consume()).expect("Invalid opcode");

		if self.verbose {
			println!("Step: {:?}", opcode);
		}

		match opcode {
			// Instr::Noop => StepResult::OK,
			Instr::Noop => panic!("noop"),
			Instr::Push => {
				let arg = self.instr_consume();
				self.push(arg);
				StepResult::OK
			}

			Instr::Dup => {
				let x = self.peek();
				self.push(x);
				StepResult::OK
			}
			Instr::Drop => {
				_ = self.pop();
				StepResult::OK
			}
			Instr::Swap => {
				let a = self.pop();
				let b = self.pop();
				self.push(a);
				self.push(b);
				StepResult::OK
			}
			Instr::Rot => {
				let a = self.pop();
				let b = self.pop();
				let c = self.pop();
				self.push(b);
				self.push(a);
				self.push(c);
				StepResult::OK
			}
			Instr::Pick => {
				let n = self.pop();
				let addr: usize = (self.stack_pointer as i32 + n) as usize;
				self.push(self.memory[addr]);
				StepResult::OK
			}

			Instr::Plus => {
				let a = self.pop();
				let b = self.pop();
				self.push(b + a);
				StepResult::OK
			}
			Instr::Minus => {
				let a = self.pop();
				let b = self.pop();
				self.push(b - a);
				StepResult::OK
			}
			Instr::Mul => {
				let a = self.pop();
				let b = self.pop();
				self.push(b * a);
				StepResult::OK
			}
			Instr::Div => {
				let a = self.pop();
				let b = self.pop();
				self.push(b / a);
				StepResult::OK
			}
			Instr::Negate => {
				let a = self.pop();
				self.push(-a);
				StepResult::OK
			}
			Instr::BitAnd => {
				let a = self.pop();
				let b = self.pop();
				self.push(a & b);
				StepResult::OK
			}
			Instr::BitOr => {
				let a = self.pop();
				let b = self.pop();
				self.push(a | b);
				StepResult::OK
			}
			Instr::BitNot => {
				let a = self.pop();
				self.push(!a);
				StepResult::OK
			}

			Instr::Gt => {
				let a = self.pop();
				let b = self.pop();
				self.push(if b > a { !0 } else { 0 });
				StepResult::OK
			}
			Instr::Eq => {
				let a = self.pop();
				let b = self.pop();
				self.push(if a == b { !0 } else { 0 });
				StepResult::OK
			}

			Instr::WriteString => {
				let n = self.instr_consume();
				for _ in 0..n {
					let a = self.instr_consume();
					print!("{}", a as u8 as char);
				}
				StepResult::OK
			}
			Instr::WriteChar => {
				let a = self.pop();
				print!("{}", a as u8 as char);
				StepResult::OK
			}
			Instr::WriteInt => {
				let a = self.pop();
				print!("{}", a);
				StepResult::OK
			}

			Instr::Call => {
				let addr = self.pop();
				self.callstack_push(self.cursor as i32);
				self.goto(addr as usize);
				StepResult::OK
			}
			Instr::CallIf => {
				let body_addr = self.pop();
				let cond = self.pop();
				if cond != 0 {
					self.callstack_push(self.cursor as i32);
					self.goto(body_addr as usize);
				}
				StepResult::OK
			}

			Instr::Return => {
				let addr = self.callstack_pop();
				self.goto(addr as usize);
				StepResult::OK
			}
			Instr::Halt => StepResult::End,
			Instr::VarRead => {
				let n = self.pop();
				self.push(self.memory[FIRST_VAR + n as usize]);
				StepResult::OK
			}
			Instr::VarWrite => {
				let var = self.pop();
				let value = self.pop();
				self.memory[FIRST_VAR + var as usize] = value;
				StepResult::OK
			}
		}
	}

	pub fn run(&mut self) {
		let mut total = 0;
		loop {
			let result = self.step();
			if result == StepResult::End {
				return
			}
			total += 1;
			// if total > 50 { panic!("I'm tired"); }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_vm_2plus2() {
		let mut vm = FalseVM::new();
		vm.instr_push(Instr::Push);
		vm.instr_push_raw(2);
		vm.instr_push(Instr::Push);
		vm.instr_push_raw(2);
		vm.instr_push(Instr::Plus);
		vm.instr_push(Instr::Halt);
		vm.goto(FIRST_INSTR);
		vm.run();
		assert_eq!(vm.stack_pointer, MEM_SIZE - 1);
		assert_eq!(vm.memory[vm.stack_pointer], 4);
	}

	#[test]
	fn test_empty() {
		let mut vm = FalseVM::new();
		vm.load("");
		vm.run();
		assert_eq!(vm.stack_size(), 0);
	}

	#[test]
	fn test_2plus2() {
		let mut vm = FalseVM::new();
		vm.load("2 2 +");
		vm.run();
		assert_eq!(vm.stack_size(), 1);
		assert_eq!(vm.pop(), 4);
	}

	#[test]
	fn test_put_i() {
		let mut vm = FalseVM::new();
		vm.load("1 2 3 4 5");
		vm.run();
		assert_eq!(vm.stack_size(), 5);
		for i in [5, 4, 3, 2, 1] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_charcode() {
		let mut vm = FalseVM::new();
		vm.load("'a    'b         'c'd");
		vm.run();
		assert_eq!(vm.stack_size(), 4);
		for i in [100, 99, 98, 97] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_dup() {
		let mut vm = FalseVM::new();
		vm.load("2 4$");
		vm.run();
		assert_eq!(vm.stack_size(), 3);
		for i in [4, 4, 2] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_drop() {
		let mut vm = FalseVM::new();
		vm.load("1 2 3%");
		vm.run();
		assert_eq!(vm.stack_size(), 2);
		for i in [2, 1] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_swap() {
		let mut vm = FalseVM::new();
		vm.load("1 2 \\");
		vm.run();
		assert_eq!(vm.stack_size(), 2);
		for i in [1, 2] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_rot() {
		let mut vm = FalseVM::new();
		vm.load("0 1 2 3 @");
		vm.run();
		assert_eq!(vm.stack_size(), 4);
		for i in [1, 3, 2, 0] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_pick_1() {
		let mut vm = FalseVM::new();
		vm.load("7 8 9 2 ø");
		vm.run();
		assert_eq!(vm.stack_size(), 4);
		for i in [7, 9, 8, 7] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_pick_2() {
		let mut vm = FalseVM::new();
		vm.load("7 8 9 2P");
		vm.run();
		assert_eq!(vm.stack_size(), 4);
		for i in [7, 9, 8, 7] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_negate() {
		let mut vm = FalseVM::new();
		vm.load("1920_");
		vm.run();
		assert_eq!(vm.stack_size(), 1);
		for i in [-1920] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_if() {
		let mut vm = FalseVM::new();
		vm.load("1[777]?  0[333]?  2 2+ 4=[777]?");
		vm.run();
		assert_eq!(vm.stack_size(), 2);
		for i in [777, 777] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_vars_1() {
		let mut vm = FalseVM::new();
		vm.load("50 f: 1 f; +");
		vm.run();
		assert_eq!(vm.stack_size(), 1);
		for i in [51] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_vars_2() {
		let mut vm = FalseVM::new();
		vm.load("[1 +]f: 50 f;!");
		vm.run();
		assert_eq!(vm.stack_size(), 1);
		for i in [51] {
			assert_eq!(vm.pop(), i);
		}
	}

	#[test]
	fn test_fn_factorial() {
		let mut vm = FalseVM::new();
		vm.load("[$1=$[\\%1\\]?~[$1-f;!*]?]f:    6 f;!");
		vm.run();
		assert_eq!(vm.stack_size(), 1);
		for i in [720] {
			assert_eq!(vm.pop(), i);
		}
	}
}
