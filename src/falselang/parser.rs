use super::tokenizer::{Token, Tokenizer};

pub struct Parser {
	pub tokenizer: Tokenizer,
}

impl Parser {
	pub fn new(t: Tokenizer) -> Parser {
		Parser {
			tokenizer: t,
		}
	}

	fn read_lambda(&mut self) -> Vec<Token> {
		let mut tokens = Vec::new();
		while let Some(t) = self.tokenizer.next_token() {
			if t == Token::LambdaEnd {
				break;
			}
			if t == Token::LambdaStart {
				let lambda = self.read_lambda();
				tokens.push(Token::ParsedLambda(lambda));
				continue;
			}
			tokens.push(t);
		}
		tokens
	}

	pub fn next(&mut self) -> Option<Token> {
		let tok = self.tokenizer.next_token();
		match tok {
			Some(Token::LambdaStart) => {
				let tokens = self.read_lambda();
				Some(Token::ParsedLambda(tokens))
			}
			Some(t) => Some(t),
			None => None,
		}
	}

	pub fn all(&mut self) -> Vec<Token> {
		let mut tokens = Vec::new();
		while let Some(token) = self.next() {
			tokens.push(token);
		}
		tokens
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_next() {
		let mut parser = Parser::new(Tokenizer::new("[2 2+]"));
		assert_eq!(parser.all(), vec![
			Token::ParsedLambda(vec![
				Token::Number(2),
				Token::Number(2),
				Token::Plus,
			]),
		])
	}
}
