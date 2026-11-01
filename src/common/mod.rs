use std::collections::HashMap;
use colored::Colorize;
pub type Base = i64;

impl Token{
    pub fn new(typ: &TokenType, text: &String, index: usize) -> Token{
        Token{typ: typ.clone(), text: text.clone(), index}
    }
    pub fn from_type(typ: &TokenType, index: usize) -> Token{
        Token{typ: typ.clone(), text: "".to_string(), index}
    }
    pub fn from_string(text: &String, index: usize) -> Token{
        Token{typ: TokenType::Unknown, text: text.clone(), index}
    }
    pub fn with_type(mut self: Self, typ: TokenType) -> Token{
        self.typ = typ;
        self
    }
}
pub enum FunctionDecl{
    Keyword(String),
    Type,
}
#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum Comparison {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum Constant{
    True,
    False,
    Null
}
#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum Operation{
    Add,
    Subtract,
    Multiply,
    Power,
    Root,
    Divide,
    IntDivide,
    Modulo,
    BitXor,
    BitAnd,
    BitOr,
    BitLeftShift,
    BitRightShift,
    BitFlip,
}
#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum BooleanOperation{
    And,
    Or,
    Xor,
    Not,
    Nand
}
#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum Keyword{
    If,
    Else,
    ElseIf,
    While,
    New,
    Return,
    For,
    Class,
    Enum,
    Interface,
    ScopeEnter,
    ScopeExit,
    TypeArgumentStart,
    TypeArgumentEnd,
    ArgumentStart,
    ArgumentEnd,
    ArrayStart,
    ArrayEnd,
    EndStatement,
    FieldAccess,
    ArgumentSeperator,
    TypeSpecifier,
    Assignment,
    OperationAssignment(Operation),
    BooleanOperationAssignment(BooleanOperation),
    Arrow,
    FatArrow,

    This,
}
#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum Type{
    Void,
    Boolean,
    Byte,
    Short,
    Char,
    Integer,
    Long,
    Float,
    Double,
    String,
    Generic,
    Function,
    Variable,
    Constant,
    Custom(String),
    Array(Box<Type>, u8),
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum TokenType{
    EndOfFile,
    Unknown,
    Comparison(Comparison),
    Operation(Operation),
    Keyword(Keyword),
    Type(Type),
    BooleanOperation(BooleanOperation),
    Constant(Constant),
    Identifier,
    TypeIdentifier,
    FuncIdentifier,
    Operand,
    RawString(usize),
    Ambiguous(Box<TokenType>,Box<TokenType>),
    Multiple(Vec<TokenType>),
}


#[derive(Clone, Debug, PartialEq)]
pub enum ReadMode{
    Normal,
    Syntax,
    String(String, usize),
}
pub enum ScopeSymbol{
    Indent(char),
    OpenClose([char; 2])
}
#[derive(Clone, Debug)]
pub struct Token{
    pub typ: TokenType,
    pub text: String,
    pub index: usize
}
pub struct CompilerConfig{
    pub normal_chars: Vec<char>,
    pub syntax_chars: Vec<char>,
    pub scope: ScopeSymbol,
    pub decimal: char,
    pub splitter: char,
    pub number_values: HashMap<char, u8>,
    pub keywords: HashMap<String, TokenType>,
    pub number_suffixes: HashMap<String, Type>,
    pub number_systems: HashMap<String, Base>,
    pub string_starters: Vec<String>,
    pub string_enders: Vec<String>,
    pub ambiguous_debug: HashMap<String, String>
}

impl CompilerConfig {
    pub fn default() -> CompilerConfig {
        CompilerConfig {
            number_suffixes: HashMap::from([
                ("b".to_string(), Type::Byte),
                ("s".to_string(), Type::Short),
                ("l".to_string(), Type::Long),
                ("f".to_string(), Type::Float),
                ("d".to_string(), Type::Double),
                ("B".to_string(), Type::Byte),
                ("S".to_string(), Type::Short),
                ("L".to_string(), Type::Long),
                ("F".to_string(), Type::Float),
                ("D".to_string(), Type::Double),
            ]),
            number_systems: HashMap::from([
                ("0x".to_string(), 16),
                ("0b".to_string(), 2),
                ("0o".to_string(), 8),
            ]),
            normal_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_\"'`".chars().collect(),
            syntax_chars: "!@#$%^&|*{}()[]<>.,/\\\x04:;~?=+-".chars().collect(),
            number_values: HashMap::from([
                ('0', 0),
                ('1', 1),
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('A', 10),
                ('B', 11),
                ('C', 12),
                ('D', 13),
                ('E', 14),
                ('F', 15),
                ('a', 10),
                ('b', 11),
                ('c', 12),
                ('d', 13),
                ('e', 14),
                ('f', 15),
            ]),
            decimal: '.',
            scope: ScopeSymbol::OpenClose(['{','}']),
            splitter: ' ',
            keywords: HashMap::from([
                //constants
                ("true".to_string(), TokenType::Constant(Constant::True)),
                ("false".to_string(), TokenType::Constant(Constant::False)),
                ("null".to_string(), TokenType::Constant(Constant::Null)),
                ("this".to_string(), TokenType::Keyword(Keyword::This)),
                ("=".to_string(), TokenType::Keyword(Keyword::Assignment)),
                (",".to_string(), TokenType::Keyword(Keyword::ArgumentSeperator)),
                (".".to_string(), TokenType::Keyword(Keyword::FieldAccess)),
                (":".to_string(), TokenType::Keyword(Keyword::TypeSpecifier)),
                ("[".to_string(), TokenType::Keyword(Keyword::ArrayStart)),
                ("]".to_string(), TokenType::Keyword(Keyword::ArrayEnd)),
                ("(".to_string(), TokenType::Keyword(Keyword::ArgumentStart)),
                (")".to_string(), TokenType::Keyword(Keyword::ArgumentEnd)),
                (";".to_string(), TokenType::Keyword(Keyword::EndStatement)),
                ("+=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::Add))),
                ("-=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::Subtract))),
                ("/=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::Divide))),
                ("*=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::Multiply))),
                ("%=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::Modulo))),
                ("**=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::Power))),
                ("^=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::BitXor))),
                ("|=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::BitOr))),
                ("&=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::BitAnd))),
                ("<<=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::BitLeftShift))),
                (">>=".to_string(), TokenType::Keyword(Keyword::OperationAssignment(Operation::BitRightShift))),
                //bool ops and comparisons
                ("&&=".to_string(), TokenType::Keyword(Keyword::BooleanOperationAssignment(BooleanOperation::And))),
                ("||=".to_string(), TokenType::Keyword(Keyword::BooleanOperationAssignment(BooleanOperation::Or))),
                //operators
                ("+".to_string(), TokenType::Operation(Operation::Add)),
                ("~".to_string(), TokenType::Operation(Operation::BitFlip)),
                ("-".to_string(), TokenType::Operation(Operation::Subtract)),
                ("/".to_string(), TokenType::Operation(Operation::Divide)),
                ("*".to_string(), TokenType::Operation(Operation::Multiply)),
                ("%".to_string(), TokenType::Operation(Operation::Modulo)),
                ("**".to_string(), TokenType::Operation(Operation::Power)),
                ("^".to_string(), TokenType::Operation(Operation::BitXor)),
                ("|".to_string(), TokenType::Operation(Operation::BitOr)),
                ("&".to_string(), TokenType::Operation(Operation::BitAnd)),
                ("<<".to_string(), TokenType::Operation(Operation::BitLeftShift)),
                (">>".to_string(), TokenType::Ambiguous(
                    Box::from(TokenType::Operation(Operation::BitRightShift)),
                    Box::from(TokenType::Multiple(vec![
                        TokenType::Keyword(Keyword::TypeArgumentEnd),
                        TokenType::Keyword(Keyword::TypeArgumentEnd)
                    ])))
                ),
                //bool ops and comparisons
                ("&&".to_string(), TokenType::BooleanOperation(BooleanOperation::And)),
                ("||".to_string(), TokenType::BooleanOperation(BooleanOperation::Or)),
                ("!".to_string(), TokenType::BooleanOperation(BooleanOperation::Not)),
                (">".to_string(), TokenType::Ambiguous(
                    Box::from(TokenType::Comparison(Comparison::GreaterThan)),
                    Box::from(TokenType::Keyword(Keyword::TypeArgumentEnd)))
                ),
                ("<".to_string(), TokenType::Ambiguous(
                    Box::from(TokenType::Comparison(Comparison::LessThan)),
                    Box::from(TokenType::Keyword(Keyword::TypeArgumentStart)))
                ),
                ("<=".to_string(), TokenType::Comparison(Comparison::LessThanOrEqual)),
                (">=".to_string(), TokenType::Comparison(Comparison::GreaterThanOrEqual)),
                ("==".to_string(), TokenType::Comparison(Comparison::Equals)),
                ("!=".to_string(), TokenType::Comparison(Comparison::NotEquals)),
                // keywords
                ("if".to_string(), TokenType::Keyword(Keyword::If)),
                ("else".to_string(), TokenType::Keyword(Keyword::Else)),
                ("while".to_string(), TokenType::Keyword(Keyword::While)),
                ("for".to_string(), TokenType::Keyword(Keyword::For)),
                ("class".to_string(), TokenType::Keyword(Keyword::Class)),
                ("enum".to_string(), TokenType::Keyword(Keyword::Enum)),
                ("interface".to_string(), TokenType::Keyword(Keyword::Interface)),
                ("new".to_string(), TokenType::Keyword(Keyword::New)),
                ("return".to_string(), TokenType::Keyword(Keyword::Return)),
                ("->".to_string(), TokenType::Keyword(Keyword::Arrow)),
                ("=>".to_string(), TokenType::Keyword(Keyword::FatArrow)),
                ("void".to_string(), TokenType::Type(Type::Void)),
                ("int".to_string(), TokenType::Type(Type::Integer)),
                ("byte".to_string(), TokenType::Type(Type::Byte)),
                ("short".to_string(), TokenType::Type(Type::Short)),
                ("char".to_string(), TokenType::Type(Type::Char)),
                ("long".to_string(), TokenType::Type(Type::Long)),
                ("float".to_string(), TokenType::Type(Type::Float)),
                ("double".to_string(), TokenType::Type(Type::Double)),
                ("boolean".to_string(), TokenType::Type(Type::Boolean)),
                ("String".to_string(), TokenType::Type(Type::String)),
                ("function".to_string(), TokenType::Type(Type::Function)),
                ("let".to_string(), TokenType::Type(Type::Variable)),
                ("const".to_string(), TokenType::Type(Type::Constant)),
            ]),
            ambiguous_debug: HashMap::from([
                (">>".to_string(),
                 {
                     let symbol = ">>".red();
                     format!("\">>\" could be either a right bit shift (16 {symbol} 2) or two closing angle brackets (<A<B{symbol})").to_string()
                 })
            ]),
            string_starters: vec!["\"".to_string(), "'".to_string()],
            string_enders: vec!["\"".to_string(), "'".to_string()],
        }
    }
}