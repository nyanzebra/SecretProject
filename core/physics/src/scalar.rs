use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

pub trait Scalar:
    Copy
    + Default
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + PartialOrd
{
}

// integers
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
impl Scalar for isize {}

// unsigned integers
impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for u128 {}

// floats
impl Scalar for f32 {}
impl Scalar for f64 {}
