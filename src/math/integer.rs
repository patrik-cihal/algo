use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Rem, Sub},
};

pub trait Integer:
    Sized
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + PartialEq
    + Eq
    + Ord
    + PartialOrd
    + Copy
    + Clone
    + Display
    + Debug
{
    const ONE: Self;
    const ZERO: Self;

    fn from_i64(val: i64) -> Self;
    fn to_i64(self) -> i64;
}

macro_rules! impl_integer {
    ($t: ty) => {
        impl Integer for $t {
            const ONE: Self = 1;
            const ZERO: Self = 0;

            fn from_i64(val: i64) -> Self {
                val as $t
            }
            fn to_i64(self) -> i64 {
                self as i64
            }
        }
    };
}

impl_integer!(usize);
impl_integer!(isize);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(u32);
impl_integer!(u64);
