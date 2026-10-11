use std::cmp::PartialEq;

enum TokenType{
    Unknown
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
struct CompilerConfig{
    normal_chars: Vec<char>,
    syntax_chars: Vec<char>,
    scope: ScopeSymbol,
    splitter: char,
}

impl CompilerConfig{
    fn default() -> CompilerConfig{
        CompilerConfig{
            normal_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234556789_".chars().collect(),
            syntax_chars: "-+=\"'*7%@[]{}(),.\\/!&|^:;".chars().collect(),
            scope: ScopeSymbol::OpenClose(String::from("{"),String::from("}")),
            splitter: ' ',
        }
    }
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

fn reading_mode_of(character: char, config: &CompilerConfig) -> ReadMode{
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
            let rm_symbol = reading_mode_of(c, config);
            if rm_symbol != read_mode{
                read_mode = rm_symbol;
                tokens.push(parse_token(&current_string));
                current_string = String::new();
            }
            if c == config.splitter && current_string.len() > 1{
                tokens.push(parse_token(&current_string));
                current_string = String::new();
                continue;
            }
        }
        current_string.push(c);
    }

    tokens
}