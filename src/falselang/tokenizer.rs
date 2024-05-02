use super::Token;

const SIMPLE_TOKENS: [(char, Token); 14] = [
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
];

/// Supported tokens:
///
/// Literals
/// - `123` put integer onto the stack
/// - `'c` put character code onto the stack
///
/// Stack operations
/// - `$` DUP
/// - `%` DROP
/// - `\` SWAP
/// - `@` ROT
/// - `ø` or `P` PICK (dup the nth stack item). **Note**: `P` is rfalse extension
///
/// Arithmetic
/// - `+`
/// - `-`
/// - `*`
/// - `/`
/// - `_` negate (negative numbers are entered "123_")
/// - `&` bitwise AND
/// - `|` bitwise OR
/// - `~` bitwise NOT
pub struct Tokenizer {
	code: String,
}

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

		for &(token_char, token) in SIMPLE_TOKENS.iter() {
			if c == token_char {
				self.code = self.code.chars().skip(1).collect();
				return Some(token)
			}
		}

		match c {
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
	fn test_all() {
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
}
