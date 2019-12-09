pub type Memory = Vec<i64>;
use std::mem::replace;
pub fn create_memory(data: String) -> Memory {
    data.split(",").map(|x| x.parse().unwrap()).collect()
}

pub fn intcode(memory: &mut Memory) -> i64 {
    let mut ptr: usize = 0;
    loop {
        if ptr >= memory.len() {
            return memory[0];
        }
        let opcode = memory[ptr];
        match opcode {
            99 => return memory[0],
            1 => {
                let ra = memory[ptr + 1] as usize;
                let rb = memory[ptr + 2] as usize;
                let rc = memory[ptr + 3] as usize;
                let a = memory[ra];
                let b = memory[rb];
                replace(&mut memory[rc], a + b);
                ptr = ptr + 4;
            }
            2 => {
                let ra = memory[ptr + 1] as usize;
                let rb = memory[ptr + 2] as usize;
                let rc = memory[ptr + 3] as usize;
                let a = memory[ra];
                let b = memory[rb];
                replace(&mut memory[rc], a * b);
                ptr = ptr + 4;
            }
            _ => unreachable!(),
        }
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
    }
    #[test]
    fn test_intcode() {
        let mut mem = create_memory("1,9,10,3,2,3,11,0,99,30,40,50".to_string());
        assert_eq!(intcode(&mut mem), 3500);

        let mut mem = create_memory("1,0,0,0,99".to_string());
        assert_eq!(intcode(&mut mem), 2);

        let mut mem = create_memory("2,3,0,3,99".to_string());
        intcode(&mut mem);
        assert_eq!(mem[3], 6);

        let mut mem = create_memory("2,4,4,5,99,0".to_string());
        intcode(&mut mem);
        assert_eq!(mem[5], 9801);

        let mut mem = create_memory("1,1,1,4,99,5,6,0,99".to_string());
        assert_eq!(intcode(&mut mem), 30);
        assert_eq!(mem[4], 2);
    }
}
