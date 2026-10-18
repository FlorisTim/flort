use common::*;
use crate::common;

enum ExpressionParsing{
    Pratt,
    LeftToRight,
}

enum Value{
    Int(i32),
    Float(f32),
    Bool(bool),
    Byte(i8),
    Char(char),
    Long(i64),
    Short(i16),
    Double(f64),
    String(String),
}

enum Node{
    Function(Type,String,Vec<Argument>),
    ClassDefinition,
    Initializer(Type,String,Vec<Node>),
    Operator(Operation),
    Operand(Type,Value),
}
struct Argument{
    typ: Type,
    name: String
}

fn parse_value(value: &str, typ: &Type) -> Value{

}

pub fn parse () {

}