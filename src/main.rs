use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/// `a` + `b` * `j`, where `j` is sqrt(-1)
#[derive(Clone, Copy, Debug)]
struct CompNum {
    a: f64,
    b: f64,
}
impl CompNum {
    fn from(a: f64, b: f64) -> Self {
        Self { a, b }
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

fn main() {
    let num1 = CompNum::from(1.0, 2.0); // 1 + 2j
    let num2 = CompNum::from(2.0, 3.0); // 3 + 4j

    println!("{}", num1.clone() / num2.clone());
    println!("{}", num1.clone() + 1.0);
}
