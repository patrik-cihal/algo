use super::Integer;

pub trait PrimeExt: Integer {
    fn is_prime(&self) -> bool {
        let mut cur = Self::ONE + Self::ONE;

        while cur*cur <= *self {
            if *self % cur == Self::ZERO {
                return false;
            }
            cur = cur + Self::ONE;
        }

        true
    }
}