use std::ops::{Add, AddAssign, Div, Mul};

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Number {
    pub numerator: u32,
    pub denominator: u32,
    positive: bool,
}

impl Number {
    pub fn new(numerator: u32, denominator: u32, sign: bool) -> Self {
        let mut out = Self {
            numerator,
            denominator,
            positive: sign,
        };
        out.simplify();
        out
    }

    pub fn simplify(&mut self) {
        let fac = gcd(self.numerator, self.denominator);
        self.numerator /= fac;
        self.denominator /= fac;
    }

    pub fn not_zero(&self) -> bool {
        self.numerator != 0
    }

    pub fn factors(&self) -> Vec<i32> {
        (1..=self.numerator / 2)
            .filter(|x| self.numerator % x == 0)
            .flat_map(|x| [x as i32, -(x as i32)])
            .chain([self.numerator as i32, -(self.numerator as i32)])
            .collect()
    }

    pub fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    pub fn pow(&self, degree: i32) -> Number {
        let mut numerator = 1;
        let mut denominator = 1;
        for _ in 0..degree {
            numerator *= self.numerator;
            denominator *= self.denominator
        }
        Number::new(
            numerator,
            denominator,
            if degree % 2 == 0 { true } else { self.positive },
        )
    }
}

impl Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        let lhs_factor = if self.positive { 1 } else { -1 };
        let rhs_factor = if rhs.positive { 1 } else { -1 };
        let numerator = ((self.numerator * rhs.denominator) as i32 * lhs_factor)
            + ((rhs.numerator * self.denominator) as i32 * rhs_factor);
        let denom = self.denominator * rhs.denominator;
        Number::new(numerator.abs() as u32, denom, numerator.is_positive())
    }
}

impl Add<f64> for Number {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        self.add(Into::<Number>::into(rhs)).into()
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        let lhs_factor = if self.positive { 1 } else { -1 };
        let rhs_factor = if rhs.positive { 1 } else { -1 };
        let numerator = ((self.numerator * rhs.denominator) as i32 * lhs_factor)
            + ((rhs.numerator * self.denominator) as i32 * rhs_factor);
        let denom = self.denominator * rhs.denominator;
        let number = Number::new(numerator.abs() as u32, denom, numerator.is_positive());
        *self = number;
    }
}

impl Mul<f64> for Number {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        (self.numerator as f64 * rhs) / self.denominator as f64
    }
}

impl Mul<u32> for Number {
    type Output = Number;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::new(self.numerator * rhs, self.denominator, self.positive)
    }
}

impl Div<f64> for Number {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        (self.numerator as f64) / (self.denominator as f64 * rhs)
    }
}

impl Div<u32> for Number {
    type Output = Number;

    fn div(self, rhs: u32) -> Self::Output {
        Self::new(self.numerator, self.denominator * rhs, self.positive)
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        let sign = match self.positive {
            true => rhs.positive,
            false => !rhs.positive,
        };
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
            sign,
        )
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
            self.positive ^ rhs.positive,
        )
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.numerator * other.denominator).cmp(&(other.numerator * self.denominator)))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::new(value, 1, true)
    }
}

impl Into<f64> for Number {
    fn into(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        let mut i = 0;
        let mut new_numerator = value;

        loop {
            if (new_numerator.floor() - new_numerator).abs() < f64::EPSILON {
                break;
            }
            i += 1;
            new_numerator = value * 10_f64.powi(i);
        }

        Self::new(
            new_numerator.floor().abs() as u32,
            (10 as u32).pow(i as u32),
            value.is_sign_positive(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simplify() {
        assert_eq!(Number::new(9, 3, true), Number::new(3, 1, true));
        assert_eq!(Number::new(8, 16, true), Number::new(1, 2, true));
        assert_eq!(Number::new(153, 3, true), Number::new(51, 1, true));
    }

    #[test]
    fn test_division() {}

    #[test]
    fn test_multiplication() {
        assert_eq!(Number::new(4, 1, true) * 4, Number::new(16, 1, true));
        assert_eq!(
            Number::new(4, 1, true) * Number::new(4, 1, true),
            Number::new(16, 1, true)
        );
        assert_eq!(
            Number::new(4, 1, false) * Number::new(4, 1, true),
            Number::new(16, 1, false)
        );
        assert_eq!(
            Number::new(4, 1, true) * Number::new(4, 1, false),
            Number::new(16, 1, false)
        );
        assert_eq!(
            Number::new(4, 2, true) * Number::new(3, 2, false),
            Number::new(3, 1, false)
        );
        assert_eq!(
            Number::new(2, 1, true) * Number::new(225, 1, true),
            Number::new(450, 1, true)
        );
    }

    #[test]
    fn test_pow() {
        assert_eq!(Number::new(4, 1, true).pow(2), Number::new(16, 1, true));
        assert_eq!(Number::new(1, 1, true).pow(2), Number::new(1, 1, true));
        assert_eq!(Number::new(4, 3, false).pow(3), Number::new(64, 27, false));
    }
}
