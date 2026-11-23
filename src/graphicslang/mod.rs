
use std::process::exit;
use colored::Colorize;
use macroquad::color::WHITE;
use macroquad::prelude::draw_circle;
use macroquad::shapes::draw_rectangle;


type Element = [u8; 512];
const ELEMENT: Element = [0u8; 512];

const FUNCTIONS: [&str; 10] = [
    "f",
    "->",
    "div",
    "sub",
    "add",
    "mul",
    "mod",
    "pow",
    "circle",
    "rectangle",
];

const ME_VALUES: [&str; 7] = [
    "me::x",
    "me::y",
    "me::width",
    "me::height",
    "me::color",
    "me::background-color",
    "me",
];

const EOF: u8 = 0;
const EOS: u8 = 6;
const INT: u8 = 1;
const NEG: u8 = 3;
const VAR: u8 = 4;
const RESULT: u8 = 5;

const VALID_NUMBER_CHARS: [char; 11] = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    '-',
];
pub struct GraphicParser{
    pub output: Element,
    pub index: usize,
}

impl GraphicParser {
    pub fn new() -> Self{
        GraphicParser{
            output: ELEMENT,
            index: 0,
        }
    }

    pub fn parse_number(input: &str) -> i16{
        let mut out: i16 = 0;
        let negative = input.starts_with('-');
        let mut power: i16 = 10i16.checked_pow((input.len() as i16 - negative as i16) as u32).unwrap_or_else(|| {
            println!("{}{}", "number too big or small (allowed range is from negative to positive 8191): ".red(), &input.on_black().white());
            exit(65)
        });

        for c in input.chars(){
            if c == '-'{ continue; }
            power /= 10;
            let n = (VALID_NUMBER_CHARS.iter().position(|&x| x == c).unwrap() as i16) * power;
            out += n;

        }
        if negative{
            return -out
        }
        out
    }

    pub fn valid_number(input: &str) -> bool{
        for c in input.chars(){ if !VALID_NUMBER_CHARS.contains(&c){return false} }
        true
    }

    pub fn add_byte(&mut self, byte: u8){
        if self.index >= self.output.len() {return}
        self.output[self.index] = byte;
        self.index += 1;
    }

    pub fn parse_graphics_lang(&mut self, input: &str) {
        self.index = 0;
        let mut start = 0;

        let mut variables: [&str; 32] = [""; 32];
        let mut var_index: usize = 0;

        let mut debug_argument_amount: u8 = 0;

        for i in 0..input.len() {
            let char = &input.chars().nth(i).unwrap();
            let mut test = &input[start..i].trim();
            match char{
                &'(' =>{
                    debug_argument_amount = 0;
                    if FUNCTIONS.contains(&test){
                        self.add_byte((FUNCTIONS.iter().position(|a| a == test).unwrap()+1) as u8);
                    } else {
                        println!("{}{}", "no such function: ".red(), &test.on_black().white());
                        exit(65)
                    }
                    start = i+1;
                    continue;
                }

                &',' | &')' =>{

                    let mut typ: u8 = VAR;
                    let mut value: u16 = 0;
                    if ME_VALUES.contains(&test){
                        typ = VAR;
                        value = ME_VALUES.iter().position(|a| a == test).unwrap() as u16;
                    } else if Self::valid_number(&test) {
                        let mut number = Self::parse_number(&test);
                        let negative = number < 0;
                        number = number.abs();
                        if number >= 8192 /*8192 is 2^13, 3 bits are used to declare what type of number it is*/{
                            println!("{}{}", "number too big or small (allowed range is from negative to positive 8191): ".red(), &test.on_black().white());
                            exit(65)
                        }
                        typ = if negative {NEG} else {INT};
                        value = number as u16;
                    } else if test == &"result"{
                        typ = RESULT;
                    } else {
                        typ = VAR;
                        if variables.contains(&test){
                            value = (variables.iter().position(|a| a == test).unwrap() as u16) + 32;
                        } else {
                            variables[var_index] = test;
                            value = (var_index as u16) + 32;
                            var_index += 1;
                        }
                    }

                    value = value | ((typ as u16) << 13);

                    let first = (value >> 8) as u8;
                    let second = value as u8;
                    debug_argument_amount += 1;
                    self.add_byte(first);
                    self.add_byte(second);
                    if char == &')'{
                        self.add_byte(EOS);
                    } else {
                        if debug_argument_amount >= 5{
                            println!("{}{}", "max arguments exceeded (5 max) after: ".red(), &test.on_black().white());
                            exit(65)
                        }
                    }
                    start = i+1;
                    continue;
                }

                _=> {continue}
            }
        }
    }
}

pub struct GraphicDrawer{
    floats: [f32; 64],
    arguments: [f32; 5]
}

impl GraphicDrawer {
    pub fn new() -> Self{
        GraphicDrawer{floats: [0.0; 64], arguments: [0.0; 5]}
    }

    pub fn draw_element(&mut self, item: Element) {
        let mut function: u8 = 255; //none
        let mut start = false;

        let mut result: f32 = 0f32;
        
        let mut argument_index: usize = 0;
        let mut i = 0;
        for _ in 0..item.len() {
            let byte = item[i];
            if byte == 6 {
                start = false;

                if function >= 3 && function <= 7 {
                    if argument_index == 1 {
                        self.arguments[1] = self.arguments[0];
                        self.arguments[0] = result;
                    }
                }
                argument_index = 0;
                match function {
                    1 => { result = self.arguments[0]; },
                    2 => { self.floats[self.arguments[0] as usize] = result; },
                    3 => {
                        if self.arguments[1] == 0. {
                            self.arguments[1] = f32::MIN_POSITIVE;
                        }
                        result = self.arguments[0] / self.arguments[1];
                    },
                    4 => { result = self.arguments[0] - self.arguments[1]; },
                    5 => { result = self.arguments[0] + self.arguments[1]; },
                    6 => { result = self.arguments[0] * self.arguments[1]; },
                    7 => { result = self.arguments[0] % self.arguments[1]; },
                    8 => { result = self.arguments[0].powf(self.arguments[1]); },
                    9 => { draw_circle(self.arguments[0], self.arguments[1], self.arguments[2], WHITE); },
                    10 => { draw_rectangle(self.arguments[0], self.arguments[1], self.arguments[2], self.arguments[3], WHITE); },

                    _ => {}
                }
                i += 1;
                continue;
            } else if byte == 0 {
                return;
            }

            if !start {
                function = byte;
                start = true;
            } else {
                let first = byte as u16;
                i += 1;
                let second = item[i] as u16;

                let typ = (first >> 5) as u8;
                let value = ((first << 8) | second) & 0b0001111111111111;


                let fin = match typ {
                    VAR => {
                        if function == 2 {
                            value as f32
                        } else {
                            self.floats[value as usize]
                        }
                    }
                    INT => value as f32,
                    NEG => -(value as f32),
                    RESULT => result,
                    _ => 0.
                };

                self.arguments[argument_index] = fin;
                argument_index += 1;
            }
            i += 1;
        }
    }
}