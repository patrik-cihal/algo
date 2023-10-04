pub mod primes;

pub mod integer;
pub use integer::Integer;

pub mod modular;

pub mod fractions;

pub fn gcd<T: Integer>(mut a: T, mut b: T) -> T {
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    while a != T::ZERO {
        b = b % a;
        std::mem::swap(&mut a, &mut b);
    }

    b
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}
