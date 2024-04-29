use std::io::{stdin, BufRead, StdinLock};


#[macro_export]
macro_rules! println_iter {
    ($iter:expr) => {
        for val in $iter {
            print!("{} ", val);
        }
        println!();
    };
}

pub struct Input {
    stdin: StdinLock<'static>
}

impl Input {
    pub fn new() -> Self {
        Self {
            stdin: stdin().lock()
        }
    }
    pub fn line_str(&mut self) -> Vec<String> {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace().map(|x| x.to_string()).collect()
    }
    pub fn line(&mut self) -> Vec<usize> {
        self.line_str().into_iter().map(|x| x.parse::<usize>().unwrap()).collect()
    }
    pub fn line1(&mut self) -> usize {
        self.line()[0]
    }
    pub fn line2(&mut self) -> (usize, usize) {
        let ln = self.line();
        (ln[0], ln[1])
    }
    pub fn line3(&mut self) -> (usize, usize, usize) {
        let ln = self.line();
        (ln[0], ln[1], ln[2])
    }
    pub fn line4(&mut self) -> (usize, usize, usize, usize) {
        let ln = self.line();
        (ln[0], ln[1], ln[2], ln[3])
    }
    pub fn line5(&mut self) -> (usize, usize, usize, usize, usize) {
        let ln = self.line();
        (ln[0], ln[1], ln[2], ln[3], ln[4])
    }
}