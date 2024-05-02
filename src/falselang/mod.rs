pub mod parser;
pub mod vm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
	Number(i64),

	Dup,
	Drop,
	Swap,
	Rot,
	Pick,

	Plus,
	Minus,
	Mul,
	Div,
	Negate,
	BitAnd,
	BitOr,
	BitNot,
}
