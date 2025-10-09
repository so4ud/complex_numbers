use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/// `a` + `b` * `j`, where `j` is sqrt(-1)
#[derive(Clone, Copy, Debug)]
pub struct CompNum {
    pub a: f64,
    pub b: f64,
}
impl CompNum {
    pub fn from(a: f64, b: f64) -> Self {
        Self { a, b }
    }
    pub fn colapse(&self) -> Option<f64> {
        if self.b == 0.0 { Some(self.a) } else { None }
    }
}
impl Display for CompNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}j", self.a, self.b)
    }
}
impl Add<Self> for CompNum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let a = self.a + other.a;
        let b = self.b + other.b;

        Self { a, b }
    }
}
impl Add<f64> for CompNum {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            a: self.a + other,
            b: self.b,
        }
    }
}
impl Sub<Self> for CompNum {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a = self.a - other.a;
        let b = self.b - other.b;

        Self { a, b }
    }
}
impl Sub<f64> for CompNum {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        let a = self.a - other;
        let b = self.b;

        Self { a, b }
    }
}
impl Mul<Self> for CompNum {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (a, b) = (self.a, self.b);
        let (c, d) = (other.a, other.b);

        let first = Self::from(a * c, a * d);
        let second = Self::from(-(b * d), b * c);
        let result = first + second;

        return result;
    }
}
impl Mul<f64> for CompNum {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let (a, b) = (self.a * other, self.b * other);

        Self { a, b }
    }
}
impl Div<Self> for CompNum {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let (a, b) = (self.a, self.b);
        let (c, d) = (other.a, other.b);

        let temp_a = (a * c + b * d) / (c * c + d * d);
        let temp_b = (b * c - a * d) / (c * c + d * d);
        let result = Self::from(temp_a, temp_b);

        return result;
    }
}
impl Div<f64> for CompNum {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        let (a, b) = (self.a / other, self.b / other);

        Self { a, b }
    }
}

// fn main() {
//     let num1 = CompNum::from(1.0, 2.0); // 1 + 2j
//     let num2 = CompNum::from(2.0, 3.0); // 3 + 4j
//
//     println!("{} / {} = {}", num1.clone(), 2.0, num1.clone() / 2.0);
// }
