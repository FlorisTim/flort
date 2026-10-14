use std::collections::HashMap;

#[derive(Clone)]
pub enum NumberSystem {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
    NotANumber,
}
#[derive(Clone, PartialEq, Debug)]
pub enum Comparison {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}
#[derive(Clone, PartialEq, Debug)]
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
}
#[derive(Clone, PartialEq, Debug)]
pub enum Keyword{
    If,
    Else,
    ElseIf,
    While,
    For,
    Variable,
    Constant,
    Function,
    Class,
    Enum,
    Interface,
    ScopeEnter,
    ScopeExit,
    ArgumentStart,
    ArgumentEnd,
    EndStatement,
    FieldAccess,
    Assignment,
}
#[derive(Clone, PartialEq, Debug)]
pub enum Type{
    Void,
    Byte,
    Short,
    Char,
    Integer,
    Long,
    Float,
    Double,
    String,
}
#[derive(Clone, PartialEq, Debug)]
pub enum TokenType{
    Unknown,
    Comparison(Comparison),
    Operation(Operation),
    Keyword(Keyword),
    Type(Type),
    Identifier,
    Operand,
}
#[derive(PartialEq, Clone, Debug)]
pub enum ReadMode{
    Normal,
    Syntax,
    String
}
pub enum ScopeSymbol{
    Indent,
    OpenClose(String, String)
}
pub struct Token{
    pub typ: TokenType,
    pub text: String,
    pub read: ReadMode,
}
pub struct CompilerConfig{
    pub normal_chars: Vec<char>,
    pub syntax_chars: Vec<char>,
    pub number_chars: Vec<char>,
    pub always_singular: Vec<char>,
    pub scope: ScopeSymbol,
    pub arguments: [String; 2],
    pub type_arguments: [String; 2],
    pub array_arguments: [String; 2],
    pub field_access: char,
    pub assignment: char,
    pub end_statement: char,
    pub splitter: char,
    pub operations: HashMap<String, Operation>,
    pub comparisons: HashMap<String, Comparison>,
    pub keywords: HashMap<String, Keyword>,
    pub types: HashMap<String, Type>,
    pub number_suffixes: HashMap<String, Type>,
    pub number_systems: HashMap<String, NumberSystem>,
}

impl CompilerConfig{
    pub fn default() -> CompilerConfig{
        CompilerConfig{
            number_suffixes: HashMap::from([
                ("b".to_string(),Type::Byte),
                ("s".to_string(),Type::Short),
                ("l".to_string(),Type::Long),
                ("f".to_string(),Type::Float),
                ("d".to_string(),Type::Double),
            ]),
            number_systems: HashMap::from([
                ("0x".to_string(),NumberSystem::Hexadecimal),
                ("0b".to_string(),NumberSystem::Binary),
            ]),
            normal_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_".chars().collect(),
            always_singular: "#()[]{}~?".chars().collect(),
            syntax_chars: "!@#$%^&*{}()[]<>.,/\\\"':;`~?=+-".chars().collect(),
            number_chars: "0123456789.".chars().collect(),
            field_access: '.',
            assignment: '=',
            scope: ScopeSymbol::OpenClose("{".to_string(),"}".to_string()),
            arguments: ["(".to_string(),")".to_string()],
            type_arguments: ["<".to_string(),">".to_string()],
            array_arguments: ["[".to_string(),"]".to_string()],
            end_statement: ';',
            splitter: ' ',
            operations: HashMap::from([
                ("+".to_string(),Operation::Add),
                ("-".to_string(),Operation::Subtract),
                ("/".to_string(),Operation::Divide),
                ("*".to_string(),Operation::Multiply),
                ("%".to_string(),Operation::Modulo),
                ("**".to_string(),Operation::Power),
                ("^".to_string(),Operation::BitXor),
                ("|".to_string(),Operation::BitOr),
                ("&".to_string(),Operation::BitAnd),
                ("<<".to_string(),Operation::BitLeftShift),
                (">>".to_string(),Operation::BitRightShift),
            ]),
            comparisons: HashMap::from([
                (">".to_string(),Comparison::GreaterThan),
                ("<".to_string(),Comparison::LessThan),
                ("<=".to_string(),Comparison::LessThanOrEqual),
                (">=".to_string(),Comparison::GreaterThanOrEqual),
                ("==".to_string(),Comparison::Equals),
                ("!=".to_string(),Comparison::NotEquals),
            ]),
            keywords: HashMap::from([
                ("if".to_string(),Keyword::If),
                ("else".to_string(),Keyword::Else),
                ("while".to_string(),Keyword::While),
                ("for".to_string(),Keyword::For),
                ("let".to_string(),Keyword::Variable),
                ("const".to_string(),Keyword::Constant),
                ("func".to_string(),Keyword::Function),
                ("class".to_string(),Keyword::Class),
                ("enum".to_string(),Keyword::Enum),
                ("interface".to_string(),Keyword::Interface),
            ]),
            types: HashMap::from([
                ("void".to_string(),Type::Void),
                ("int".to_string(),Type::Integer),
                ("byte".to_string(),Type::Byte),
                ("short".to_string(),Type::Short),
                ("char".to_string(),Type::Char),
                ("long".to_string(),Type::Long),
                ("float".to_string(),Type::Float),
                ("double".to_string(),Type::Double),
                ("string".to_string(),Type::String),
            ]),
        }
    }
}