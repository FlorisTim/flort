use std::arch::is_aarch64_feature_detected;

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
    biggest_symbols: i8,
    end_statement_auto: bool,
    end_statement_char: char,
    splitter: char,
}

struct Token<'a>{
    value: &'a str,
    typ: Type
}

fn string_in_vec(string: &str, vec: &Vec<String>) -> Option<usize>{
    vec.iter().position(|a| a == string)
}

fn check_token<'a>(input: &'a str, ruleset: &Ruleset, in_string: bool) -> Token<'a>{
    let mut out: Token<'a> = Token{value: input, typ: Type::Unknown};

    if in_string{
        let closing = string_in_vec(input, &ruleset.string_closers);
        if !closing.is_none(){
            out.typ = Type::StringExit;
        }
    } else {
        let opening = string_in_vec(input, &ruleset.string_openers);
        if !opening.is_none(){
            out.typ = Type::StringEnter;
        }
    }

    out
}
fn tokens_from<'a>(code: &str, ruleset: &Ruleset) ->  Vec<Token<'a>> {
    let out: Vec<Token> = Vec::new();
    let mut current_string: Vec<char> = Vec::new();
    let in_string = false;

    for c in code.chars() {
        current_string.push(c);
    }

    out
}

pub fn compile(code: &str, ruleset: Ruleset) -> Vec<u8>{
    let out:Vec<u8> = Vec::new();
    let token: Vec<Token> = tokens_from(code, &ruleset);

    out
}
