use std::cmp::{min, PartialEq};
use crate::common::*;

mod parser {}

fn syntax_token_type(token: &String, config: &CompilerConfig) -> TokenType {
    if &config.field_access.to_string() == token{
        return TokenType::Keyword(Keyword::FieldAccess);
    }

    if &config.assignment.to_string() == token{
        return TokenType::Keyword(Keyword::Assignment);
    }

    if let ScopeSymbol::OpenClose(a, b) = &config.scope {
        if (token == a) {
            return TokenType::Keyword(Keyword::ScopeEnter);
        }
        if (token == b) {
            return TokenType::Keyword(Keyword::ScopeExit);
        }
    }

    if (token.eq(&config.end_statement.to_string())) {
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
    let pot_suffix = any_suffix(token, Vec::from_iter(config.number_suffixes.keys()));
    let pot_prefix = any_prefix(token, Vec::from_iter(config.number_systems.keys()));

    let mut suffix: String = "".to_string();
    let mut prefix: String = "".to_string();

    if pot_prefix.is_some(){ prefix = pot_prefix.unwrap(); }
    if pot_suffix.is_some(){ suffix = pot_suffix.unwrap(); }

    let minimum = suffix.len() + prefix.len();

    if (token.len() <= minimum) {
        return false;
    }

    let new_token = String::from(&token[prefix.len()..token.len()-suffix.len()]);

    let mut out = true;
    for c in new_token.chars() {
        out &= config.number_chars.contains(&c);
    }
    println!("{} is {}", token, out);
    out
}
fn reading_mode_of(character: char, current_string: &String, config: &CompilerConfig) -> ReadMode{
    let number_test = current_string.to_owned() + character.to_string().as_str();
    if is_number(&number_test, config){
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
    let mut last_char: char = config.end_statement;
    for c in code.chars(){

        if let ReadMode::String = &read_mode{
            //...
        } else {

            if (c == config.splitter){
                emit(&mut tokens, &mut current_string, &read_mode, config);
                continue;
            }
            let rm_symbol = reading_mode_of(c,&current_string , config);
            println!("{} is of readmode {:?}", c, rm_symbol);
            if rm_symbol != read_mode{
                emit(&mut tokens, &mut current_string, &read_mode, config);
                read_mode = rm_symbol;
            }
            // if let ReadMode::Syntax = read_mode {
            //     let potential: TokenType = token_type(&current_string, &read_mode, config);
            //     if TokenType::Unknown != potential {
            //         emit(&mut tokens, &mut current_string, &read_mode, config);
            //     }
            // }
            if (config.always_singular.contains(&c)){
                emit(&mut tokens, &mut current_string, &read_mode, config);
                let mut temp = c.to_string();

                emit(&mut tokens, &mut temp, &ReadMode::Syntax, config);
                continue;
            }

        }
        last_char = c;
        current_string.push(c);
    }
    let rm_symbol = reading_mode_of(last_char,&current_string , config);
    emit(&mut tokens, &mut current_string, &rm_symbol, config);

    tokens
}