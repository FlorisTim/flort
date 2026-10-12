use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Clone)]
enum NumberSystem {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
    NotANumber,
}

#[derive(Clone, PartialEq)]
enum Comparison {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}
#[derive(Clone, PartialEq)]
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
#[derive(Clone, PartialEq)]
enum Keyword{
    If,
    Else,
    ElseIf,
    While,
    Variable,
    Constant,
    Function,
    Class,
    Enum,
    Interface,
    ScopeEnter,
    ScopeExit
}
#[derive(Clone, PartialEq)]
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
#[derive(Clone, PartialEq)]
enum TokenType{
    Unknown,
    Comparison(Comparison),
    Operation(Operation),
    Keyword(Keyword),
    Type(Type),
    Identifier,
    Operand,
}
#[derive(PartialEq)]
enum ReadMode{
    Normal,
    Syntax,
    String
}
enum ScopeSymbol{
    Indent,
    OpenClose(String, String)
}

impl CompilerConfig{
    //todo make default
}
struct Token{
    typ: TokenType,
    text: String,
}
fn parse_token(token: &str) -> Token{
    let mut out = Token{
        typ: TokenType::Unknown,
        text: token.to_string()
    };

    out
}

struct CompilerConfig{
    normal_chars: Vec<char>,
    syntax_chars: Vec<char>,
    number_chars: Vec<char>,
    scope: ScopeSymbol,
    splitter: char,
    operations: HashMap<String, Operation>,
    comparisons: HashMap<String, Comparison>,
    keywords: HashMap<String, Keyword>,
    types: HashMap<String, Type>,
    number_suffixes: HashMap<String, Type>,
    number_systems: HashMap<String, NumberSystem>,
}
fn syntax_token_type(token: &String, config: &CompilerConfig) -> TokenType{
    let potential_op = config.operations.get(token);
    let potential_co = config.comparisons.get(token);
    if potential_op.is_some(){
        TokenType::Operation(potential_op.unwrap().clone())
    } else if potential_co.is_some(){
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
    let prefix = any_prefix(token, Vec::from_iter(config.number_suffixes.keys()));
     suffix.is_some()|| prefix.is_some()
         &&
            {
                let mut out = true;
                for c in token.chars(){
                    out &= config.number_chars.contains(&c);
                }
                out
            }
}
fn reading_mode_of(character: char,current_string: &String , config: &CompilerConfig) -> ReadMode{
    if is_number(current_string, config){
        return ReadMode::Normal;
    }
    if config.syntax_chars.contains(&character){ReadMode::Syntax} else {ReadMode::Normal}
}

fn get_tokens(code: &str, config: &CompilerConfig) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_string: String = String::new();
    let mut read_mode: ReadMode = ReadMode::Normal;

    for c in code.chars(){
        if let ReadMode::String = read_mode{
            //...
        } else {
            let rm_symbol = reading_mode_of(c,&current_string , config);
            if rm_symbol != read_mode{
                read_mode = rm_symbol;

                tokens.push(parse_token(&current_string));
                current_string = String::new();
            }
            if c == config.splitter && !current_string.is_empty(){

                tokens.push(parse_token(&current_string));
                current_string = String::new();

                continue;
            }

            if read_mode == ReadMode::Normal{
                let test = normal_token_type(&current_string, config);
                if test != TokenType::Unknown{
                    tokens.push(parse_token(&current_string));
                    current_string = String::new();
                }
            }

            if read_mode == ReadMode::Syntax{
                let test = syntax_token_type(&current_string, config);
                if test != TokenType::Unknown{
                    tokens.push(parse_token(&current_string));
                    current_string = String::new();
                }
            }

        }
        current_string.push(c);
    }

    tokens
}