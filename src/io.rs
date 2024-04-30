use std::io::{self, stdin, BufRead, BufWriter, StdinLock, Stdout, StdoutLock, Write};

pub static mut WRITER: Option<BufWriter<Stdout>> = None;
pub static mut READER: Option<StdinLock> = None;

pub fn writer<'a>() -> &'a mut BufWriter<Stdout> {
    unsafe {WRITER.get_or_insert_with(|| std::io::BufWriter::new(std::io::stdout()))}
}

pub fn reader<'a>() -> &'a mut StdinLock<'static> {
    unsafe {READER.get_or_insert_with(|| stdin().lock())}
}

#[macro_export]
macro_rules! outln {
    // Match arguments similar to `println!`
    () => (outln!(""));
    ($fmt:expr) => ({
        writeln!(writer(), $fmt).unwrap();
    });
    ($fmt:expr, $($arg:tt)*) => ({
        writeln!(writer(), $fmt, $($arg)*).unwrap();
    });
}

#[macro_export]
macro_rules! out {
    () => (out!(""));
    ($fmt:expr) => ({
        write!(writer(), $fmt).unwrap();
    });
    ($fmt:expr, $($arg:tt)*) => ({
        write!(writer(), $fmt, $($arg)*).unwrap();
    });
}

#[macro_export]
macro_rules! read {
    () => {
        reader().read_line(&mut String::new()).unwrap();
    };
    [$t : ty] => {{
        let line = {
            let mut buf = String::new();
            reader().read_line(&mut buf).unwrap();
            buf
        };

        line.split_whitespace().map(|x| x.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};

    ($($t:ty),+) => {{
        let line = {
            let mut buf = String::new();
            reader().read_line(&mut buf).unwrap();
            buf
        };

        let mut iter = line.split_whitespace();
        (
            $(
                iter.next().expect("Expected more input").parse::<$t>()
                    .expect(&format!("Failed to parse input as {}", stringify!($t)))
            ),+
        )
    }};
}


#[cfg(test)]
mod tests {
    use super::{writer, reader};
    use std::io::{Write, BufRead};
    #[test]
    pub fn writing() {
        outln!("Hello {}", "patrik");
        writer().flush().unwrap();
    }
    #[test]
    pub fn read_write() {
        out!("Your name please: ");
        writer().flush().unwrap();
    }
    #[test]
    pub fn reading_tuples() {
        let n = read!(usize);
        let v = read![usize];

        for val in v {
            out!("{val} ");
        }
        outln!();
        writer().flush().unwrap();
    }
}