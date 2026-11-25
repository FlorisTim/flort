use colored::Colorize;
use macroquad::color::Color;
use macroquad::prelude::draw_circle;
use macroquad::shapes::draw_rectangle;
use std::iter::Iterator;
use std::process::exit;


type Element = [u8; 152];
const ELEMENT: Element = [0u8; 152];

type Attributes = [f32; ME_VALUES.len()+6];

const FUNCTIONS: [&str; 18] = [
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
    "half",
    "double_save",
    "start_over",
    "color_copy",
    "from_here",
    "2circle",
    "sub_save",
    "add_save",
];

const ME_VALUES: [(&str, usize); 8] = [
    ("me::x", 0),
    ("me::y", 1),
    ("me::width",2),
    ("me::height",3),
    ("me::color",7),
    ("me::border_color",11),
    ("me::border_radius",12),
    ("me::border_thickness",13),
];

const INT: u8 = 1;
const SMALL_INT: u8 = 5;
const NEG: u8 = 2;
const VAR: u8 = 3;
const RESULT: u8 = 4;

const VALID_NUMBER_CHARS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'];
pub struct GraphicParser {
    pub output: Element,
    pub index: usize,
}

impl GraphicParser {

    pub fn new() -> Self {
        GraphicParser {
            output: ELEMENT,
            index: 0,
        }
    }

    pub fn parse_number(input: &str) -> i16 {
        let mut out: i16 = 0;
        let negative = input.starts_with('-');
        let mut power: i16 = 10i16
            .checked_pow((input.len() as i16 - negative as i16) as u32)
            .unwrap_or_else(|| {
                println!(
                    "{}{}",
                    "number too big or small (allowed range is from negative to positive 8191): "
                        .red(),
                    &input.on_black().white()
                );
                exit(65)
            });

        for c in input.chars() {
            if c == '-' {
                continue;
            }
            power /= 10;
            let n = (VALID_NUMBER_CHARS.iter().position(|&x| x == c).unwrap() as i16) * power;
            out += n;
        }
        if negative {
            return -out;
        }
        out
    }

    pub fn valid_number(input: &str) -> bool {
        for c in input.chars() {
            if !VALID_NUMBER_CHARS.contains(&c) {
                return false;
            }
        }
        true
    }

    pub fn add_byte(&mut self, byte: u8) {
        if self.index >= self.output.len() - if byte == 0 {0} else {1} {
            println!("{}", "file is too big".red());
            exit(65);
        }
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
            let test2 = if test.len() > 1 {&test[1..test.len()]} else {test};
            match char {
                &'(' => {
                    debug_argument_amount = 0;
                    if FUNCTIONS.contains(&test) {
                        self.add_byte(
                            (FUNCTIONS.iter().position(|a| a == test).unwrap() + 1) as u8,
                        );
                    } else {
                        println!("{}{}", "no such function: ".red(), &test.on_black().white());
                        exit(65)
                    }
                    start = i + 1;
                    continue;
                }


                &',' | &')' | '|' => {
                    if *test == ""{
                        if char == &')'{
                            self.add_byte(0);
                        }
                        start = i + 1;
                        continue;
                    }

                    let mut typ: u8 = VAR;
                    let mut value: u16 = 0;
                    if ME_VALUES.iter().any(|(a,_)| a == test) {
                        typ = VAR;
                        ME_VALUES.iter().for_each(|(a,b)| {if a == test{value = *b as u16 }})
                    } else if ME_VALUES.iter().any(|(a,_)| **a == *test2) {
                        typ = INT;
                        ME_VALUES.iter().for_each(|(a,b)| {if *a == test2{value = *b as u16 }})
                    } else if Self::valid_number(&test) {
                        let mut number = Self::parse_number(&test);
                        let negative = number < 0;
                        number = number.abs();
                        if number >= 8192
                        /*8192 is 2^13, 3 bits are used to declare what type of number it is*/
                        {
                            println!("{}{}", "number too big or small (allowed range is from negative to positive 8191): ".red(), &test.on_black().white());
                            exit(65)
                        }
                        typ = if negative { NEG } else { INT };
                        value = number as u16;
                    } else if test == &"result" {
                        typ = RESULT;
                    } else {
                        typ = VAR;
                        if variables.contains(&test) {
                            value = (variables.iter().position(|a| a == test).unwrap() as u16) + 32;
                        } else if test.starts_with('&') {
                            typ = INT;
                            if let Some(a) = variables.iter().position(|a| **a == *test2){
                                value = (a as u16) + 32;
                            }
                        } else {
                            if *test != "_"
                            { variables[var_index] = test; }
                            value = (var_index as u16) + 32;
                            var_index += 1;
                        }
                    }

                    if value < 32 && typ == INT {
                        typ = SMALL_INT;
                        value = (value) | ((typ as u16) << 5)
                    } else {
                        value = value | ((typ as u16) << 13);
                    }

                    let first = (value >> 8) as u8;
                    let second = value as u8;
                    debug_argument_amount += 1;
                    if typ != SMALL_INT {
                        self.add_byte(first);
                    }
                    self.add_byte(second);
                    if char == &')' {
                        self.add_byte(0);
                    } else if char == &'|' {
                        let value = (var_index + 32) as u16 | ((INT as u16) << 13);
                        let first = (value >> 8) as u8;
                        let second = value as u8;
                        self.add_byte(0); // end of previous statement
                        self.add_byte(2); // save function
                        self.add_byte(first);  // value part 1
                        self.add_byte(second); // value part 2
                        self.add_byte(0); // end of save function
                        self.add_byte(1); //start of load function
                        var_index += 1;
                        start = i + 1;
                        continue;
                    } else {
                        if debug_argument_amount >= 5 {
                            println!(
                                "{}{}",
                                "max arguments exceeded (5 max) after: ".red(),
                                &test.on_black().white()
                            );
                            exit(65)
                        }
                    }
                    start = i + 1;
                    continue;
                }

                _ => continue,
            }
        }
    }
}

pub struct GraphicDrawer {
    floats: [f32; 64],
    arguments: [f32; 5],
}

impl GraphicDrawer {
    pub fn new() -> Self {
        GraphicDrawer {
            floats: [0.0; 64],
            arguments: [0.0; 5],
        }
    }

    pub fn color_from(&mut self, value: f32) -> Color {
        let value = value as usize;
        Color::from_rgba(
            self.floats[value - 3] as u8,
            self.floats[value - 2] as u8,
            self.floats[value - 1] as u8,
            self.floats[value] as u8,
        )
    }

    pub fn draw_element(&mut self, item: Element, values: Attributes) {
        let mut function: u8 = 255; //none
        let mut start = false;
        let mut i = 0;
        for f in values {
            self.floats[i] = f;
            i += 1;
        }

        let mut result: f32 = 0f32;
        let mut started_over = false;
        let mut start_over_point = 0usize;
        let mut argument_index: usize = 0;
        i = 0;
        while i < item.len() {
            let byte = item[i];
            if byte == 0 {
                if i > 2 {
                    if item[i - 3..i] == [0, 0, 0] {
                        return;
                    }
                }
                start = false;

                if function >= 3 && function <= 7 {
                    if argument_index == 1 {
                        self.arguments[1] = self.arguments[0];
                        self.arguments[0] = result;
                    }
                }
                argument_index = 0;
                match function {
                    1 => {
                        result = self.arguments[0];
                        //println!("loaded {}", result);
                    }
                    2 => {
                        self.floats[self.arguments[0] as usize] = result;
                        //println!("set {} to {}", self.arguments[0], result);
                    }
                    3 => {
                        if self.arguments[1] == 0. {
                            self.arguments[1] = f32::MIN_POSITIVE;
                        }
                        result = self.arguments[0] / self.arguments[1];
                    }
                    4 => {
                        result = self.arguments[0] - self.arguments[1];
                    }
                    5 => {
                        result = self.arguments[0] + self.arguments[1];
                    }
                    6 => {
                        result = self.arguments[0] * self.arguments[1];
                    }
                    7 => {
                        result = self.arguments[0] % self.arguments[1];
                    }
                    8 => {
                        result = self.arguments[0].powf(self.arguments[1]);
                    }
                    9 => {
                        draw_circle(
                            self.arguments[0],
                            self.arguments[1],
                            self.arguments[2],
                            Self::color_from(self, self.arguments[3]),
                        );
                    }
                    10 => {
                        draw_rectangle(
                            self.arguments[0] - self.arguments[2] / 2.,
                            self.arguments[1] - self.arguments[3] / 2.,
                            self.arguments[2],
                            self.arguments[3],
                            Self::color_from(self, self.arguments[4]),
                        );
                    }
                    11 => {
                        result = self.arguments[0] / 2.
                    }
                    12 => {
                        let loc = self.arguments[0] as usize;
                        self.floats[loc] = self.floats[loc] * 2.
                    }
                    13 => {
                        if !started_over{
                            i = start_over_point;
                            started_over = true;
                            continue;
                        }
                    }
                    14 => {
                        self.floats[self.arguments[0] as usize] = self.floats[self.arguments[1] as usize];
                        self.floats[self.arguments[0] as usize - 1] = self.floats[self.arguments[1] as usize - 1];
                        self.floats[self.arguments[0] as usize - 2] = self.floats[self.arguments[1] as usize - 2];
                        self.floats[self.arguments[0] as usize - 3] = self.floats[self.arguments[1] as usize - 3];
                    }
                    15 => {
                        start_over_point = i
                    }
                    16 => {
                        let color = Self::color_from(self, self.arguments[4]);

                        draw_circle(
                            self.arguments[0],
                            self.arguments[1],
                            self.arguments[3],
                            color
                        );
                        draw_circle(
                            self.arguments[0],
                            self.arguments[2],
                            self.arguments[3],
                            color
                        );
                    }
                    17 => {
                        let loc = self.arguments[0] as usize;
                        self.floats[loc] = self.floats[loc] - self.arguments[1];

                    }
                    18 => {
                        let loc = self.arguments[0] as usize;
                        self.floats[loc] = self.floats[loc] + self.arguments[1];
                    }
                    _ => {}
                }
                i += 1;
                continue;
            }

            if !start {
                function = byte;
                start = true;
            } else {
                let first = byte as u16;
                let typ = (first >> 5) as u8;
                if typ != SMALL_INT {
                    i += 1;
                    let second = item[i] as u16;


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
                        _ => 0.,
                    };

                    self.arguments[argument_index] = fin;
                } else {
                    self.arguments[argument_index] = (first & 0b00011111) as f32;
                }
                argument_index += 1;
            }
            i += 1;
        }
    }
    pub fn attributes(x:f32, y:f32, width:f32, height:f32, color: Color, border_color: Color, border_radius:f32, border_thickness:f32) -> Attributes {
        [
            x, //x
            y, //y
            width, //w
            height, //h
            color.r * 256., //color
            color.g * 256.,
            color.b * 256.,
            color.a * 256.,
            border_color.r * 256., //border color
            border_color.g * 256.,
            border_color.b * 256.,
            border_color.a * 256.,
            border_radius, //border radius
            border_thickness, //border thickness
        ]
    }
}
