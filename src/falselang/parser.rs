use super::Token;

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
/// - `ø` or `p` PICK (dup the nth stack item). `p` is an extension
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
pub struct Parser {
	code: String,
}

impl Parser {
	pub fn new(code: &str) -> Parser {
		Parser {
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

		match c {
			'$' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Dup)
			}
			'%' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Drop)
			}
			'\\' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Swap)
			}
			'@' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Rot)
			}
			'ø' | 'p' => {
				self.code = self.code.chars().skip(1).collect();
				return Some(Token::Pick)
			}

			'+' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Plus)
			}
			'-' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Minus)
			}
			'*' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Mul)
			}
			'/' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Div)
			}
			'_' => {
				self.code = self.code[1..].to_string();
				return Some(Token::Negate)
			}
			'&' => {
				self.code = self.code[1..].to_string();
				return Some(Token::BitAnd)
			}
			'|' => {
				self.code = self.code[1..].to_string();
				return Some(Token::BitOr)
			}
			'~' => {
				self.code = self.code[1..].to_string();
				return Some(Token::BitNot)
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

	pub fn parse_all(&mut self) -> Vec<Token> {
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
		let mut parser = Parser::new("1+2-3* 4    / 5     ");

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
	fn test_parse_all() {
		let mut parser = Parser::new("1+2-3* 4    / 5     ");
		let tokens = parser.parse_all();

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
