mod number;

use std::collections::{HashMap, HashSet};

use self::number::Number;

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

    fn evaluate(&self, x: Number) -> Number {
        self.coefficient * (x.pow(self.degree))
    }
}

struct PolynomialFunction {
    terms: Vec<Term>,
}

impl PolynomialFunction {
    fn new(terms: Vec<Term>) -> Self {
        let mut out = Self { terms };
        out.simplify();
        out
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

        self.terms.sort_by(|x, y| y.degree.cmp(&x.degree));
    }

    // Change x to Number?
    pub fn evaluate(&self, x: f64) -> f64 {
        self.terms
            .iter()
            .fold(0 as f64, |acc, t| (t.evaluate(x.into())) + acc)
    }

    fn _evaluate(&self, x: Number) -> Number {
        self.terms
            .iter()
            .fold(0.into(), |acc, t| t.evaluate(x) + acc)
    }

    fn roots(&self) -> Vec<Number> {
        let mut roots = HashSet::new();

        let constant_term = self
            .terms
            .last()
            .map(|x| {
                if x.degree == 0 {
                    x.coefficient
                } else {
                    1.into()
                }
            })
            .unwrap_or(1.into());

        let leading_coefficient = self
            .terms
            .first()
            .map(|x| {
                if x.degree == 0 {
                    x.coefficient
                } else {
                    1.into()
                }
            })
            .unwrap_or(1.into());
        if !constant_term.is_integer() || !leading_coefficient.is_integer() {
            return unimplemented!();
        }

        let lc_factors = leading_coefficient.factors();

        println!("{lc_factors:?}");

        let potential_roots = constant_term
            .factors()
            .into_iter()
            .flat_map(|c| lc_factors.iter().map(move |l| Number::new(c, *l)));

        for x in potential_roots {
            if self._evaluate(x) == 0.into() {
                roots.insert(x);
            }
        }

        roots.into_iter().collect()
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

        let func = PolynomialFunction::new(terms);

        assert_eq!(
            func.terms,
            vec![
                Term::new(Number::new(2, 1), 2),
                Term::new(Number::new(6, 1), 1),
                Term::new(Number::new(15, 1), 0),
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

        let func = PolynomialFunction::new(terms);

        assert_eq!(func.evaluate(15 as f64), 555 as f64);
    }

    #[test]
    fn find_roots() {
        let terms = vec![
            Term::new(Number::new(1, 1), 2),
            Term::new(Number::new(-5, 1), 1),
            Term::new(Number::new(6, 1), 0),
        ];

        let func = PolynomialFunction::new(terms);
        let mut roots = func.roots();
        roots.sort();

        assert_eq!(roots, vec![Number::new(2, 1), Number::new(3, 1)])
    }
}
