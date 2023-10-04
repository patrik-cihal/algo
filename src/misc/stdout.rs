#[macro_export]
macro_rules! println_iter {
    ($iter:expr) => {
        for val in $iter {
            print!("{} ", val);
        }
        println!();
    };
}
