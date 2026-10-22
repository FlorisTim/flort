use std::cmp::{PartialEq};
use crate::common::*;

fn syntax_token_type(token: &str, last_token: Option<&Token>, config: &CompilerConfig) -> TokenType{
    let out = naive_syntax_token_type(token, last_token, config);
    let potential = config.ambiguous.get(&out);
    if potential.is_some() && !(last_token.unwrap().typ == TokenType::Operand) {
        TokenType::Ambiguous(Box::new(out.clone()),Box::new(potential.unwrap().clone()))
    } else {
        out
    }
}

fn naive_syntax_token_type(token: &str,last_token: Option<&Token>, config: &CompilerConfig) -> TokenType {
    let last_char = token.chars().last();
    if last_char.is_some() && last_char.unwrap() == config.assignment {
        let test_string = token[0..token.len()-1].to_string();
        let test_type = naive_syntax_token_type(&test_string, last_token, config);
        if let TokenType::Operation(a) = test_type{
            return TokenType::Keyword(Keyword::OperationAssignment(a));
        }
        if let TokenType::BooleanOperation(a) = test_type{
            return TokenType::Keyword(Keyword::BooleanOperationAssignment(a));
        }
        if let TokenType::BooleanOperation(a) = test_type{
            return TokenType::Keyword(Keyword::BooleanOperationAssignment(a));
        }
    }
    if &config.field_access.to_string() == token { return TokenType::Keyword(Keyword::FieldAccess); }
    if &config.type_specifier.to_string() == token { return TokenType::Keyword(Keyword::TypeSpecifier); }
    if &config.arg_seperator.to_string() == token { return TokenType::Keyword(Keyword::ArgumentSeperator); }
    if &config.assignment.to_string() == token { return TokenType::Keyword(Keyword::Assignment); }
    if let ScopeSymbol::OpenClose(a, b) = &config.scope {
        if token == a { return TokenType::Keyword(Keyword::ScopeEnter); }
        if token == b { return TokenType::Keyword(Keyword::ScopeExit); }
    }
    if token.eq(&config.end_statement.to_string()) { return TokenType::Keyword(Keyword::EndStatement); }
    if token.eq(&config.arguments[0]) { return TokenType::Keyword(Keyword::ArgumentStart); }
    if token.eq(&config.arguments[1]) { return TokenType::Keyword(Keyword::ArgumentEnd); }

    if token.eq(&config.array_arguments[0]) { return TokenType::Keyword(Keyword::ArrayStart); }
    if token.eq(&config.array_arguments[1]) { return TokenType::Keyword(Keyword::ArrayEnd); }

    let potential_op = config.operations.get(token);
    let potential_co = config.comparisons.get(token);
    let potential_boop = config.boolean_operations.get(token);
    if potential_op.is_some() {
        TokenType::Operation(potential_op.unwrap().clone())
    } else if potential_co.is_some() {
        TokenType::Comparison(potential_co.unwrap().clone())
    } else if potential_boop.is_some() {
        TokenType::BooleanOperation(potential_boop.unwrap().clone())
    } else { TokenType::Unknown }
}
fn normal_token_type(token: &str, config: &CompilerConfig) -> TokenType{
    let potential_kw = config.keywords.get(token);
    let potential_ty = config.types.get(token);
    let potential_boop = config.boolean_operations.get(token);
    let potential_co = config.constants.get(token);

    if potential_kw.is_some(){
        TokenType::Keyword(potential_kw.unwrap().clone())
    } else if potential_ty.is_some(){
        TokenType::Type(potential_ty.unwrap().clone())
    } else if is_number(token, config){
        TokenType::Operand
    } else if potential_boop.is_some() {
        TokenType::BooleanOperation(potential_boop.unwrap().clone())
    } else if potential_co.is_some() {
        TokenType::Constant(potential_co.unwrap().clone())
    } else {
        TokenType::Unknown
    }
}

fn token_type(current_string: &str,read_mode: &ReadMode,last_token: Option<&Token>, config: &CompilerConfig) -> TokenType{
    if let ReadMode::Syntax = read_mode{syntax_token_type(&current_string, last_token, config)} else {normal_token_type(&current_string, config)}
}

fn is_number(token: &str, config: &CompilerConfig) -> bool {
    let pot_suffix = config.number_suffixes.iter().find(|(a,_)| token.ends_with(*a));
    let pot_prefix = config.number_suffixes.iter().find(|(a,_)| token.starts_with(*a));

    let mut suffix: &String = &String::new();
    let mut prefix: &String = &String::new();

    let mut minimum = 0;

    if pot_prefix.is_some(){ prefix = pot_prefix.unwrap().0; minimum += prefix.len(); }
    if pot_suffix.is_some(){ suffix = pot_suffix.unwrap().0; minimum += suffix.len(); }


    let mut selected_chars = config.number_chars.clone();

    if prefix != "" {
        for c in &config.extended_number_chars{
            selected_chars.push(*c);
        }
    }

    let single_char = *config.number_chars.last().unwrap();

    if token.len() <= minimum {
        return false;
    }

    let new_token = String::from(&token[prefix.len()..token.len()-suffix.len()]);

    let mut single_char_used = false;
    let mut out = true;
    for c in new_token.chars() {
        if single_char_used && c == single_char {out = false};
        single_char_used |= c == single_char;
        out &= selected_chars.contains(&c);
    }
    out
}
fn reading_mode_of(character: char, current_string: &str, config: &CompilerConfig) -> ReadMode{

    let test = current_string.to_owned() + character.to_string().as_str();
    if config.string_starters.contains(&test){
        return ReadMode::String;
    }
    if is_number(&test, config){
        return ReadMode::Normal;
    }
    if config.syntax_chars.contains(&character){ReadMode::Syntax} else {ReadMode::Normal}
}


fn parse_token(token: &str, read_mode: &ReadMode, last_token: Option<&Token>, config: &CompilerConfig) -> Token{
    let mut out = Token{
        typ: TokenType::Unknown,
        text: token.to_owned(),
        read: read_mode.to_owned(),
    };

    if read_mode == &ReadMode::Normal{
        out.typ = normal_token_type(token, config);
    }
    if read_mode == &ReadMode::Syntax{
        out.typ = syntax_token_type(token, last_token, config);
    }

    out
}


fn emit(tokens: &mut Vec<Token>, current_string: &mut String, read_mode: &ReadMode, config: &CompilerConfig, string_type: usize){
    if current_string.is_empty(){
        return;
    }
    let mut possible = parse_token(&current_string, &read_mode, tokens.last(), config);
    if read_mode == &ReadMode::String && possible.typ == TokenType::Unknown{
        possible.typ = TokenType::RawString(string_type);
    }
    tokens.push(possible);
    current_string.clear();
}


pub fn get_tokens(code: &str, config: &CompilerConfig) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_string: String = String::new();
    let mut read_mode: ReadMode = ReadMode::Normal;
    let mut last_char: char = config.end_statement;
    let mut string_type: usize = 0;
    for c in code.chars() {
        if let ReadMode::String = &read_mode {
            let mut test_string = current_string.to_owned() + c.to_string().as_str();
            if test_string.ends_with(config.string_enders.get(string_type).unwrap()) {
                emit(&mut tokens, &mut test_string, &read_mode, config, string_type);
                read_mode = ReadMode::Normal;
                current_string.clear();
                continue;
            }
        } else {
            let mut test_string = current_string.to_owned() + c.to_string().as_str();
            if c == config.splitter || c == '\r' || c == '\n' {
                if let ReadMode::Normal = read_mode
                { emit(&mut tokens, &mut current_string, &read_mode, config, 0); }
                continue;
            }
            if config.illegal_combinations.contains(&test_string) {
                emit(&mut tokens, &mut current_string, &read_mode, config, 0);
                test_string = c.to_string();
            }
            let rm_symbol = reading_mode_of(c, &current_string, config);
            if let ReadMode::String = rm_symbol {
                string_type = config.string_starters.iter().position(|s| s == test_string.as_str()).unwrap();
            }
            if rm_symbol != read_mode {
                emit(&mut tokens, &mut current_string, &read_mode, config, 0);
                read_mode = rm_symbol;
            }
            if config.always_singular.contains(&c) {
                emit(&mut tokens, &mut current_string, &read_mode, config, 0);
                let mut temp = c.to_string();

                emit(&mut tokens, &mut temp, &ReadMode::Syntax, config, 0);
                continue;
            }
            if c == config.field_access && tokens.last().is_some() {
                if tokens.last().unwrap().read == ReadMode::Syntax
                {
                    emit(&mut tokens, &mut current_string, &read_mode, config, 0);
                    let mut temp = c.to_string();

                    emit(&mut tokens, &mut temp, &ReadMode::Syntax, config, 0);
                    continue;
                }
            }
        }
        last_char = c;
        current_string.push(c);
    }
    let rm_symbol = if read_mode != ReadMode::String { reading_mode_of(last_char, &current_string, config) } else { read_mode };
    emit(&mut tokens, &mut current_string, &rm_symbol, config, 0);

    tokens
}


fn simple_token(token_type: TokenType) -> Token{
    Token{typ: token_type, text: String::new(), read: ReadMode::Normal}
}
fn list_types(tokens: &Vec<Token>) -> Vec<String>{
    let mut out: Vec<String> = Vec::new();
    let mut last: &TokenType = &TokenType::Unknown;
    for token in tokens {
        if TokenType::Unknown == token.typ {
            if let TokenType::Keyword(Keyword::Class) = last{
                out.push(token.text.clone());
            }
        }
        last = &token.typ;
    }
    out
}
fn list_variables_defs(tokens: &Vec<Token>, custom_types: &Vec<String>, config: &CompilerConfig, defs: bool) -> Vec<String>{
    let mut out: Vec<String> = Vec::new();
    let mut last_type: Type = Type::Void;
    let mut array_amt: u8 = 0;
    for i in 0..tokens.len() {
        let possible = tokens.get(i);
        if possible.is_none(){
            continue;
        }
        let current = possible.unwrap();
        if custom_types.contains(&current.text){
            last_type = Type::Custom(current.text.clone());
            array_amt = 0;
        }
        if config.types.contains_key(&current.text) {
            last_type = config.types[&current.text].clone();
            array_amt = 0;
        }
        if let TokenType::Keyword(Keyword::ArrayStart) = current.typ {
            array_amt += 1;
        }
        if TokenType::Unknown == current.typ {
            if i == 0{
                continue;
            }
            let pre = tokens.get(i-1);
            let suf = tokens.get(i+1);
            if pre.is_some() && suf.is_some(){
                let suf_:&Token = suf.unwrap();
                let pre_:&Token = pre.unwrap();
                if TokenType::Keyword(Keyword::ArgumentStart) == suf_.typ ||
                    TokenType::Keyword(Keyword::TypeArgumentStart) == suf_.typ{
                    if !defs{
                        continue;
                    }
                } else{
                    if defs{
                        continue;
                    }
                }
                let determined_type: Type =
                    if array_amt > 0 {Type::Array(Box::new(last_type.clone()), array_amt)} else {last_type.clone()};
                if let TokenType::Keyword(Keyword::ArrayEnd) = &pre_.typ{
                    out.push(current.text.clone());
                } else if let TokenType::Type(typ) = &pre_.typ{
                    out.push(current.text.clone());
                } else if custom_types.contains(&pre_.text){
                    out.push(current.text.clone());
                    continue;
                }
            }
        }
    }
    out
}
pub fn resolve_ambiguous(tokens: Vec<Token>, config: &CompilerConfig) -> Vec<Token>{
    let types = list_types(&tokens);
    let variables = list_variables_defs(&tokens, &types, config, false);
    let function_decl = list_variables_defs(&tokens, &types, config, true);

    let mut out: Vec<Token> = Vec::new();

    for i in 0..tokens.len(){
        let potential_token= tokens.get(i);
        if potential_token.is_none() {
            continue;
        }
        let mut token: Token = potential_token.unwrap().clone();
        let last = out.last();

        if let TokenType::Unknown = token.typ {
            if types.contains(&token.text) {
                token.typ = TokenType::TypeIdentifier;
            }
            if variables.contains(&token.text) {
                token.typ = TokenType::Identifier;
            }
            if function_decl.contains(&token.text) {
                token.typ = TokenType::FuncIdentifier;
            }
            let last = out.last();
            if last.is_some()
            {
                if let TokenType::Keyword(Keyword::TypeArgumentEnd) = last.unwrap().typ {
                    token.typ = TokenType::Identifier;
                }
                if let TokenType::Keyword(Keyword::ArrayEnd) = last.unwrap().typ {
                    token.typ = TokenType::Identifier;
                }
            }
        }

        if let TokenType::Ambiguous(a,b) = &token.typ{
            if last.is_some(){
                let last_ = last.unwrap();

                let a_ = a.as_ref();
                let b_ = b.as_ref();

                if TokenType::Operand == last_.typ{
                    token.typ = a_.clone();
                } else if let TokenType::Identifier = &last_.typ{
                    token.typ = a_.clone();
                } else {
                    if let TokenType::Multiple(vec) = b_{
                        for typ in vec{
                            out.push(simple_token(typ.clone()));
                        }
                        continue;
                    } else {
                        token.typ = b_.clone();
                    }
                }
            }
        }

        out.push(token);
    }

    out
}