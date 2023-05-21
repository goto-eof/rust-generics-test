use std::ops::{Add, Div, Mul, Sub};
pub fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn mul<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

pub fn div<T: Div<Output = T>>(a: T, b: T) -> T {
    a / b
}

pub fn sub<T: Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}
