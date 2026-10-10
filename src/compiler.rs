use std::cmp::PartialEq;
#[derive(PartialEq, Eq)]
enum Type{
    Unknown,
    ScopeEnter,
    ScopeExit,
    StringEnter,
    StringExit,
    String,
    PrimitiveValue,
    Type,
    Assignment,
    EndStatement
}

pub struct Ruleset{
    string_openers: Vec<String>,
    string_closers: Vec<String>,
    string_injections: Vec<String>,
    biggest_symbols: usize,
    end_statement_char: char,
    splitter: char,
    end_statement_auto: bool,
}

struct Token{
    value: String,
    typ: Type
}

fn string_in_vec(string: &str, vec: &Vec<String>) -> Option<usize>{
    vec.iter().position(|a| a == string)
}

fn check_token(input: String, ruleset: &Ruleset, in_string: bool) -> Token{
    let mut out: Token = Token{value: input, typ: Type::Unknown};

    if in_string{
        let closing = string_in_vec(&out.value, &ruleset.string_closers);
        if !closing.is_none(){
            out.typ = Type::StringExit;
        }
    } else {
        let opening = string_in_vec(&out.value, &ruleset.string_openers);
        if !opening.is_none(){
            out.typ = Type::StringEnter;
        }
    }

    out
}


fn tokens_from(code: &str, ruleset: &Ruleset) ->  Vec<Token> {
    let out: Vec<Token> = Vec::new();
    let mut current_token: Token;
    let mut current_string: String = String::new();
    let mut in_string = false;

    for c in code.chars() {
        current_string.push(c);
        if current_string.len() <= ruleset.biggest_symbols{
            current_token = check_token(current_string.clone(), &ruleset, in_string);
            if current_token.typ == Type::Unknown{
                println!("invalid symbol: {}", &current_token.value);
                continue;
            }
            in_string = match current_token.typ {
                Type::StringEnter => true,
                Type::StringExit => false,
                _ => in_string
            }
        }
    }

    out
}

pub fn compile(code: &str, ruleset: Ruleset) -> Vec<u8>{
    let out:Vec<u8> = Vec::new();
    let token: Vec<Token> = tokens_from(code, &ruleset);

    out
}
