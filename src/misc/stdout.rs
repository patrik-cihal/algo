#[macro_export]
macro_rules! print_iter {
    ($iter:expr) => {
        for val in $iter {
            print!("{} ", val);
        }
        println!();
    };
}