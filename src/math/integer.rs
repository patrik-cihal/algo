use std::ops::{Mul, Div, Rem, Add, Sub};

pub trait Integer: Sized + Mul<Self, Output=Self> + Div<Self, Output=Self>+ Rem<Self, Output=Self> + Add<Self, Output=Self> + Sub<Self, Output=Self> + PartialEq + Eq + Ord + PartialOrd + Copy + Clone {
    const ONE: Self;
    const ZERO: Self;
}

macro_rules! impl_integer {
    ($t: ty) => {
        impl Integer for $t {
            const ONE: Self = 1;
            const ZERO: Self = 0;
        }
    };
}

impl_integer!(usize);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(u32);
impl_integer!(u64);