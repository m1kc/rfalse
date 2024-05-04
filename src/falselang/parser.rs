use super::tokenizer::{Token, Tokenizer};

pub struct Parser {
	pub tokenizer: Tokenizer,
	pub lambda_storage: Vec<Vec<Token>>,
}

impl Parser {
	pub fn new(t: Tokenizer) -> Parser {
		Parser {
			tokenizer: t,
			lambda_storage: Vec::new(),
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
				self.lambda_storage.push(lambda);
				tokens.push(Token::LambdaPointer(self.lambda_storage.len() - 1));
				continue;
			}
			tokens.push(t);
		}
		tokens
	}

	pub fn parse(&mut self) -> Token {
		let tokens = self.read_lambda();
		self.lambda_storage.push(tokens);
		Token::LambdaPointer(self.lambda_storage.len() - 1)
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_next() {
		let mut parser = Parser::new(Tokenizer::new("[2 2+]"));
		assert_eq!(parser.parse(), Token::LambdaPointer(1));
		assert_eq!(parser.lambda_storage, vec![
			vec![Token::Number(2), Token::Number(2), Token::Plus],
			vec![Token::LambdaPointer(0)],
		]);
	}

	#[test]
	fn test_empty() {
		let mut parser = Parser::new(Tokenizer::new(""));
		assert_eq!(parser.parse(), Token::LambdaPointer(0));
		assert_eq!(parser.lambda_storage, vec![
			vec![],
		]);
	}
}
