use common::*;
use crate::common;

enum ExpressionParsing{
    Pratt,
    LeftToRight,
}

#[derive(Debug)]
pub enum Value{
    Int(i32),
    Float(f32),
    Bool(bool),
    Byte(i8),
    Char(char),
    Long(i64),
    Short(i16),
    Double(f64),
    String(String),
    Null
}
type Arguments = Vec<(Type,String)>;
type Expression = Vec<Node>;
enum Node{
    Function(Type,String,Arguments),
    ClassDefinition,
    Initializer(Type,String,Expression),
    Operator(Operation),
    Operand(Type,Value),
}


pub fn parse_value(value: Token, config: &CompilerConfig) -> Option<Value>{
    match value.typ {
        TokenType::Constant(s) => Some(match s {
            Constant::False => Value::Bool(false),
            Constant::True => Value::Bool(true),
            Constant::Null => Value::Null,
        }),
        TokenType::RawString(typ) => {
            let a = config.string_starters.get(typ);
            let b = config.string_enders.get(typ);
            if a.is_none() || b.is_none() {
                return None;
            }
            let out = value.text[a.unwrap().len()..value.text.len()-b.unwrap().len()].to_string();

            Some(if out.len() == 1 {Value::Char(out.chars().nth(0)?)} else {Value::String(out)})
        },
        TokenType::Operand => {
            let system_lookup = config.number_systems.iter().find(|(b,_)| value.text.starts_with(*b))?;
            let type_lookup = config.number_suffixes.iter().find(|(b,_)| value.text.ends_with(*b))?;

            let system: &NumberSystem = system_lookup.1;
            let typ: &Type = type_lookup.1;
            parse_number(
            &value.text[
                system_lookup.0.len()
                    ..
                    value.text.len() - type_lookup.0.len()
            ],
            system,
            typ,
            config,
            )
        }
        _ => None
    }
}

fn parse_number(input: &str, system: &NumberSystem, typ: &Type, config: &CompilerConfig) -> Option<Value>{
    let base = *system as i64;
    if input.contains(*config.number_chars.last()?){
        let output = parse_float(input, base, config)?;
        return match typ {
            Type::Float => Some(Value::Float(output as f32)),
            Type::Double => Some(Value::Double(output)),
            _ => None
        }
    }
    let output = parse_int(input, base, config)?;
    match typ {
        Type::Byte => Some(Value::Byte(output as i8)),
        Type::Integer => Some(Value::Int(output as i32)),
        Type::Long => Some(Value::Long(output)),
        Type::Short => Some(Value::Short(output as i16)),
        _ => None
    }
}

fn parse_int(input: &str, base: i64, config: &CompilerConfig) -> Option<i64>{
    let mut total: i64 = 0;
    let mut power = 1;
    for c in input.chars().rev(){
        println!("{c}");
        let worth = *config.number_values.get(&c)? as i64;
        total += worth * power;
        power *= base;
    }
    Some(total)
}

fn parse_float(input: &str, base: i64, config: &CompilerConfig) -> Option<f64>{
    let decimal = *config.number_chars.last()?;
    if !input.contains(decimal){ return Some(parse_int(input, base, config)? as f64)}
    if input.chars().last()?.eq(&decimal){ return Some(parse_int(&input[0..input.len()-1], base, config)? as f64)}
    let mut total: f64 = 0.;
    let mut power: f64 = input.chars().into_iter().position(|c: char| c == decimal).unwrap_or(1) as f64;
    power = (base as f64).powf(power-1.);
    println!("{power}");
    for c in input.chars(){
        if c == decimal{
            continue;
        }
        let worth = *config.number_values.get(&c)? as f64;
        total += worth * power;
        power /= base as f64;
    }
    Some(total)
}





pub fn parse () {

}