use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Number {
    numerator: i32,
    denominator: i32,
}

impl Number {
    fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn simplify(&mut self) {
        let fac = gcd(self.numerator, self.denominator);
        self.numerator /= fac;
        self.denominator /= fac;
    }

    fn not_zero(&self) -> bool {
        self.numerator != 0
    }
}

impl Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        let mut number = Number {
            numerator: (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator),
            denominator: rhs.denominator * self.denominator,
        };
        number.simplify();
        number
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        let mut number = Number {
            numerator: (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator),
            denominator: rhs.denominator * self.denominator,
        };
        number.simplify();
        *self = number;
    }
}

impl Mul<f64> for Number {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        (self.numerator as f64 * rhs) / self.denominator as f64
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Term {
    coefficient: Number,
    degree: i32,
}

impl Term {
    fn new(coefficient: Number, degree: i32) -> Self {
        Self {
            coefficient,
            degree,
        }
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * (x.powf(self.degree as f64))
    }
}

struct PolynomialFunction {
    terms: Vec<Term>,
}

impl PolynomialFunction {
    fn new(terms: Vec<Term>) -> Self {
        Self { terms }
    }

    fn simplify(&mut self) {
        let mut map: HashMap<i32, Term> = HashMap::new();

        for mut t in std::mem::take(&mut self.terms).into_iter() {
            match map.get_mut(&t.degree) {
                Some(original_term) => original_term.coefficient += t.coefficient,
                None => {
                    t.coefficient.simplify();
                    map.insert(t.degree, t);
                }
            }
        }

        self.terms = map
            .into_iter()
            .map(|(_, t)| t)
            .filter(|t| t.coefficient.not_zero())
            .collect();

        self.terms.sort_by(|x, y| x.degree.cmp(&y.degree));
    }

    // Change x to Number?
    fn evaluate(&self, x: f64) -> f64 {
        self.terms
            .iter()
            .fold(0 as f64, |acc, t| acc + t.evaluate(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn poly_new() {
        assert_eq!(PolynomialFunction::new(Vec::new()).terms.len(), 0);
    }

    #[test]
    fn simplify() {
        let terms = vec![
            Term::new(Number::new(5, 1), 0),
            Term::new(Number::new(10, 1), 0),
            Term::new(Number::new(4, 2), 1),
            Term::new(Number::new(4, 1), 1),
            Term::new(Number::new(4, 2), 2),
            Term::new(Number::new(4, 2), 3),
            Term::new(Number::new(-4, 2), 3),
        ];

        let mut func = PolynomialFunction::new(terms);
        func.simplify();

        assert_eq!(
            func.terms,
            vec![
                Term::new(Number::new(15, 1), 0),
                Term::new(Number::new(6, 1), 1),
                Term::new(Number::new(2, 1), 2),
            ]
        );
    }

    #[test]
    fn evaluate() {
        let terms = vec![
            Term::new(Number::new(15, 1), 0),
            Term::new(Number::new(6, 1), 1),
            Term::new(Number::new(2, 1), 2),
        ];

        let mut func = PolynomialFunction::new(terms);

        assert_eq!(func.evaluate(15 as f64), 555 as f64);
    }
}
