use std::io::{BufRead, Write};
use std::iter::repeat;
use std::mem::replace;
macro_rules! parse {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().expect("parse failed")
    };
}

type Memory = Vec<i64>;
fn create_memory(data: String) -> Memory {
    data.trim_end()
        .split(",")
        .map(|x| {
            //debug!("cm_map{:?}", x);
            x.parse().unwrap()
        })
        .collect()
}

pub struct IntCode<'a> {
    memory: Memory,
    input: Box<&'a mut dyn BufRead>,
    output: Box<&'a mut dyn Write>,
    ptr: usize,
}

impl<'a> IntCode<'a> {
    pub fn new(
        line: String,
        input: Box<&'a mut dyn BufRead>,
        output: Box<&'a mut dyn Write>,
    ) -> Self {
        let memory = create_memory(line).to_owned();
        Self {
            memory: memory,
            input,
            output,
            ptr: 0,
        }
    }
    fn extend(&mut self, addr: &usize) {
        self.memory.extend(repeat(0).take(addr - self.memory.len()))
    }
    fn fetch_param(&mut self, mode: u32, offset: usize) -> i64 {
        if self.ptr + offset > self.memory.len() {
            self.extend(&offset);
        }
        let value = self.memory[self.ptr + offset];
        match mode {
            0 => {
                if value as usize > self.memory.len() {
                    self.extend(&(value as usize));
                }
                self.memory[value as usize]
            }
            1 => value,
            _ => unimplemented!(),
        }
    }
    fn write(&mut self, address: usize, value: i64) {
        replace(&mut self.memory[address], value);
    }

    fn input(&mut self) -> i64 {
        let mut line = String::new();
        self.input.read_line(&mut line).expect("coudnt read line");
        parse!(line, i64)
    }
    fn output(&mut self, value: i64) {
        writeln!(self.output.as_mut(), "{}", value).expect("coudnt write line")
    }

    pub fn run(&mut self) {
        let mut last = 0;
        loop {
            let code = self.memory[self.ptr];
            debug!("run|ptr:{}", self.ptr);
            debug!("run|memory:{:?}", self.memory);
            let inst = instruction(code);
            self.ptr = inst.call(self);
            if self.ptr == last {
                break;
            }
            last = self.ptr;
        }
    }
}

trait Instruction {
    fn call(&self, intcode: &mut IntCode) -> usize;
}

macro_rules! pop_digit {
    ($x:expr) => {
        $x.pop().unwrap_or('0').to_digit(10).unwrap_or(0)
    };
}

#[derive(Debug)]
struct Halt();
impl Halt {
    fn new() -> Self {
        Self()
    }
}

impl Instruction for Halt {
    fn call(&self, intcode: &mut IntCode) -> usize {
        debug!("{:?}", self);
        intcode.ptr + 0
    }
}
#[derive(Debug)]
struct Add(u32, u32, u32);

impl Add {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        let b = pop_digit!(&mut params);
        let c = pop_digit!(&mut params);
        Self(a, b, c)
    }
}
impl Instruction for Add {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        let vb = intcode.fetch_param(self.1, 2);
        let ac = intcode.fetch_param(1, 3);
        debug!("{:?}|{},{},{}", self, va, vb, ac);
        intcode.write(ac as usize, va + vb);
        intcode.ptr + 4
    }
}

#[derive(Debug)]
struct Mul(u32, u32, u32);
impl Mul {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        let b = pop_digit!(&mut params);
        let c = pop_digit!(&mut params);
        Self(a, b, c)
    }
}
impl Instruction for Mul {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        let vb = intcode.fetch_param(self.1, 2);
        let ac = intcode.fetch_param(1, 3);
        debug!("{:?}|{},{},{}", self, va, vb, ac);
        intcode.write(ac as usize, va * vb);
        intcode.ptr + 4
    }
}
#[derive(Debug)]
struct In(u32);
impl In {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        Self(a)
    }
}

impl Instruction for In {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let aa = intcode.fetch_param(1, 1);
        let va = intcode.input();
        debug!("{:?}|{},{}", self, aa, va);
        intcode.write(aa as usize, va);
        intcode.ptr + 2
    }
}

#[derive(Debug)]
struct Out(u32);

impl Out {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        Self(a)
    }
}

impl Instruction for Out {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        debug!("{:?}|{}", self, va);
        intcode.output(va);
        intcode.ptr + 2
    }
}
#[derive(Debug)]
struct JumpTrue(u32, u32);
impl JumpTrue {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        let b = pop_digit!(&mut params);
        Self(a, b)
    }
}

impl Instruction for JumpTrue {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        let vb = intcode.fetch_param(self.1, 2);
        debug!("{:?}|{},{}", self, va, vb);
        if va > 0 {
            return vb as usize;
        }
        intcode.ptr + 3
    }
}

#[derive(Debug)]
struct JumpFalse(u32, u32);
impl JumpFalse {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        let b = pop_digit!(&mut params);
        Self(a, b)
    }
}

impl Instruction for JumpFalse {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        let vb = intcode.fetch_param(self.1, 2);
        debug!("{:?}|{},{}", self, va, vb);
        if va == 0 {
            return vb as usize;
        }
        intcode.ptr + 3
    }
}

#[derive(Debug)]
struct LessThan(u32, u32, u32);
impl LessThan {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        let b = pop_digit!(&mut params);
        let c = pop_digit!(&mut params);
        Self(a, b, c)
    }
}

impl Instruction for LessThan {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        let vb = intcode.fetch_param(self.1, 2);
        let ac = intcode.fetch_param(1, 3);
        debug!("{:?}|{},{},{}", self, va, vb, ac);
        if va < vb {
            intcode.write(ac as usize, 1);
        } else {
            intcode.write(ac as usize, 0);
        }
        intcode.ptr + 4
    }
}

#[derive(Debug)]
struct Equals(u32, u32, u32);
impl Equals {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        let b = pop_digit!(&mut params);
        let c = pop_digit!(&mut params);
        Self(a, b, c)
    }
}

impl Instruction for Equals {
    fn call(&self, intcode: &mut IntCode) -> usize {
        let va = intcode.fetch_param(self.0, 1);
        let vb = intcode.fetch_param(self.1, 2);
        let ac = intcode.fetch_param(1, 3);
        debug!("{:?}|{},{},{}", self, va, vb, ac);
        if va == vb {
            intcode.write(ac as usize, 1);
        } else {
            intcode.write(ac as usize, 0);
        }
        intcode.ptr + 4
    }
}

fn opcode(code: i64) -> (u8, String) {
    let digits = code.to_string();
    if digits.len() <= 2 {
        return (code as u8, String::from(""));
    }
    (
        digits[digits.len() - 2..]
            .parse::<u8>()
            .expect("coudnt parse opcode"),
        digits[..digits.len() - 2].to_string(),
    )
}

fn instruction(code: i64) -> Box<dyn Instruction> {
    let (op, params) = opcode(code);
    debug!("inst({},{})", op, params);
    match op {
        99 => Box::new(Halt::new()),
        1 => Box::new(Add::new(params)),
        2 => Box::new(Mul::new(params)),
        3 => Box::new(In::new(params)),
        4 => Box::new(Out::new(params)),
        5 => Box::new(JumpTrue::new(params)),
        6 => Box::new(JumpFalse::new(params)),
        7 => Box::new(LessThan::new(params)),
        8 => Box::new(Equals::new(params)),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    #[test]
    fn test_opcode() {
        let (op, param) = opcode(1002);
        assert_eq!(op, 2);

        let add = Add::new(param);
        assert_eq!(add.0, 0);
        assert_eq!(add.1, 1);
        assert_eq!(add.2, 0);
    }
    #[test]
    fn test_intcode() {
        env_logger::init();
        let mut cursor = io::Cursor::new(b"1\n");
        let input: Box<&mut dyn BufRead> = Box::new(&mut cursor);
        let mut outbuf: Vec<u8> = Vec::new();
        {
            let output: Box<&mut dyn Write> = Box::new(&mut outbuf);
            let mut p: IntCode =
                IntCode::new(String::from("1,9,10,3,2,3,11,0,99,30,40,50"), input, output);
            p.run();
            println!("{:?}", p.memory);
            assert_eq!(p.memory[0], 3500);
            assert_eq!(p.memory[3], 70);
        }
        println!("{:?}", &outbuf);
    }
}
