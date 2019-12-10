use std::mem::replace;

pub type Memory = Vec<i64>;

pub fn create_memory(data: String) -> Memory {
    data.trim_end()
        .split(",")
        .map(|x| {
            //debug!("cm_map{:?}", x);
            x.parse().unwrap()
        })
        .collect()
}

fn parameter_mode(memory: &Memory, mode: &u8, ptr: usize) -> i64 {
    debug!("parameter_mode:{} {} {:?}", mode, ptr, memory);
    match mode {
        0 => memory[memory[ptr] as usize],
        1 => memory[ptr],
        _ => unimplemented!(),
    }
}

trait Instruction {
    fn call(&self, memory: &mut Memory) -> usize;
}
#[derive(Debug)]
struct Add {
    ptr: usize,
    ma: u8,
    mb: u8,
}

impl Instruction for Add {
    fn call(&self, memory: &mut Memory) -> usize {
        let va = parameter_mode(&memory, &self.ma, self.ptr + 1);
        let vb = parameter_mode(&memory, &self.mb, self.ptr + 2);
        let rc = memory[self.ptr + 3] as usize;
        debug!("{:?}, {}, {}, {}", self, va, vb, rc);
        replace(&mut memory[rc], va + vb);
        self.ptr + 4
    }
}
#[derive(Debug)]
struct Mul {
    ptr: usize,
    ma: u8,
    mb: u8,
}

impl Instruction for Mul {
    fn call(&self, memory: &mut Memory) -> usize {
        let va = parameter_mode(&memory, &self.ma, self.ptr + 1);
        let vb = parameter_mode(&memory, &self.mb, self.ptr + 2);
        let rc = memory[self.ptr + 3] as usize;
        debug!("{:?}, {}, {}, {}", self, va, vb, rc);
        replace(&mut memory[rc], va * vb);
        self.ptr + 4
    }
}
#[derive(Debug)]
struct In {
    ptr: usize,
}

impl Instruction for In {
    fn call(&self, memory: &mut Memory) -> usize {
        let ra = memory[self.ptr + 1] as usize;

        println!("enter value:");
        let input: i64;
        scan!("{}", input);
        replace(&mut memory[ra], input);
        self.ptr + 2
    }
}
#[derive(Debug)]
struct Out {
    ptr: usize,
    ma: u8,
}

impl Instruction for Out {
    fn call(&self, memory: &mut Memory) -> usize {
        debug!("{:?}", self);
        let va = parameter_mode(&memory, &self.ma, self.ptr + 1);
        println!("output: {}", va);
        self.ptr + 2
    }
}
#[derive(Debug)]
struct Halt {
    ptr: usize,
}

impl Instruction for Halt {
    fn call(&self, memory: &mut Memory) -> usize {
        self.ptr
    }
}

fn instruction_factory(ptr: &usize, code: String) -> Box<dyn Instruction> {
    debug!("factory({}, {})", ptr, code);
    let ptr = *ptr;
    let opcode: usize;
    let mut params = String::from("");
    if code.len() == 1 {
        opcode = code.parse().unwrap();
    } else {
        let code = code.chars().rev().collect::<String>();
        opcode = code[0..2]
            .chars()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .expect("invalid opcode");
        params = code[2..].to_string();
    }
    debug!("factory: {}, {}", opcode, params);
    match opcode {
        99 => Box::new(Halt { ptr }),
        1 => {
            let mut ma: u8 = 0;
            let mut mb: u8 = 0;
            if params.len() >= 1 {
                ma = params[0..1].parse().unwrap();
            }
            if params.len() >= 2 {
                mb = params[1..2].parse().unwrap();
            }
            Box::new(Add { ptr, ma, mb })
        }
        2 => {
            let mut ma: u8 = 0;
            let mut mb: u8 = 0;
            if params.len() >= 1 {
                ma = params[0..1].parse().unwrap();
            }
            if params.len() >= 2 {
                mb = params[1..2].parse().unwrap();
            }
            Box::new(Mul { ptr, ma, mb })
        }
        3 => Box::new(In { ptr }),
        4 => {
            let mut ma: u8 = 0;
            if params.len() >= 1 {
                ma = params[0..1].parse().unwrap();
            }
            Box::new(Out { ptr, ma })
        }
        _ => unimplemented!(),
    }
}

pub fn intcode(memory: &mut Memory) {
    let mut prev: usize = 0;
    let mut ptr: usize = 0;
    loop {
        if ptr >= memory.len() {
            panic!("ptr out of bounds");
        }
        let instr_box = instruction_factory(&ptr, memory[ptr].to_string());
        ptr = instr_box.as_ref().call(memory);
        if ptr == prev {
            return;
        }
        prev = ptr;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_memory() {
        let mem = create_memory("1,9,10,3,2,3,11,0,99,30,40,50".to_string());
        assert_eq!(mem.len(), 12 as usize);
        assert_eq!(mem[3], 3);
        let mut mem = create_memory("1,0,0,0,99".to_string());
        intcode(&mut mem);
        assert_eq!(mem[0], 2);

        let mut mem = create_memory("2,3,0,3,99".to_string());
        intcode(&mut mem);
        assert_eq!(mem[3], 6);

        let mut mem = create_memory("2,4,4,5,99,0".to_string());
        intcode(&mut mem);
        assert_eq!(mem[5], 9801);

        let mut mem = create_memory("1,1,1,4,99,5,6,0,99".to_string());
        intcode(&mut mem);
        assert_eq!(mem[0], 30);
        assert_eq!(mem[4], 2);
    }
}
