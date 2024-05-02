use super::Token;

/// Supported tokens:
///
/// ### Literals
/// - `123` put integer onto the stack
/// - `'c` put character code onto the stack
///
/// ### Stack operations
/// - `$` DUP
/// - `%` DROP
/// - `\` SWAP
/// - `@` ROT
/// - `ø` or `P` PICK (dup the nth stack item). **Note**: `P` is rfalse extension
///
/// ### Arithmetic
/// - `+`
/// - `-`
/// - `*`
/// - `/`
/// - `_` negate (negative numbers are entered "123_")
/// - `&` bitwise AND
/// - `|` bitwise OR
/// - `~` bitwise NOT
///
/// ### Comparison
/// False is 0. True is all bits set (-1 or ~0), so that bitwise operators can be used.
///
/// - `>` greater than
/// - `=` equals
/// - `<` less than. **Note**: `<` is rfalse extension
///
/// ### Lambdas and control flow
/// - `[...]` define and put a lambda onto the stack
/// - `!` execute lambda
/// - `?` conditional execute: `condition[true]?`. If-else can be expressed as: `condition$[\true\]?~[false]?`
/// - `#` while loop: `[condition][body]#` (tests for non-zero)
///
/// ### Names
/// - `a-z` put a reference to one of the 26 available variables onto the stack
/// - `:` store into a variable
/// - `;` fetch from a variable
///
/// ### I/O
/// - `^` read a character (-1 for EOF)
/// - `,` write a character
/// - `"string"` write a string (may contain embedded newlines)
/// - `.` write top of stack as a decimal integer
/// - `ß` or `B` flush buffered input/output. **Note**: `B` is rfalse extension
///
/// ### Other
/// - `{...}` comment
pub struct Tokenizer {
	code: String,
}

const SIMPLE_TOKENS: [(char, Token); 55] = [
	('$', Token::Dup),
	('%', Token::Drop),
	('\\', Token::Swap),
	('@', Token::Rot),
	('ø', Token::Pick),
	('P', Token::Pick),

	('+', Token::Plus),
	('-', Token::Minus),
	('*', Token::Mul),
	('/', Token::Div),
	('_', Token::Negate),
	('&', Token::BitAnd),
	('|', Token::BitOr),
	('~', Token::BitNot),

	('>', Token::GreaterThan),
	('=', Token::Equal),
	('<', Token::LessThan),

	('[', Token::LambdaStart),
	(']', Token::LambdaEnd),
	('!', Token::LambdaExecute),
	('?', Token::LambdaIf),
	('#', Token::LambdaWhile),

	('a', Token::Variable('a')),
	('b', Token::Variable('b')),
	('c', Token::Variable('c')),
	('d', Token::Variable('d')),
	('e', Token::Variable('e')),
	('f', Token::Variable('f')),
	('g', Token::Variable('g')),
	('h', Token::Variable('h')),
	('i', Token::Variable('i')),
	('j', Token::Variable('j')),
	('k', Token::Variable('k')),
	('l', Token::Variable('l')),
	('m', Token::Variable('m')),
	('n', Token::Variable('n')),
	('o', Token::Variable('o')),
	('p', Token::Variable('p')),
	('q', Token::Variable('q')),
	('r', Token::Variable('r')),
	('s', Token::Variable('s')),
	('t', Token::Variable('t')),
	('u', Token::Variable('u')),
	('v', Token::Variable('v')),
	('w', Token::Variable('w')),
	('x', Token::Variable('x')),
	('y', Token::Variable('y')),
	('z', Token::Variable('z')),  // :)
	(':', Token::VarWrite),
	(';', Token::VarRead),

	('^', Token::ReadChar),
	(',', Token::WriteChar),
	('.', Token::WriteInt),
	('ß', Token::FlushIO),
	('B', Token::FlushIO),
];

impl Tokenizer {
	pub fn new(code: &str) -> Tokenizer {
		Tokenizer {
			code: code.to_string(),
		}
	}

	pub fn skip_whitespace(&mut self) {
		match self.code.chars().next() {
			None => return,
			Some(c) => {
				if c.is_whitespace() {
					self.code = self.code[1..].to_string();
					self.skip_whitespace();
				}
			}
		}
	}

	pub fn next_token(&mut self) -> Option<Token> {
		self.skip_whitespace();

		let c = match self.code.chars().next() {
			None => return None,
			Some(x) => x,
		};

		for (token_char, token) in SIMPLE_TOKENS.iter() {
			if c == *token_char {
				self.code = self.code.chars().skip(1).collect();
				return Some(token.clone())
			}
		}

		match c {
			'{' => {
				let end = self.code.find('}').expect("parser error");
				self.code = self.code[end+1..].to_string();
				return self.next_token()
			}
			'"' => {
				self.code = self.code[1..].to_string();
				let end = self.code.find('"').expect("parser error");
				let token = Token::PrintString(self.code[0..end].to_string());
				self.code = self.code[end+1..].to_string();
				return Some(token)
			}
			'\'' => {
				let charcode = self.code.chars().nth(1).expect("parser error");
				self.code = self.code[2..].to_string();
				return Some(Token::Number(charcode as i64))
			}
			_ => {
				let end = self.code.find(|c: char| !c.is_digit(10)).unwrap_or(self.code.len());
				let token = self.code[..end].parse().expect("Invalid token");
				self.code = self.code[end..].to_string();
				return Some(Token::Number(token))
			}
		}
	}

	pub fn all(&mut self) -> Vec<Token> {
		let mut tokens = Vec::new();
		while let Some(token) = self.next_token() {
			tokens.push(token);
		}
		tokens
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_next_token() {
		let mut parser = Tokenizer::new("1+2-3* 4    / 5     ");

		assert_eq!(parser.next_token(), Some(Token::Number(1)));
		assert_eq!(parser.next_token(), Some(Token::Plus));
		assert_eq!(parser.next_token(), Some(Token::Number(2)));
		assert_eq!(parser.next_token(), Some(Token::Minus));
		assert_eq!(parser.next_token(), Some(Token::Number(3)));
		assert_eq!(parser.next_token(), Some(Token::Mul));
		assert_eq!(parser.next_token(), Some(Token::Number(4)));
		assert_eq!(parser.next_token(), Some(Token::Div));
		assert_eq!(parser.next_token(), Some(Token::Number(5)));
		assert_eq!(parser.next_token(), None);
	}

	#[test]
	fn test_1() {
		let mut parser = Tokenizer::new("1+2-3* 4    / 5     ");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Number(1),
			Token::Plus,
			Token::Number(2),
			Token::Minus,
			Token::Number(3),
			Token::Mul,
			Token::Number(4),
			Token::Div,
			Token::Number(5),
		]);
	}

	#[test]
	fn test_2() {
		let mut parser = Tokenizer::new("'a'b     'c    'd    266667");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Number(97),
			Token::Number(98),
			Token::Number(99),
			Token::Number(100),
			Token::Number(266667),
		]);
	}

	#[test]
	fn test_3() {
		let mut parser = Tokenizer::new("$ % \\ @ ø P");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Dup,
			Token::Drop,
			Token::Swap,
			Token::Rot,
			Token::Pick,
			Token::Pick,
		]);
	}

	#[test]
	fn test_4() {
		let mut parser = Tokenizer::new("+ - * / _ & | ~");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Plus,
			Token::Minus,
			Token::Mul,
			Token::Div,
			Token::Negate,
			Token::BitAnd,
			Token::BitOr,
			Token::BitNot,
		]);
	}

	#[test]
	fn test_5() {
		let mut parser = Tokenizer::new("> = <");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::GreaterThan,
			Token::Equal,
			Token::LessThan,
		]);
	}

	#[test]
	fn test_6() {
		let mut parser = Tokenizer::new("[]!?#");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::LambdaStart,
			Token::LambdaEnd,
			Token::LambdaExecute,
			Token::LambdaIf,
			Token::LambdaWhile,
		]);
	}

	#[test]
	fn test_7() {
		let mut parser = Tokenizer::new("a:b;z");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Variable('a'),
			Token::VarWrite,
			Token::Variable('b'),
			Token::VarRead,
			Token::Variable('z'),
		]);
	}

	#[test]
	fn test_8() {
		let mut parser = Tokenizer::new("a:b;z");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Variable('a'),
			Token::VarWrite,
			Token::Variable('b'),
			Token::VarRead,
			Token::Variable('z'),
		]);
	}

	#[test]
	fn test_9() {
		let mut parser = Tokenizer::new("^,.ßB");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::ReadChar,
			Token::WriteChar,
			Token::WriteInt,
			Token::FlushIO,
			Token::FlushIO,
		]);
	}

	#[test]
	fn test_10() {
		let mut parser = Tokenizer::new("\"hello\"        \"world\"");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::PrintString("hello".to_string()),
			Token::PrintString("world".to_string()),
		]);
	}

	#[test]
	fn test_11() {
		let mut parser = Tokenizer::new("1{wow}2{cool}3");
		let tokens = parser.all();

		assert_eq!(tokens, vec![
			Token::Number(1),
			Token::Number(2),
			Token::Number(3),
		]);
	}
}
