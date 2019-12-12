use std::iter::repeat;
use std::mem::replace;
use std::sync::mpsc::{Sender, Receiver};
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

pub struct IntCode {
    memory: Memory,
    rx: Receiver<String>,
    tx: Sender<String>,
    ptr: usize,
}

impl IntCode {
    pub fn new(
        line: String,
        rx: Receiver<String>,
        tx: Sender<String>,
    ) -> Self {
        let memory = create_memory(line).to_owned();
        Self {
            memory: memory,
            rx,
            tx,
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

    fn input(&self) -> i64 {
        let line = self.rx.recv().expect("input: cannot receive");
        parse!(line, i64)
    }
    fn output(&mut self, value: i64) {
        self.tx.send(format!("{}", value)).expect("output: cannot transmit value");
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
    use std::sync::mpsc::channel;
    use std::thread;
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
    fn test_immediate_eq8() {
        env_logger::init();
        let (tx, prx) = channel();
        let (ptx, rx) = channel();
        thread::spawn(move || {
            let mut p = IntCode::new(String::from("3,3,1108,-1,8,3,4,3,99"), prx, ptx);
            p.run();
        });
        tx.send(String::from("1")).expect("cannot send");
        let pout = rx.recv().expect("cannot recv");
        assert_eq!(parse!(pout, i64), 0);
    }
}
