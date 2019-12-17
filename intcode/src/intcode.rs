
use std::iter::repeat;
use std::mem::replace;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[macro_export]
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

struct IntCode {
    memory: Memory,
    tx: Sender<String>,
    rx: Receiver<String>,
    ptr: usize,
    rel: i64,
}

impl IntCode {
    fn new(line: String, tx: Sender<String>, rx: Receiver<String>) -> Self {
        let memory = create_memory(line).to_owned();
        Self {
            memory: memory,
            tx,
            rx,
            ptr: 0,
            rel: 0,
        }
    }
    fn extend(&mut self, addr: &usize) {
        // debug!("extend({}, {})", addr, self.memory.len());
        if self.memory.len() <= *addr {
            self.memory
                .extend(repeat(0).take(*addr - self.memory.len() + 1))
        }
    }
    fn get_address(&mut self, mode: u32, offset: usize) -> usize {
        let addr = self.ptr + offset;
        let out = match mode {
            0 => self.memory[addr] as usize,
            1 => addr,
            2 => {
                let addr = self.rel + self.memory[addr];
                addr as usize
            }
            _ => unimplemented!(),
        };
        self.extend(&out);
        debug!("get_addr({}, {}) -> {}", mode, offset, out);
        out
    }

    fn input(&self) -> i64 {
        let line = self.rx.recv().expect("input: cannot receive");
        parse!(line, i64)
    }
    fn output(&mut self, value: i64) {
        self.tx
            .send(format!("{}", value))
            .expect("output: cannot transmit value");
    }

    pub fn run(&mut self) {
        let mut last = 0;
        loop {
            let code = self.memory[self.ptr];
            debug!("run|ptr:{}, rel:{}, {:?}", self.ptr, self.rel, self.memory);
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        let b = p.get_address(self.1, 2);
        let c = p.get_address(self.2, 3);
        debug!("{:?}:{} {} {}", self, a, b, c);
        let value = p.memory[a] + p.memory[b];
        replace(&mut p.memory[c], value);
        p.ptr + 4
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        let b = p.get_address(self.1, 2);
        let c = p.get_address(self.2, 3);
        debug!("{:?}:{} {} {}", self, a, b, c);
        let value = p.memory[a] * p.memory[b];
        replace(&mut p.memory[c], value);
        p.ptr + 4
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        debug!("{:?}:{}", self, a);
        let value = p.input();
        replace(&mut p.memory[a], value);
        p.ptr + 2
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        debug!("{:?}:{}", self, a);
        p.output(p.memory[a]);
        p.ptr + 2
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        let b = p.get_address(self.1, 2);
        let va = p.memory[a];
        let vb = p.memory[b];
        debug!("{:?}:[{}, {}] -> [{}, {}]", self, a, b, va, vb);
        if va > 0 {
            return vb as usize;
        }
        p.ptr + 3
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        let b = p.get_address(self.1, 2);
        let va = p.memory[a];
        let vb = p.memory[b];
        debug!("{:?}:[{}, {}] -> [{}, {}]", self, a, b, va, vb);
        if va == 0 {
            return vb as usize;
        }
        p.ptr + 3
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        let b = p.get_address(self.1, 2);
        let c = p.get_address(self.2, 3);
        debug!("{:?}:{} {} {}", self, a, b, c);
        let value = p.memory[a] < p.memory[b];
        if value {
            replace(&mut p.memory[c], 1);
        } else {
            replace(&mut p.memory[c], 0);
        }
        p.ptr + 4
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
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        let b = p.get_address(self.1, 2);
        let c = p.get_address(self.2, 3);
        debug!("{:?}:{} {} {}", self, a, b, c);
        let value = p.memory[a] == p.memory[b];
        if value {
            replace(&mut p.memory[c], 1);
        } else {
            replace(&mut p.memory[c], 0);
        }
        p.ptr + 4
    }
}
#[derive(Debug)]
struct AdjustRel(u32);
impl AdjustRel {
    fn new(params: String) -> Self {
        let mut params = params;
        let a = pop_digit!(&mut params);
        Self(a)
    }
}

impl Instruction for AdjustRel {
    fn call(&self, p: &mut IntCode) -> usize {
        let a = p.get_address(self.0, 1);
        debug!("{:?}:{}", self, a);
        let value = p.memory[a];
        p.rel += value;
        p.ptr + 2
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
        9 => Box::new(AdjustRel::new(params)),
        _ => unimplemented!(),
    }
}

pub fn spawn(
    data: String,
    init: Option<String>,
) -> (Sender<String>, Receiver<String>, thread::JoinHandle<()>) {
    let (tx, rxp) = channel();
    let (txp, rx) = channel();
    let handle = thread::spawn(move || IntCode::new(data, txp, rxp).run());
    match init {
        Some(data) => tx.send(data).unwrap(),
        None => {}
    };
    (tx, rx, handle)
}

#[cfg(test)]
mod test {
    use super::*;
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
    fn test_add() {
        let data = String::from("1,0,0,0,99");
        let (tx, rx) = channel();
        let mut p = IntCode::new(data, tx, rx);
        p.run();
        let expected: Memory = vec![2, 0, 0, 0, 99];
        assert_eq!(p.memory, expected);
    }
    #[test]
    fn test_mul() {
        let data = String::from("2,3,0,3,99");
        let (tx, rx) = channel();
        let mut p = IntCode::new(data, tx, rx);
        p.run();
        let expected: Memory = vec![2, 3, 0, 6, 99];
        assert_eq!(p.memory, expected);

        let data = String::from("2,4,4,5,99,0");
        let (tx, rx) = channel();
        let mut p = IntCode::new(data, tx, rx);
        p.run();
        let expected: Memory = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(p.memory, expected);

        let data = String::from("1,1,1,4,99,5,6,0,99");
        let (tx, rx) = channel();
        let mut p = IntCode::new(data, tx, rx);
        p.run();
        let expected: Memory = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(p.memory, expected);
    }
    #[test]
    fn test_pos_eq8() {
        let data = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("1".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 0);
        }
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("8".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 1);
        }
    }

    #[test]
    fn test_pos_lt8() {
        let data = String::from("3,9,7,9,10,9,4,9,99,-1,8");
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("1".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 1);
        }
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("8".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 0);
        }
    }

    #[test]
    fn test_immediate_eq8() {
        let data = String::from("3,3,1108,-1,8,3,4,3,99");
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("1".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 0);
        }
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("8".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 1);
        }
    }

    #[test]
    fn test_immediate_lt8() {
        let data = String::from("3,3,1107,-1,8,3,4,3,99");
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("1".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 1);
        }
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("8".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 0);
        }
    }
    #[test]
    fn test_pos_jump() {
        let data = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("8".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 1);
        }
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("0".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 0);
        }
    }

    #[test]
    fn test_immediate_jump() {
        let data = String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("8".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 1);
        }
        {
            let (_tx, rx, _) = spawn(data.clone(), Some("0".to_string()));
            let pout = rx.recv().expect("cannot recv");
            assert_eq!(parse!(pout, i64), 0);
        }
    }
    #[test]
    fn test_relative_mode() {
        env_logger::init();
        let data = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let (_tx, rx, _) = spawn(data.clone(), None);
        let mut output: Vec<String> = Vec::with_capacity(16);
        for recv in rx {
            output.push(recv);
        }
        assert_eq!(output.join(","), data);
    }

    #[test]
    fn test_sixteen_digits() {
        let data = String::from("1102,34915192,34915192,7,4,7,99,0");
        let (_tx, rx, _) = spawn(data.clone(), None);
        let output = rx.recv().unwrap();
        assert_eq!(output, "1219070632396864".to_string());
    }

    #[test]
    fn test_output_large() {
        let data = String::from("104,1125899906842624,99");
        let (_tx, rx, _) = spawn(data.clone(), None);
        let output = rx.recv().unwrap();
        assert_eq!(output, "1125899906842624".to_string());
    }
}
