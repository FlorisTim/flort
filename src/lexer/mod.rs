use macroquad::logging::trace;
use crate::common::*;

pub fn get_tokens(input: &str, config: &CompilerConfig) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();

    let mut current_string: String = String::new();
    for c in input.chars() {

    }

    out
}

pub fn match_functions_class(input: &Vec<Token>, custom_types: Option<&Vec<String>>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in 1..input.len() {
        let last_token = if let Some(last_token) = input.get(i-1) {last_token} else {continue};
        let next_token = if let Some(next_token) = input.get(i+1) {next_token} else {continue};
        let current_token = if let Some(current_token) = input.get(i) {current_token} else {continue}; //would be weird if this continues
        if !(
                (custom_types.is_some() && (next_token.typ == TokenType::Keyword(Keyword::ArgumentStart) ||
                    next_token.typ == TokenType::Keyword(Keyword::TypeArgumentStart))) ||
                (custom_types.is_none() && (next_token.typ == TokenType::Keyword(Keyword::ScopeEnter) ||
                    next_token.typ == TokenType::Keyword(Keyword::TypeArgumentStart)))
        ){ continue; }
        if let Some(types) = custom_types{
            if let TokenType::Type(_) = last_token.typ { out.push(current_token.text.clone()); }
            if let TokenType::Keyword(Keyword::ArrayEnd) = last_token.typ { out.push(current_token.text.clone()); }
            if types.contains(&last_token.text) { out.push(current_token.text.clone()); }
        }
        if let None = custom_types{
            if let TokenType::Keyword(Keyword::Class) = last_token.typ { out.push(current_token.text.clone()); }
        }
    }
    out
}
pub fn resolve_ambiguous(input: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut variables: Vec<String> = Vec::new();
    let functions = match_functions_class(&input, None);
    let classes = match_functions_class(&input, Some(&functions));
    for token in input {
        if let TokenType::Unknown = token.typ {
            if functions.contains(&token.text) {out.push(token.clone().with_type(TokenType::FuncIdentifier))}
            if classes.contains(&token.text) {out.push(token.clone().with_type(TokenType::TypeIdentifier))}
            if let Some(last) = out.last() {
                if let TokenType::Type(_) = last.typ {
                    variables.push(last.text.clone());
                    out.push(token.clone().with_type(TokenType::Identifier));
                    continue;
                }
                if classes.contains(&last.text){
                    variables.push(last.text.clone());
                    out.push(token.clone().with_type(TokenType::Identifier));
                    continue;
                }
            } else if variables.contains(&token.text) {
                out.push(token.clone().with_type(TokenType::Identifier));
                continue;
            }
        }
        if let TokenType::Ambiguous(a, b) = &token.typ {
            if let Some(last) = out.last() {
                let token_type: &TokenType = if
                last.typ == TokenType::Operand ||
                    last.typ == TokenType::Keyword(Keyword::ArgumentEnd) ||
                    last.typ == TokenType::Identifier { a } else { b };
                if let TokenType::Multiple(vec) = token_type {
                    vec.iter().for_each(|t| { out.push(Token::from_type(t)); });
                    continue;
                } else {
                    out.push(Token::from_type(token_type));
                    continue;
                }
            }
        }
        out.push(token);
    }
    out
}