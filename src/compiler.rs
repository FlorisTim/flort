use std::cmp::{min, PartialEq};
use std::collections::HashMap;

enum MyOption<T>{
    Some(T),
    None,
}
enum ExpressionParsing{
    Pratt,
    LeftToRight,
}
#[derive(Clone)]
enum NumberSystem {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
    NotANumber,
}
#[derive(Clone, PartialEq, Debug)]
enum Comparison {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}
#[derive(Clone, PartialEq, Debug)]
enum Operation{
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
enum Keyword{
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
}
#[derive(Clone, PartialEq, Debug)]
enum Type{
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
enum ScopeSymbol{
    Indent,
    OpenClose(String, String)
}
pub struct Token{
    pub typ: TokenType,
    pub text: String,
    pub read: ReadMode,
}
pub struct CompilerConfig{
    normal_chars: Vec<char>,
    syntax_chars: Vec<char>,
    number_chars: Vec<char>,
    always_singular: Vec<char>,
    scope: ScopeSymbol,
    arguments: [String; 2],
    end_statement: String,
    splitter: char,
    operations: HashMap<String, Operation>,
    comparisons: HashMap<String, Comparison>,
    keywords: HashMap<String, Keyword>,
    types: HashMap<String, Type>,
    number_suffixes: HashMap<String, Type>,
    number_systems: HashMap<String, NumberSystem>,
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
            always_singular: "()[]{}~?".chars().collect(),
            syntax_chars: "!@#$%^&*{}()[]<>.,/\\\"':;`~?=+-".chars().collect(),
            number_chars: "0123456789.".chars().collect(),
            scope: ScopeSymbol::OpenClose("{".to_string(),"}".to_string()),
            arguments: ["(".to_string(),")".to_string()],
            end_statement: ";".to_string(),
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
fn syntax_token_type(token: &String, config: &CompilerConfig) -> TokenType {
    if let ScopeSymbol::OpenClose(a, b) = &config.scope {
        if (token == a) {
            return TokenType::Keyword(Keyword::ScopeEnter);
        }
        if (token == b) {
            return TokenType::Keyword(Keyword::ScopeExit);
        }
    }

    if (token.eq(&config.end_statement)) {
        return TokenType::Keyword(Keyword::EndStatement);
    }
    if(token.eq(&config.arguments[0])) {
        return TokenType::Keyword(Keyword::ArgumentStart);
    }
    if (token.eq(&config.arguments[1])) {
        return TokenType::Keyword(Keyword::ArgumentEnd);
    }

    let potential_op = config.operations.get(token);
    let potential_co = config.comparisons.get(token);
    if potential_op.is_some() {
        TokenType::Operation(potential_op.unwrap().clone())
    } else if potential_co.is_some() {
        TokenType::Comparison(potential_co.unwrap().clone())
    } else { TokenType::Unknown }
}
fn normal_token_type(token: &String, config: &CompilerConfig) -> TokenType{
    let potential_kw = config.keywords.get(token);
    let potential_ty = config.types.get(token);
    if potential_kw.is_some(){
        TokenType::Keyword(potential_kw.unwrap().clone())
    } else if potential_ty.is_some(){
        TokenType::Type(potential_ty.unwrap().clone())
    } else if is_number(token, config){
        TokenType::Operand
    } else {
        TokenType::Unknown
    }
}

fn token_type(current_string: &String,read_mode: &ReadMode, config: &CompilerConfig) -> TokenType{
    if let ReadMode::Syntax = read_mode{syntax_token_type(&current_string, config)} else {normal_token_type(&current_string, config)}
}
fn any_suffix(string: &String, values: Vec<&String>) -> Option<String>{
    for s in values{
        if (string.ends_with(s)){
            return Some(s.to_string());
        }
    }
    None
}

fn any_prefix(string: &String, values: Vec<&String>) ->  Option<String>{
    for s in values{
        if (string.starts_with(s)){
            return Some(s.to_string());
        }
    }
    None
}

fn is_number(token: &String, config: &CompilerConfig) -> bool {
    let suffix = any_suffix(token, Vec::from_iter(config.number_suffixes.keys()));
    let prefix = any_prefix(token, Vec::from_iter(config.number_systems.keys()));

    let minimum =
        if suffix.is_some() {
            suffix.unwrap().len()
        } else { 0 } + if prefix.is_some() {
            prefix.unwrap().len()
        } else { 0 };

    if (token.len() < minimum) {
        return false;
    }

    let mut out = true;
    for c in token.chars() {
        out &= config.number_chars.contains(&c);
    }
    out
}
fn reading_mode_of(character: char, current_string: &String, config: &CompilerConfig) -> ReadMode{
    if is_number(current_string, config){
        return ReadMode::Normal;
    }
    if config.syntax_chars.contains(&character){ReadMode::Syntax} else {ReadMode::Normal}
}


fn parse_token(token: &String, read_mode: &ReadMode, last_token: Option<&Token>, config: &CompilerConfig) -> Token{
    let mut out = Token{
        typ: TokenType::Unknown,
        text: token.to_owned(),
        read: read_mode.to_owned(),
    };

    if (read_mode == &ReadMode::Normal){
        out.typ = normal_token_type(token, config);
    }
    if (read_mode == &ReadMode::Syntax){
        out.typ = syntax_token_type(token, config);
    }

    out
}
fn emit(tokens: &mut Vec<Token>, current_string: &mut String, read_mode: &ReadMode, config: &CompilerConfig){
    if (current_string.is_empty()){
        return;
    }
    tokens.push(parse_token(&current_string, &read_mode, tokens.last(), config));
    current_string.clear();
}
pub fn get_tokens(code: &str, config: &CompilerConfig) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_string: String = String::new();
    let mut read_mode: ReadMode = ReadMode::Normal;

    for c in code.chars(){

        if let ReadMode::String = read_mode{
            //...
        } else {
            if (c == config.splitter){
                emit(&mut tokens, &mut current_string, &read_mode, config);
                continue;
            }
            let potential: TokenType = token_type(&current_string,&read_mode ,config);
            if TokenType::Unknown != potential{
                emit(&mut tokens, &mut current_string, &read_mode, config);
            }
            let rm_symbol = reading_mode_of(c,&current_string , config);
            if rm_symbol != read_mode{
                emit(&mut tokens, &mut current_string, &read_mode, config);
                read_mode = rm_symbol;
            }
            if (config.always_singular.contains(&c)){
                emit(&mut tokens, &mut current_string, &read_mode, config);
                emit(&mut tokens, &mut c.to_string(), &ReadMode::Syntax, config);
                continue;
            }
        }
        current_string.push(c);
    }
    emit(&mut tokens, &mut current_string, &read_mode, config);

    tokens
}