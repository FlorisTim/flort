use crate::common::*;

pub fn get_tokens(input: &str, config: &CompilerConfig) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();

    let mut current_string: String = String::new();
    for c in input.chars() {

    }

    out
}


pub fn resolve_ambiguous(input: Vec<Token>, config: &CompilerConfig) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut variables: Vec<String> = Vec::new();
    for token in input{
        if let TokenType::Unknown = token.typ {
            if let Some(last) = out.last() {
                if let TokenType::Type(_) = last.typ {
                    variables.push(last.text.clone());
                }
            }
        }

        if let TokenType::Ambiguous(a,b) = &token.typ{
            if let Some(last) = out.last() {
                let token_type: &TokenType = if
                    last.typ == TokenType::Operand ||
                    last.typ == TokenType::Keyword(Keyword::ArgumentEnd) ||
                    last.typ == TokenType::Identifier { a } else { b };
                if let TokenType::Multiple(vec) = token_type {
                    vec.iter().for_each(|t| { out.push(Token::from_type(t));});
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