pub mod tokenizer;
pub mod vm;

#[derive(Debug, Clone, PartialEq, Eq)]
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

	GreaterThan,
	Equal,
	LessThan,

	LambdaStart,
	LambdaEnd,
	LambdaExecute,
	LambdaIf,
	LambdaWhile,

	Variable(char),
	VarWrite,
	VarRead,

	ReadChar,
	WriteChar,
	PrintString(String),
	WriteInt,
	FlushIO,
}
