use super::Integer;

pub trait PrimeExt: Integer {
    fn is_prime(&self) -> bool {
        if *self < Self::ZERO {
            panic!("Attempted to check primality for negative integer.");
        }

        if *self == Self::ZERO || *self == Self::ONE {
            return false;
        }

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

impl<T: Integer> PrimeExt for T {}

#[cfg(test)]
mod tests {
    use super::PrimeExt;

    #[test]
    fn test_is_prime_small_numbers() {
        assert_eq!(2.is_prime(), true);
        assert_eq!(3.is_prime(), true);
        assert_eq!(4.is_prime(), false);
        assert_eq!(5.is_prime(), true);
    }

    #[test]
    fn test_is_prime_large_numbers() {
        assert_eq!(97.is_prime(), true);
        assert_eq!(100.is_prime(), false);
        assert_eq!(101.is_prime(), true);
        assert_eq!(103.is_prime(), true);
        assert_eq!(104.is_prime(), false);
    }

    #[test]
    fn test_is_prime_edge_cases() {
        assert_eq!(0.is_prime(), false);
        assert_eq!(1.is_prime(), false);
    }

    #[test]
    fn test_is_prime_non_primes() {
        assert_eq!(9.is_prime(), false);
        assert_eq!(15.is_prime(), false);
        assert_eq!(21.is_prime(), false);
        assert_eq!(25.is_prime(), false);
        assert_eq!(27.is_prime(), false);
    }
}