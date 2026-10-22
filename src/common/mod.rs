use std::collections::HashMap;


#[derive(Clone, Copy)]
pub enum NumberSystem {
    Decimal = 10,
    Hexadecimal = 16,
    Octal = 8,
    Binary = 2,
    NotANumber = 0
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
    BitRightShift
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
    String
}
pub enum ScopeSymbol{
    Indent,
    OpenClose(String, String)
}
#[derive(Clone)]
pub struct Token{
    pub typ: TokenType,
    pub text: String,
    pub read: ReadMode,
}
pub struct CompilerConfig{
    pub normal_chars: Vec<char>,
    pub syntax_chars: Vec<char>,
    pub number_chars: Vec<char>,
    pub extended_number_chars: Vec<char>,
    pub always_singular: Vec<char>,
    pub scope: ScopeSymbol,
    pub arguments: [String; 2],
    pub type_arguments: [String; 2],
    pub array_arguments: [String; 2],
    pub field_access: char,
    pub type_specifier: char,
    pub arg_seperator: char,
    pub assignment: char,
    pub end_statement: char,
    pub splitter: char,
    pub number_values: HashMap<char, u8>,
    pub constants: HashMap<String, Constant>,
    pub operations: HashMap<String, Operation>,
    pub boolean_operations: HashMap<String, BooleanOperation>,
    pub comparisons: HashMap<String, Comparison>,
    pub keywords: HashMap<String, Keyword>,
    pub types: HashMap<String, Type>,
    pub number_suffixes: HashMap<String, Type>,
    pub number_systems: HashMap<String, NumberSystem>,
    pub ambiguous: HashMap<TokenType, TokenType>,
    pub string_starters: Vec<String>,
    pub string_enders: Vec<String>,
    pub illegal_combinations: Vec<String>,
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
            always_singular: "#()[]{}~?,;:\r\n".chars().collect(),
            syntax_chars: "!@#$%^&*{}()[]<>.,/\\\"':;`~?=+-".chars().collect(),
            number_chars: "0123456789_.".chars().collect(),
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
            extended_number_chars: "abcdefABCDEF".chars().collect(),
            field_access: '.',
            type_specifier: ':',
            arg_seperator: ',',
            assignment: '=',
            scope: ScopeSymbol::OpenClose("{".to_string(),"}".to_string()),
            arguments: ["(".to_string(),")".to_string()],
            type_arguments: ["<".to_string(),">".to_string()],
            array_arguments: ["[".to_string(),"]".to_string()],
            end_statement: ';',
            splitter: ' ',
            constants: HashMap::from([
                ("true".to_string(),Constant::True),
                ("false".to_string(),Constant::False),
                ("null".to_string(),Constant::Null),
            ]),
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
            boolean_operations: HashMap::from([
                ("&&".to_string(),BooleanOperation::And),
                ("||".to_string(),BooleanOperation::Or),
                ("!".to_string(),BooleanOperation::Not),
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
                ("class".to_string(),Keyword::Class),
                ("enum".to_string(),Keyword::Enum),
                ("interface".to_string(),Keyword::Interface),
                ("new".to_string(),Keyword::New),
                ("return".to_string(),Keyword::Return),
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
                ("boolean".to_string(),Type::Boolean),
                ("String".to_string(),Type::String),
                ("function".to_string(),Type::Function),
                ("let".to_string(),Type::Variable),
                ("const".to_string(),Type::Constant),
            ]),
            ambiguous: HashMap::from([
                (TokenType::Comparison(Comparison::LessThan),TokenType::Keyword(Keyword::TypeArgumentStart)),
                (TokenType::Comparison(Comparison::GreaterThan),TokenType::Keyword(Keyword::TypeArgumentEnd)),
                (TokenType::Operation(Operation::BitRightShift),TokenType::Multiple(vec![TokenType::Keyword(Keyword::TypeArgumentEnd),TokenType::Keyword(Keyword::TypeArgumentEnd)])),
            ]),
            string_starters: vec!["\"".to_string(),"'".to_string()],
            string_enders: vec!["\"".to_string(),"'".to_string()],
            illegal_combinations: vec![
                "=-".to_string(),
                "=+".to_string(),
                "&&!".to_string(),
                "||!".to_string(),
                "=\"".to_string(),
                ").".to_string(),
                "].".to_string(),
                ">.".to_string(),
                "\".".to_string(),
            ]
        }
    }
}