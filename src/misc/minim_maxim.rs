#[macro_export]
macro_rules! maxim {
    ($a:expr, $b:expr) => {
        if $b > $a {
            $a = $b;
        }
    };
}

#[macro_export]
macro_rules! minim {
    ($a:expr, $b:expr) => {
        if $b < $a {
            $a = $b;
        }
    };
}
