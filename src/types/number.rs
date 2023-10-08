use std::ops::{Add, AddAssign, Div, Mul};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Number {
    numerator: i32,
    denominator: i32,
}

impl Number {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        let mut out = Self {
            numerator,
            denominator,
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
            .flat_map(|x| [x, -x])
            .chain([self.numerator, -self.numerator])
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
        Number::new(numerator, denominator)
    }
}

impl Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        Number {
            numerator: (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator),
            denominator: rhs.denominator * self.denominator,
        }
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
        let number = Number {
            numerator: (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator),
            denominator: rhs.denominator * self.denominator,
        };
        *self = number;
    }
}

impl Mul<f64> for Number {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        (self.numerator as f64 * rhs) / self.denominator as f64
    }
}

impl Div<f64> for Number {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        (self.numerator as f64) / (self.denominator as f64 * rhs)
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

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

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
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

        Self {
            numerator: new_numerator.floor() as i32,
            denominator: (10 as i32).pow(i as u32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simplify() {
        assert_eq!(Number::new(9, 3), Number::new(3, 1));
        assert_eq!(Number::new(8, 16), Number::new(1, 2));
        assert_eq!(Number::new(153, 3), Number::new(51, 1));
    }

    #[test]
    fn test_division() {}

    #[test]
    fn test_pow() {
        assert_eq!(Number::new(4, 1).pow(2), Number::new(16, 1));
        assert_eq!(Number::new(1, 1).pow(2), Number::new(1, 1));
        assert_eq!(Number::new(-4, 3).pow(3), Number::new(-64, 27));
    }
}
