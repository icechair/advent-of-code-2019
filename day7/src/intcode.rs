use std::io::{BufRead, Write};
type Memory = Vec<i64>;

pub struct IntCode<'a> {
    memory: Memory,
    input: &'a mut Box<dyn BufRead + 'a>,
    output: &'a mut Box<dyn Write + 'a>,
    ptr: usize,
}

impl<'a> IntCode<'a> {
    pub fn new(
        line: String,
        input: &'a mut Box<dyn BufRead + 'a>,
        output: &'a mut Box<dyn Write + 'a>,
    ) -> Self {
        Self {
            memory: line.trim_end().split(",").map(|x| x.parse().unwrap()).collect(),
            input,
            output,
            ptr: 0,
        }
    }
}
