use std::{collections::VecDeque, io::{self, stdin, BufRead, BufWriter, StdinLock, Stdout, StdoutLock, Write}, str::{FromStr, SplitWhitespace}};

pub static mut WRITER: Option<BufWriter<Stdout>> = None;
pub static mut READER: Option<Reader<'static>> = None;

pub fn writer<'a>() -> &'a mut BufWriter<Stdout> {
    unsafe {WRITER.get_or_insert_with(|| std::io::BufWriter::new(std::io::stdout()))}
}

pub fn reader<'a>() -> &'a mut Reader<'static> {
    unsafe {READER.get_or_insert_with(|| Reader::<'static>::new())}
}

pub struct Reader<'a> {
    reader: StdinLock<'a>,
    active_line: VecDeque<String>
}

impl<'a> Reader<'a> {
    pub fn new() -> Self {
        Self {
            reader: stdin().lock(),
            active_line: VecDeque::new()
        }
    }
    pub fn token<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: std::fmt::Debug {
        if let Some(token) = self.active_line.pop_front() {
            return token.parse::<T>().unwrap();
        }

        let mut buf = String::new();
        self.reader.read_line(&mut buf).unwrap();
        self.active_line = buf.split_whitespace().map(|x| x.to_string()).collect();
        self.token()
    }
}

#[macro_export]
macro_rules! outp {
    () => (out!(""));
    ($fmt:expr) => ({
        write!(writer(), $fmt).unwrap();
    });
    ($fmt:expr, $($arg:tt)*) => ({
        write!(writer(), $fmt, $($arg)*).unwrap();
    });
}

#[macro_export]
macro_rules! inp {
    [$t : ty; $len : expr] => {{
        let reader = reader();
        let mut res = Vec::with_capacity($len);
        for _ in 0..$len {
            res.push(reader.token::<$t>());
        }
        res
    }};

    ($($t:ty),+) => {{
        let reader = reader();

        (
            $(
                reader.token::<$t>()
            ),+
        )
    }};
}


mod tests {
    use super::{writer, reader};
    use std::io::{Write, BufRead};
    #[test]
    pub fn test_output() {
        outp!("Hello world\n");
        outp!("{} And this bob\n", 54);
    }
    #[test]
    pub fn test_input_and_output() {
        let (n, k) = inp!(usize, usize);
        let a = inp![usize; n];

        for val in a {
            outp!("{val} ");
        }
        outp!("\n");

        writer().flush().unwrap();
    }
}