use std::process::exit;
use colored::Colorize;
use macroquad::logging::{error, warn};
use crate::common::*;
pub struct Lexer<'a> {
    pub(crate) config: &'a CompilerConfig,
    pub(crate) output_tokens: Vec<Token>
}

impl<'a> Lexer<'a>{
    pub fn new(config: &'a CompilerConfig) -> Lexer<'a> {
        Lexer{config, output_tokens: Vec::new()}
    }
    fn match_functions_class(input: &Vec<Token>, custom_types: Option<&Vec<String>>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for i in 1..input.len() {
            let last_token = if let Some(last_token) = input.get(i - 1) { last_token } else { continue };
            let next_token = if let Some(next_token) = input.get(i + 1) { next_token } else { continue };
            let current_token = if let Some(current_token) = input.get(i) { current_token } else { continue }; //would be weird if this continues
            if !(
                (custom_types.is_some() && (next_token.typ == TokenType::Keyword(Keyword::ArgumentStart) ||
                    next_token.typ == TokenType::Keyword(Keyword::TypeArgumentStart))) ||
                    custom_types.is_none()
            ) { continue; }
            if let Some(types) = custom_types {
                if let TokenType::Type(_) = last_token.typ { out.push(current_token.text.clone()); }
                if let TokenType::Keyword(Keyword::ArrayEnd) = last_token.typ { out.push(current_token.text.clone()); }
                if types.contains(&last_token.text) { out.push(current_token.text.clone()); }
            }
            if let None = custom_types {
                if let TokenType::Keyword(Keyword::Class) = last_token.typ { out.push(current_token.text.clone()); }
            }
        }
        out
    }
}

//we must assume utf8
impl<'a> Lexer<'_>
{
    pub fn valid_number(&self, string: &String) -> bool {
        let base_option = self.config.number_systems
            .iter()
            .find(|(pre, base)| { string.starts_with(*pre) });
        let suffix_option = self.config.number_suffixes
            .iter()
            .find(|(pre, base)| { string.ends_with(*pre) });
        let mut base = 10;
        let base_len = match base_option {
            None => 0,
            Some((str, b)) => {
                base = *b;
                str.len()
            }
        };
        let suffix_len = match suffix_option {
            None => 0,
            Some((str, _)) => str.len()
        };
        if string.len() <= base_len {return false;};
        let test_string = &string[base_len..string.len()-suffix_len];
        if test_string.len() == 0 {return false;}
        let mut decimal = false;
        for c in test_string.chars() {
            if c == self.config.decimal{
                if decimal {return false}
                decimal = true;
            } else {
                match self.config.number_values.get(&c) {
                    Some(num) => if *num >= base as u8 { return false },
                    None => return false
                }
            }
        }
        if decimal && test_string.len() == 1 {false} else {true}
    }
    fn token_type(&self, string: &String) -> &TokenType{
        if let Some(t) = self.config.keywords.get(string) {t} else {&TokenType::Unknown}
    }
    fn read_mode_of(&self, character: char) -> ReadMode {
        if self.config.syntax_chars.contains(&character) {ReadMode::Syntax} else {ReadMode::Normal}
    }
    pub fn get_tokens(&mut self, input: &str) {
        let mut index: usize = 0;
        let mut reading_mode: ReadMode = ReadMode::Normal;
        let mut current_string: String = String::new();
        let mut indent_stack: Vec<u8> = Vec::new();
        let mut newline_reached: bool = true;
        let mut indent_chars: u8 = 0;
        let mut indent_level: usize = 0;
        //todo string handling
        for c in input.chars() {
            index+=1;
            let splitter =  c == self.config.splitter;
            let next_cleared_string = if splitter {String::new()} else {c.to_string()};
            let next_string = current_string.clone() + &*next_cleared_string;

            let mut determined_type = self.token_type(&current_string).clone();
            let current_rm = self.read_mode_of(c);
            //begin check

            //indentation handling
            if c == '\r' || c == '\n' { index -= 1; newline_reached = true; continue}
            if let ScopeSymbol::Indent(indent) = &self.config.scope {
                let indent = *indent;
                if c != indent {newline_reached = false;}
                //todo...
            } else if let ScopeSymbol::OpenClose(chars) = &self.config.scope {
                let enter = current_string == chars[0].to_string();
                let exit = current_string == chars[1].to_string();
                if enter || exit {
                    self.output_tokens.push(Token::new(&TokenType::Keyword( if enter {Keyword::ScopeEnter} else {Keyword::ScopeExit}), &current_string,index));
                    current_string = next_cleared_string;
                    reading_mode = ReadMode::Normal;
                    continue;
                }
            }

            //numbers check if next string is a valid number, then skip this cycle (to make 5. be a number and not 5 and a field access)
            //if next string ia not valid check if im valid, and either the next symbol is the splitter or the next symbol is a different readmode
            //for example so that 5.5.pow(3) is recognized as 5.5, field access, pow(3). (5.5. isnt a valid number anymore), this will cause 5.5.5.5 to be 5.5 and then .5 and .5

            //also if interupted "0 x5" then its not valid anymore
            if self.valid_number(&next_string) && !splitter {
                current_string = next_string;
                continue;
            } else if self.valid_number(&current_string) && (c == self.config.splitter || reading_mode != current_rm){
                self.output_tokens.push(Token::new(&TokenType::Operand, &current_string,index));
                reading_mode = current_rm;
                current_string = next_cleared_string;
                continue;
            }

            //if read mode of c is not current rm then emit token (example: "abc!?*" gets split into "abc" and "!?*")
            if reading_mode != current_rm || splitter{
                self.output_tokens.push(Token::new(&determined_type, &current_string,index));
                reading_mode = current_rm;
                current_string = next_cleared_string;
                continue;
            }

            //if read mode is syntax, consume the first valid token so "+=" becomes "+=" and "=+" turns into "+" and "="
            if let ReadMode::Syntax = reading_mode {
                let next_determined_type = self.token_type(&next_string);
                if let TokenType::Unknown = next_determined_type{
                    self.output_tokens.push(Token::new(&determined_type, &current_string,index));
                    current_string = next_cleared_string;
                    continue;
                }
            }
            //end checks

            current_string = next_string;
        }
        self.output_tokens.push(Token::new(self.token_type(&current_string), &current_string,index));
    }

    pub fn resolve_ambiguous(&mut self) {
        let mut out: Vec<Token> = Vec::new();
        let mut variables: Vec<&String> = Vec::new();
        let classes = Lexer::match_functions_class(&self.output_tokens, None);
        let functions = Lexer::match_functions_class(&self.output_tokens, Some(&classes));
        for token in &self.output_tokens {
            if let TokenType::Unknown = token.typ {
                //quick fix for missed operands
                if self.valid_number(&token.text){out.push(token.clone().with_type(TokenType::Operand)); continue}
                if token.text.len() == 0{continue;}
                if functions.contains(&token.text) { out.push(token.clone().with_type(TokenType::FuncIdentifier)); continue }
                if classes.contains(&token.text) { out.push(token.clone().with_type(TokenType::TypeIdentifier)); continue }
                if let Some(last) = out.last() {
                    if let TokenType::Type(_) = last.typ {
                        variables.push(&token.text);
                        out.push(token.clone().with_type(TokenType::Identifier));
                        continue;
                    }
                    if TokenType::Keyword(Keyword::ArrayEnd) == last.typ ||
                       TokenType::Keyword(Keyword::TypeArgumentEnd) == last.typ ||
                       TokenType::Keyword(Keyword::ArgumentEnd) == last.typ
                    {
                        variables.push(&token.text);
                        out.push(token.clone().with_type(TokenType::Identifier));
                        continue;
                    }
                    if classes.contains(&last.text) {
                        variables.push(&token.text);
                        out.push(token.clone().with_type(TokenType::Identifier));
                        continue;
                    }
                }
                if variables.contains(&&token.text) {
                    out.push(token.clone().with_type(TokenType::Identifier));
                    continue;
                }
            }
            if let TokenType::Ambiguous(a, b) = &token.typ {
                if let Some(last) = out.last() {
                    let index = last.index;
                    if let TokenType::Unknown = last.typ {out.push(token.clone()); continue}
                    let token_type: &TokenType = if
                    last.typ == TokenType::Operand ||
                        last.typ == TokenType::Keyword(Keyword::ArgumentEnd) ||
                        last.typ == TokenType::Identifier { a } else { b };
                    if let TokenType::Multiple(vec) = token_type {
                        vec.iter().for_each(|t|{out.push(Token::from_type(t,index));});
                        continue;
                    } else {
                        out.push(Token::from_type(token_type, index));
                        continue;
                    }
                }
            }
            out.push(token.clone());
        }
        self.output_tokens = out
    }

    pub fn debug(&self){
        let mut out = false;
        let fatal = "Error".on_red();
        for t in &self.output_tokens{
            let index = t.index.checked_sub(1).unwrap_or(0);
            if let TokenType::Unknown  = t.typ {
                println!("{fatal} {} {index}","unknown token at");
                println!("      \"{}\" is never defined",t.text);
                println!();
                out = true;
            }
            if let TokenType::Ambiguous(a, b) = &t.typ {
                let a = a.as_ref().clone();
                let b = b.as_ref().clone();
                println!("{fatal} {} {index}","ambiguous token at");
                if let Some(msg) = self.config.ambiguous_debug.get(&t.text){
                    println!("      {}", msg);
                } else
                {
                    println!("      \"{}\" could be", t.text);
                    println!("          1. {a:?}");
                    println!("          2. {b:?}");
                }
                println!();
                out = true;
            }
        }
        if out {
            println!("{}", "Lexer output invalid!!! Can't start doing anything else, Read previous outputs for how badly you messed up.");
            exit(65);
        }
        println!("{}", "[SUCCESS]".on_green());
    }
}