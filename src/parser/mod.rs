use std::collections::HashMap;
use common::*;
use crate::common;

enum ExpressionParsing{
    Pratt,
    LeftToRight,
}

#[derive(Debug, Clone)]
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
    Function(String,Type,Arguments),
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

struct Parser<'a> {
    pub(crate) config: &'a CompilerConfig,
    pub(crate) output_nodes: Vec<Node>,
}
impl<'a> Parser<'a>
{
    pub fn parse_value(&self, value: Token, config: &CompilerConfig) -> Option<Value> {
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
                let out = value.text[a.unwrap().len()..value.text.len() - b.unwrap().len()].to_string();

                Some(if out.len() == 1 { Value::Char(out.chars().nth(0)?) } else { Value::String(out) })
            }
            TokenType::Operand => {
                let system_lookup = config.number_systems.iter().find(|(b, _)| value.text.starts_with(*b))?;
                let type_lookup = config.number_suffixes.iter().find(|(b, _)| value.text.ends_with(*b))?;

                let system: Base = *system_lookup.1;
                let typ: &Type = type_lookup.1;
                self.parse_number(
                    &value.text[
                        system_lookup.0.len()
                            ..
                            value.text.len() - type_lookup.0.len()
                        ],
                    system,
                    typ,
                )
            }
            _ => None
        }
    }

    fn parse_number(&self, input: &str, system: Base, typ: &Type) -> Option<Value> {
        let base = system;
        if input.contains(self.config.decimal) {
            let output = self.parse_float(input, base)?;
            return match typ {
                Type::Float => Some(Value::Float(output as f32)),
                Type::Double => Some(Value::Double(output)),
                _ => None
            };
        }
        let output = self.parse_int(input, base)?;
        match typ {
            Type::Byte => Some(Value::Byte(output as i8)),
            Type::Integer => Some(Value::Int(output as i32)),
            Type::Long => Some(Value::Long(output)),
            Type::Short => Some(Value::Short(output as i16)),
            _ => None
        }
    }

    fn parse_int(&self, input: &str, base: i64) -> Option<i64> {
        let mut total: i64 = 0;
        let mut power = 1;
        for c in input.chars().rev() {
            println!("{c}");
            let worth = *self.config.number_values.get(&c)? as i64;
            total += worth * power;
            power *= base;
        }
        Some(total)
    }

    fn parse_float(&self, input: &str, base: i64) -> Option<f64> {
        let decimal = self.config.decimal;
        if !input.contains(decimal) { return Some(self.parse_int(input, base)? as f64); }
        if input.chars().last()?.eq(&decimal) { return Some(self.parse_int(&input[0..input.len() - 1], base)? as f64); }
        let mut total: f64 = 0.;
        let mut power: f64 = input.chars().into_iter().position(|c: char| c == decimal).unwrap_or(1) as f64;
        power = (base as f64).powf(power - 1.);
        println!("{power}");
        for c in input.chars() {
            if c == decimal {
                continue;
            }
            let worth = *self.config.number_values.get(&c)? as f64;
            total += worth * power;
            power /= base as f64;
        }
        Some(total)
    }
}

impl<'a> Parser<'a>{

    pub fn get_classes(tokens: &Vec<Token>) -> Vec<ClassDescription> {
        let mut out = Vec::new();
        let mut vector: Vec<Token> = Vec::new();
        let mut c_depth: u32 = 0;
        let mut depth: u32 = 0;
        let mut index: usize = 0;
        let mut desc: ClassDescription = ClassDescription::new(tokens);

        let mut class_name = false;
        for t in tokens {
            if let TokenType::Keyword(Keyword::ScopeEnter) = t.typ{ depth += 1; }
            if let TokenType::Keyword(Keyword::ScopeExit) = t.typ{ depth -= 1; }
            if let TokenType::Keyword(Keyword::Class) = t.typ {
                c_depth = depth + 1;
                desc.start_token_index = index;
                class_name = false;
            }
            if !class_name {
                if let TokenType::TypeIdentifier = t.typ {
                    class_name = true;
                    desc.name = t.text.clone();
                }
            }
            if depth != c_depth {
                desc.end_token_index = index;
                out.push(desc);
                desc = ClassDescription::new(tokens);
            }
            index += 1;
        }
        out
    }
    pub fn parse() {

    }
}

pub struct FieldDescription<'a>{
    name: &'a str,
    typ: Type,
    value: Option<Expression>,
    private: bool,
    stat: bool,
    mutable: bool,
}

pub struct MethodDescription<'a>{
    name: &'a str,
    return_type: Type,
    arguments: Arguments,
    block: Expression,
    private: bool,
    stat: bool,
}
pub struct ClassDescription<'a>{
    source_tokens: &'a Vec<Token>,
    start_token_index: usize,
    end_token_index: usize,
    name: String,
    fields: Vec<FieldDescription<'a>>,
    methods: Vec<MethodDescription<'a>>,
}


impl ClassDescription<'_>{
    pub fn new(tokens: &Vec<Token>) -> ClassDescription {
        ClassDescription{
            source_tokens: tokens,
            start_token_index: 0,
            end_token_index: 0,
            name: String::new(),
            fields: Vec::new(),
            methods: Vec::new()
        }
    }

}