use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct MInt<const MODULUS: u64> {
    pub value: u64,
}

impl<const M: u64> Display for MInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MODULUS: u64> MInt<MODULUS> {
    pub fn one() -> Self {
        Self { value: 1 }
    }
    pub fn zero() -> Self {
        Self { value: 0 }
    }
    pub fn new(value: u64) -> Self {
        Self {
            value: value % MODULUS,
        }
    }
    pub fn pow(self, n: u64) -> MInt<MODULUS> {
        let mut result = Self::one();
        let mut base = self;
        let mut exponent = n;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result * base;
            }
            base = base * base;
            exponent /= 2;
        }
        result
    }
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = Self::extended_gcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
    pub fn inv(self) -> Self {
        let (_, x, _) = Self::extended_gcd(self.value as i64, MODULUS as i64);
        Self::new((x + MODULUS as i64) as u64)
    }
}

impl<const T: u64> Add<MInt<T>> for MInt<T> {
    type Output = MInt<T>;
    fn add(self, rhs: MInt<T>) -> Self::Output {
        let mut value = self.value + rhs.value;
        if value >= T {
            value -= T;
        }
        Self { value }
    }
}

impl<const T: u64> AddAssign<MInt<T>> for MInt<T> {
    fn add_assign(&mut self, rhs: MInt<T>) {
        *self = *self + rhs;
    }
}

impl<const T: u64> Mul<MInt<T>> for MInt<T> {
    type Output = MInt<T>;
    fn mul(self, rhs: MInt<T>) -> Self::Output {
        Self {
            value: (self.value * rhs.value) % T,
        }
    }
}

impl<const T: u64> MulAssign<MInt<T>> for MInt<T> {
    fn mul_assign(&mut self, rhs: MInt<T>) {
        self.value = (self.value * rhs.value) % T;
    }
}

impl<const T: u64> Sub<MInt<T>> for MInt<T> {
    type Output = MInt<T>;
    fn sub(self, rhs: MInt<T>) -> Self::Output {
        let mut value = self.value + T - rhs.value;
        if value >= T {
            value -= T;
        }
        Self { value }
    }
}

impl<const T: u64> SubAssign<MInt<T>> for MInt<T> {
    fn sub_assign(&mut self, rhs: MInt<T>) {
        *self = *self - rhs;
    }
}

impl<const T: u64> Div<MInt<T>> for MInt<T> {
    type Output = MInt<T>;
    fn div(self, rhs: MInt<T>) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const T: u64> DivAssign<MInt<T>> for MInt<T> {
    fn div_assign(&mut self, rhs: MInt<T>) {
        *self = *self / rhs;
    }
}

impl<const T: u64> Into<MInt<T>> for u64 {
    fn into(self) -> MInt<T> {
        MInt::new(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const MOD: u64 = 1_000_000_007;

    #[test]
    fn test_constructors_and_basic_properties() {
        let a = MInt::<MOD>::one();
        let b = MInt::<MOD>::zero();
        let c = MInt::<MOD>::new(123456789);

        assert_eq!(a.value, 1);
        assert_eq!(b.value, 0);
        assert_eq!(c.value, 123456789);
    }

    #[test]
    fn test_addition() {
        let a = MInt::<MOD>::new(500_000_004);
        let b = MInt::<MOD>::new(500_000_004);

        let sum = a + b;

        assert_eq!(sum.value, 1);
    }

    #[test]
    fn test_subtraction() {
        let a = MInt::<MOD>::new(5);
        let b = MInt::<MOD>::new(7);

        let diff = a - b;

        assert_eq!(diff.value, MOD - 2);
    }

    #[test]
    fn test_multiplication() {
        let a = MInt::<MOD>::new(500_000_004);
        let b = MInt::<MOD>::new(2);

        let prod = a * b;

        assert_eq!(prod.value, 1);
    }

    #[test]
    fn test_add_assign() {
        let mut a = MInt::<MOD>::new(500_000_004);
        let b = MInt::<MOD>::new(500_000_004);

        a += b;

        assert_eq!(a.value, 1);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = MInt::<MOD>::new(5);
        let b = MInt::<MOD>::new(7);

        a -= b;

        assert_eq!(a.value, MOD - 2);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = MInt::<MOD>::new(500_000_004);
        let b = MInt::<MOD>::new(2);

        a *= b;

        assert_eq!(a.value, 1);
    }

    #[test]
    fn test_pow() {
        let a = MInt::<MOD>::new(3);
        let result = a.pow(4);

        assert_eq!(result.value, 81);
    }

    #[test]
    fn test_inv() {
        let a = MInt::<MOD>::new(500_000_004);
        let inverse = a.inv();

        assert_eq!(inverse.value, 2);
    }
}


#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct MInt128<const MODULUS: u128> {
    pub value: u128,
}

impl<const M: u128> Display for MInt128<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MODULUS: u128> MInt128<MODULUS> {
    pub fn one() -> Self {
        Self { value: 1 }
    }
    pub fn zero() -> Self {
        Self { value: 0 }
    }
    pub fn new(value: u128) -> Self {
        Self {
            value: value % MODULUS,
        }
    }
    pub fn pow(self, n: u128) -> MInt128<MODULUS> {
        let mut result = Self::one();
        let mut base = self;
        let mut exponent = n;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result * base;
            }
            base = base * base;
            exponent /= 2;
        }
        result
    }
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = Self::extended_gcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
    pub fn inv(self) -> Self {
        let (_, x, _) = Self::extended_gcd(self.value as i64, MODULUS as i64);
        Self::new((x + MODULUS as i64) as u128)
    }
}

impl<const T: u128> Add<MInt128<T>> for MInt128<T> {
    type Output = MInt128<T>;
    fn add(self, rhs: MInt128<T>) -> Self::Output {
        let mut value = self.value + rhs.value;
        if value >= T {
            value -= T;
        }
        Self { value }
    }
}

impl<const T: u128> AddAssign<MInt128<T>> for MInt128<T> {
    fn add_assign(&mut self, rhs: MInt128<T>) {
        *self = *self + rhs;
    }
}

impl<const T: u128> Mul<MInt128<T>> for MInt128<T> {
    type Output = MInt128<T>;
    fn mul(self, rhs: MInt128<T>) -> Self::Output {
        Self {
            value: (self.value * rhs.value) % T,
        }
    }
}

impl<const T: u128> MulAssign<MInt128<T>> for MInt128<T> {
    fn mul_assign(&mut self, rhs: MInt128<T>) {
        self.value = (self.value * rhs.value) % T;
    }
}

impl<const T: u128> Sub<MInt128<T>> for MInt128<T> {
    type Output = MInt128<T>;
    fn sub(self, rhs: MInt128<T>) -> Self::Output {
        let mut value = self.value + T - rhs.value;
        if value >= T {
            value -= T;
        }
        Self { value }
    }
}

impl<const T: u128> SubAssign<MInt128<T>> for MInt128<T> {
    fn sub_assign(&mut self, rhs: MInt128<T>) {
        *self = *self - rhs;
    }
}

impl<const T: u128> Div<MInt128<T>> for MInt128<T> {
    type Output = MInt128<T>;
    fn div(self, rhs: MInt128<T>) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const T: u128> DivAssign<MInt128<T>> for MInt128<T> {
    fn div_assign(&mut self, rhs: MInt128<T>) {
        *self = *self / rhs;
    }
}

impl<const T: u128> Into<MInt128<T>> for u128 {
    fn into(self) -> MInt128<T> {
        MInt128::new(self)
    }
}