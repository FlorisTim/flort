use num_traits::ToBytes;

static STRINGS_LOCATION: usize = 0;
static FILE_LOCATION: usize = 1;

static EOF: u8 = 255u8;
static TERMINATOR: u8 = 254u8;

pub struct VM{
    strings: Vec<&'static str>,
    stack: [u8; 1_000_000usize],
    top: usize,
}
//deliberately unsafe for performance
impl VM {
    pub fn new() -> VM {VM{strings: Vec::new(),stack: [0; 1_000_000usize], top: 0usize}}
    pub fn get_string(&mut self, index: usize) -> &'static str {
        self.strings.get(index).unwrap()
    }
    pub fn push_string(&mut self, value: &'static str){
        self.strings.push(value);
    }
    pub fn pop_string(&mut self) -> &'static str {
        self.strings.pop().unwrap()
    }
    pub fn push_bool(&mut self, value: bool){
        self.add(value);
    }
    pub fn pop_bool(&mut self) -> bool {
        self.top -= 1usize;
        self.stack[self.top] != 0
    }
    pub fn push_byte(&mut self, value: i8){
        self.add(value);
    }
    pub fn pop_byte(&mut self) -> i8 {
        self.top -= 1usize;
        self.stack[self.top] as i8
    }
    pub fn push_short(&mut self, value: i16){
        self.add(value);
    }
    pub fn pop_short(&mut self) -> i16 {
        self.top -= 2usize;
        self.get(self.top, 2)
    }
    pub fn push_int(&mut self, value: i32){
        self.add(value);
    }
    pub fn pop_int(&mut self) -> i32{
        self.top -= 4usize;
        self.get(self.top, 4)
    }
    pub fn push_long(&mut self, value: i64){
        self.add(value);
    }
    pub fn pop_long(&mut self) -> i64{
        self.top -= 8usize;
        self.get(self.top, 8)
    }
    pub fn push_long_long(&mut self, value: i128){
        self.add(value);
    }
    pub fn pop_long_long(&mut self) -> i128{
        self.top -= 16usize;
        self.get(self.top, 16)
    }

    pub fn add<'a, T: ToBytes<Bytes = [u8]>>(&mut self, value: T) -> usize where &'a [u8] : From<T>{
        let bytes:&[u8] = value.into();
        let size = bytes.len();
        self.stack[self.top..self.top + size].copy_from_slice(bytes);
        self.top += size;
        self.top - size
    }
    pub fn get<T>(&mut self, index: usize, size: usize) -> T{
        unsafe {
            (self.stack[index..index + size].as_ptr() as *const T).read_unaligned()
        }
    }
}

impl VM{
    fn get_strings(&mut self, bytes: &[u8; 1_000_000usize] ) -> Vec<&'static str> {
        let mut out:Vec<&'static str> = Vec::new();
        let mut i = bytes[STRINGS_LOCATION] as usize;
        let mut str: String = String::new();
        while bytes[i] != EOF{
            if bytes[i] == TERMINATOR {
                out.push(&str.clone());
                str.clear();
                continue;
            }
            str.push(bytes[i] as char);
            i += 1;
        }
        out
    }
    pub fn parse_bytecode(&mut self, bytes: [u8; 1_000_000usize]) -> VM{
        let mut out = VM{strings: self.get_strings(&bytes), stack: [0; 1_000_000], top: 0usize};

        out
    }
}