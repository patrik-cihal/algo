use super::Integer;

pub trait PrimeExt: Integer {
    /// Checks whether a given integer is prime
    fn is_prime(&self) -> bool {
        if *self < Self::ZERO {
            panic!("failed to check whether integer '{:?}' is a prime number, because it is smaller than zero for which this function is undefined", self);
        }

        if *self == Self::ZERO || *self == Self::ONE {
            return false;
        }

        let mut cur = Self::from_i64(2);

        while cur * cur <= *self {
            if *self % cur == Self::ZERO {
                return false;
            }
            cur = cur + Self::ONE;
        }

        true
    }

    /// Returns prime factors of an integer in increasing order
    fn factorize(&self) -> Vec<Self> {
        if *self < Self::ZERO {
            panic!("failed to find prime factors of the integer '{:?}', because it is smaller than zero for which this function is undefined", self);
        }

        let mut result = vec![];
        let mut k = Self::from_i64(2);
        let mut n = *self;

        while k * k <= *self {
            while n % k == Self::ZERO {
                result.push(k);
                n = n / k;
            }
            k = k + Self::ONE;
        }

        if n > Self::ONE {
            result.push(n);
        }

        result
    }
}

pub fn find_primes(n: usize) -> Vec<u64> {
    let mut is_prime = vec![true; n+1];
    let mut primes = vec![];

    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }

        primes.push(i as u64);

        for j in (i*i..=n).step_by(i) {
            is_prime[j] = false;
        }
    }

    primes
}

impl<T: Integer> PrimeExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize_all() {
        assert_eq!(find_primes(10), vec![2, 3, 5, 7])
    }

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

    #[test]
    fn test_factorize_primes() {
        assert_eq!(2.factorize(), vec![2]);
        assert_eq!(3.factorize(), vec![3]);
        assert_eq!(5.factorize(), vec![5]);
        assert_eq!(7.factorize(), vec![7]);
        assert_eq!(11.factorize(), vec![11]);
        assert_eq!(13.factorize(), vec![13]);
    }

    #[test]
    fn test_factorize_composites() {
        assert_eq!(4.factorize(), vec![2, 2]);
        assert_eq!(6.factorize(), vec![2, 3]);
        assert_eq!(8.factorize(), vec![2, 2, 2]);
        assert_eq!(9.factorize(), vec![3, 3]);
        assert_eq!(10.factorize(), vec![2, 5]);
        assert_eq!(12.factorize(), vec![2, 2, 3]);
        assert_eq!(15.factorize(), vec![3, 5]);
        assert_eq!(16.factorize(), vec![2, 2, 2, 2]);
        assert_eq!(18.factorize(), vec![2, 3, 3]);
    }

    #[test]
    fn test_factorize_less_than_two() {
        assert_eq!(1.factorize(), vec![]);
        assert_eq!(0.factorize(), vec![]);
    }

    #[test]
    fn test_factorize_product() {
        let factors = 100.factorize();
        let product: i64 = factors.iter().product();
        assert_eq!(product, 100);
    }
}
