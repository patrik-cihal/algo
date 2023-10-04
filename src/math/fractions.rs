use std::fmt::Display;

use super::{gcd, lcm, Integer};

impl Display for Fract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl<I: Integer> From<I> for Fract {
    fn from(value: I) -> Self {
        Self::new(value.to_i64(), 1) 
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Fract {
    num: i64,
    den: u64,
}

impl Fract {
    pub fn new<N: Integer, D: Integer>(num: N, den: D) -> Self {
        let (num, den) = Self::base_form(num, den);
        Self { num, den }
    }
    pub fn base_form<N: Integer, D: Integer>(num: N, den: D) -> (i64, u64) {
        let (mut num, mut den) = (num.to_i64(), den.to_i64());
        assert!(den != 0, "Division by zero!");
        let gd = gcd(num.abs(), den.abs());
        num /= gd;
        den /= gd;
        if den < 0 {
            den = -den;
            num = -num;
        }
        (num, den as u64)
    }
}


impl std::ops::Add<Fract> for Fract {
    type Output = Fract;

    fn add(self, rhs: Fract) -> Self::Output {
        let cden = lcm(self.den, rhs.den);
        let num = self.num * (cden / self.den) as i64;
        let rhs_num = rhs.num * (cden / rhs.den) as i64;
        Self::new(num + rhs_num, cden)
    }
}

impl std::ops::AddAssign for Fract {
    fn add_assign(&mut self, rhs: Fract) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Fract {
    type Output = Fract;

    fn sub(self, rhs: Fract) -> Self::Output {
        let cden = lcm(self.den, rhs.den);
        let num = self.num * (cden / self.den) as i64;
        let rhs_num = rhs.num * (cden / rhs.den) as i64;
        Self::new(num - rhs_num, cden)
    }
}

impl std::ops::SubAssign for Fract {
    fn sub_assign(&mut self, rhs: Fract) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Fract {
    type Output = Fract;

    fn mul(self, rhs: Fract) -> Self::Output {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl std::ops::MulAssign for Fract {
    fn mul_assign(&mut self, rhs: Fract) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Fract {
    type Output = Fract;

    fn div(self, rhs: Fract) -> Self::Output {
        assert!(rhs.num != 0, "Division by zero!");
        Self::new(self.num * rhs.den as i64, self.den as i64 * rhs.num)
    }
}

impl std::ops::DivAssign for Fract {
    fn div_assign(&mut self, rhs: Fract) {
        *self = *self / rhs;
    }
}
