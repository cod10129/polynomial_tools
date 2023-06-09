//! Functions for operations on polynomials.

fn plus_minus(val: f64, string: String) -> String {
  if val > 0.0 {
    format!(" + {string}")
  } else if val < 0.0 {
    format!(" - {string}")
  } else {
    string
  }
}

fn signed_val(val: f64, suffix: String) -> String {
  if val.abs() == 1.0 {
    plus_minus(val, suffix)
  } else if val != 0.0 { 
    let unsigned = format!("{}{suffix}", val.abs());
    plus_minus(val, unsigned)
  } else {
    String::new()
  }
}

fn s_val_last(val: f64) -> String {
  if val == 0.0 {
    return String::new()
  }
  plus_minus(val, val.abs().to_string())
}

fn display_header(val: f64, suffix: String) -> String {
  if val == 0.0 {
    String::new()
  } else if val == 1.0 {
    format!("{suffix}")
  } else {
    format!("{val}{suffix}")
  }
}

fn larger<T: std::cmp::PartialOrd>(first: T, second: T) -> T {
  if first > second {
    first
  } else {
    second
  }
}

fn inbounds<T>(index: usize, vec: &Vec<T>) -> bool {
  index < vec.len()
}

/// A struct that represents any polynomial.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralPolynomial {
  data: Vec<f64>
}

impl std::ops::Add<Self> for GeneralPolynomial {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    let size = larger(self.data.len(), other.data.len());
    let mut new = Vec::with_capacity(size);
    let mut to_push = 0.0;
    for i in 0..size {
      if inbounds(i, &self.data) && inbounds(i, &other.data) {
        to_push = self.data[i] + other.data[i];
      }
      else if inbounds(i, &self.data) && !inbounds(i, &other.data) { 
        to_push = self.data[i];
      } 
      else if inbounds(i, &other.data) && !inbounds(i, &self.data) {
        to_push = other.data[i];
      }
      new.push(to_push)
    }
    Self::new(new)
  }
}

impl std::ops::Sub<Self> for GeneralPolynomial {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    let size = larger(self.data.len(), other.data.len());
    let mut new = Vec::with_capacity(size);
    let mut to_push = 0.0;
    for i in 0..size {
      if inbounds(i, &self.data) && inbounds(i, &other.data) {
        to_push = self.data[i] - other.data[i];
      }
      else if inbounds(i, &self.data) && !inbounds(i, &other.data) { 
        to_push = self.data[i];
      } 
      else if inbounds(i, &other.data) && !inbounds(i, &self.data) {
        to_push = other.data[i];
      }
      new.push(to_push)
    }
    Self::new(new)
  }
}

// fmt::Display for GeneralPolynomial coming in 0.2.2

impl GeneralPolynomial {
  /// Creates a new GeneralPolynomial
  pub fn new(data: Vec<f64>) -> Self {
    Self { data }
  }

  /// Creates a new GeneralPolynomial from integers.
  pub fn new_i(data: Vec<i32>) -> Self {
    let mut new = Vec::with_capacity(data.len());
    for i in 0..data.len() {
      new.push(data[i].into());
    }
    Self { data: new }
  }
}

/// A trait providing methods for operations on polynomials.
pub trait Polynomial {
  /// Evaluates the polynomial for the given x.
  fn evaluate(&self, x: f64) -> f64;
  /// Returns true if the polynomial is a zero polynomial,
  /// false otherwise
  fn is_zero(&self) -> bool;
  /// Returns the degree of the polynomial.
  // Note: this is for functions that take <T: Polynomial>
  fn degree(&self) -> u8;
  /// The type returned by the derivative() function
  type Derivative: Polynomial;
  /// Returns the derivative of the polynomial.
  /// This derivative is returned in the form of another polynomial.
  fn derivative(&self) -> Self::Derivative;
}

impl Polynomial for f64 {
  #[allow(unused_variables)]
  fn evaluate(&self, x: f64) -> f64 {
    self.to_owned()
  }
  fn is_zero(&self) -> bool {
    self.to_owned() == 0.0
  }
  fn degree(&self) -> u8 { 0 }
  
  type Derivative = f64;
  fn derivative(&self) -> Self::Derivative {
    0.0
  }
}

mod linear;
mod quadratic;
mod cubic;
mod quartic;

pub use linear::Linear;
pub use quadratic::Quadratic;
pub use cubic::Cubic;
pub use quartic::Quartic;

#[cfg(test)]
mod tests {
  use super::*;

  mod gp {
    use super::GeneralPolynomial;

    #[test]
    fn add() {
      let gp1 = GeneralPolynomial::new(vec![-2.0, -3.0, 0.0, 4.0, 6.0]);
      let gp2 = GeneralPolynomial::new(vec![ 0.0, 4.0, -4.6, 16.0]);
      let gp3 = GeneralPolynomial::new(vec![-2.0, 1.0, -4.6, 20.0, 6.0]);
      assert_eq!(gp1 + gp2, gp3);
    }
  }
  
  mod f64 {
    use super::Polynomial;
    #[test]
    fn eval() {
      assert_eq!(1.0.evaluate(0.0), 1.0);
    }

    #[test]
    fn is_zero() {
      assert!(0.0.is_zero());
      assert!(!1.0.is_zero());
    }

    #[test]
    fn degree() {
      assert_eq!(0.0.degree(), 0);
    }
    
    #[test]
    fn derive() {
      assert_eq!(3.0.derivative(), 0.0);
    }
  }
  
  mod linear {
    use super::{Polynomial, Linear};
    #[test]
    fn eval() {
      let linear = Linear::new(2.0, 1.0);
      let result = linear.evaluate(3.0);
      assert_eq!(result, 7.0);
    }

    #[test]
    fn to_string() {
      let linear = Linear::new(2.3, 1.0);
      let result = format!("{linear}");
      assert_eq!(String::from("2.3x + 1"), result);
    }

    #[test]
    fn add_lin() {
      let l1 = Linear::new(1.0, 7.4);
      let l2 = Linear::new(1.5, 2.6);
      assert_eq!(l1 + l2, Linear::new(2.5, 10.0));
    }

    #[test]
    fn add_f64() {
      let l = Linear::new(1.0, 3.0);
      assert_eq!(l + 7.0, Linear::new(1.0, 10.0));
    }

    #[test]
    fn sub_lin() {
      let l1 = Linear::new(10.0, 5.0);
      let l2 = Linear::new(15.0, 3.0);
      assert_eq!(l1 - l2, Linear::new(-5.0, 2.0));
    }

    #[test]
    fn sub_f64() {
      let l = Linear::new(1.0, 7.0);
      assert_eq!(l - 5.0, Linear::new(1.0, 2.0));
    }

    #[test]
    fn mul_lin() {
      use super::Quadratic;
      let l = Linear::new(2.0, 3.0);
      let l2 = Linear::new(4.0, 5.0);
      assert_eq!(l * l2, Quadratic::new(8.0, 22.0, 15.0));
    }

    #[test]
    fn mul_f64() {
      let l = Linear::new(2.0, 3.0);
      assert_eq!(l * 4.0, Linear::new(8.0, 12.0));
    }
    
    #[test]
    fn degree() {
      let l = Linear::new(0.0, 0.0);
      assert_eq!(l.degree(), 1);
    }

    #[test]
    fn is_zero() {
      let l = Linear::new(0.0, 0.0);
      let l2 = Linear::new(1.0, 0.0);
      assert!(l.is_zero());
      assert!(!l2.is_zero());
    }

    #[test]
    fn root() {
      let l = Linear::new_i(1, 2);
      assert_eq!(l.root().unwrap(), -2.0);
    }

    #[test]
    fn derive() {
      assert_eq!(Linear::new_i(2, 3).derivative(), 2.0);
    }
  }

  mod quadratic {
    use super::{Polynomial, Quadratic};

    #[test]
    fn eval() {
      let q = Quadratic::new(2.0, 3.0, -1.0);
      assert_eq!(q.evaluate(5.0), 64.0);
    }

    #[test]
    fn to_string() {
      let q = Quadratic::new(-2.5, -1.0, 1.0);
      assert_eq!(q.to_string(), String::from("-2.5x^2 - x + 1"));
    }

    #[test]
    fn add_qad() {
      let q1 = Quadratic::new(3.6, 2.0, 1.0);
      let q2 = Quadratic::new(1.4, 3.0, 4.0);
      assert_eq!(q1 + q2, Quadratic::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn add_lin() {
      use super::Linear;
      let q = Quadratic::new(1.0, 2.0, 3.0);
      assert_eq!(q + Linear::new(4.0, 5.0), Quadratic::new(1.0, 6.0, 8.0));
    }

    #[test]
    fn add_f64() {
      let q = Quadratic::new(1.0, 2.0, 3.0);
      assert_eq!(q + 4.0, Quadratic::new(1.0, 2.0, 7.0));
    }
    
    #[test]
    fn sub_qad() {
      let q1 = Quadratic::new(-5.0, -5.0, -5.0);
      let q2 = Quadratic::new(5.0, 10.0, 20.0);
      assert_eq!(q1 - q2, Quadratic::new(-10.0, -15.0, -25.0));
    }

    #[test]
    fn sub_lin() {
      use super::Linear;
      let q = Quadratic::new(0.0, 1.0, 2.0);
      assert_eq!(q - Linear::new(3.0, 5.0), Quadratic::new(0.0, -2.0, -3.0));
    }

    #[test]
    fn sub_f64() {
      let q = Quadratic::new(0.0, 1.0, 2.0);
      assert_eq!(q - 5.0, Quadratic::new(0.0, 1.0, -3.0));
    }

    #[test]
    fn mul_qad() {
      use super::Quartic;
      let q1 = Quadratic::new_i(1, 2, 3);
      let q2 = Quadratic::new_i(4, 5, 6);
      assert_eq!(q1 * q2, Quartic::new_i(4, 13, 28, 27, 18));
    }
    
    #[test]
    fn mul_lin() {
      use super::{Linear, Cubic};
      let q = Quadratic::new(2.0, 3.0, 4.0);
      let l = Linear::new(5.0, 6.0);
      assert_eq!(q * l, Cubic::new(10.0, 27.0, 38.0, 24.0));
    }

    #[test]
    fn mul_f64() {
      let q = Quadratic::new(2.0, 3.0, 4.0);
      assert_eq!(q * 5.0, Quadratic::new(10.0, 15.0, 20.0));
    }
    
    #[test]
    fn degree() {
      let q = Quadratic::new(0.0, 0.0, 0.0);
      assert_eq!(q.degree(), 2);
    }

    #[test]
    fn is_zero() {
      let q = Quadratic::new(0.0, 0.0, 0.0);
      let q2 = Quadratic::new(0.0, 1.0, 0.0);
      assert!(q.is_zero());
      assert!(!q2.is_zero());
    }

    #[test]
    fn roots() {
      let result = Quadratic::new_i(1, 4, 3).roots().unwrap();
      assert_eq!(result, [-1.0, -3.0]);
    }

    #[test]
    fn derive() {
      use super::Linear;
      let q = Quadratic::new_i(3, 2, 1);
      assert_eq!(q.derivative(), Linear::new_i(6, 2));
    }
  }

  mod cubic {
    use super::{Polynomial, Cubic};

    static BC: Cubic = Cubic {
      a: 1.0,
      b: 2.0,
      c: 3.0,
      d: 4.0
    };
    
    #[test]
    fn eval() {
      let c = BC;
      let result = c.evaluate(5.0);
      assert_eq!(result, 125.0 + 50.0 + 15.0 + 4.0);
    }

    #[test]
    fn to_string() {
      let c = BC;
      assert_eq!(String::from("x^3 + 2x^2 + 3x + 4"), c.to_string());
    }

    #[test]
    fn add_cub() {
      let c1 = BC;
      let c2 = Cubic::new(8.0, 7.0, 6.0, 5.0);
      assert_eq!(c1 + c2, Cubic::new(9.0, 9.0, 9.0, 9.0));
    }

    #[test]
    fn add_qad() {
      use super::Quadratic;
      let c = BC;
      let q = Quadratic::new(5.0, 6.0, 7.0);
      assert_eq!(c + q, Cubic::new(1.0, 7.0, 9.0, 11.0));
    }

    #[test]
    fn add_lin() {
      use super::Linear;
      let c = BC;
      let l = Linear::new(4.0, 5.0);
      assert_eq!(c + l, Cubic::new(1.0, 2.0, 7.0, 9.0));
    }

    #[test]
    fn add_f64() {
      let c = BC;
      assert_eq!(c + 5.0, Cubic::new(1.0, 2.0, 3.0, 9.0));
    }
    
    #[test]
    fn sub_cub() {
      let c1 = BC;
      let c2 = Cubic::new(1.0, 7.0, 5.5, -6.0);
      assert_eq!(c1 - c2, Cubic::new(0.0, -5.0, -2.5, 10.0));
    }

    #[test]
    fn sub_qad() {
      use super::Quadratic;
      let c = BC;
      let q = Quadratic::new(5.0, 6.0, 7.0);
      assert_eq!(c - q, Cubic::new(1.0, -3.0, -3.0, -3.0));
    }

    #[test]
    fn sub_lin() {
      use super::Linear;
      let c = BC;
      let l = Linear::new(4.0, 5.0);
      assert_eq!(c - l, Cubic::new(1.0, 2.0, -1.0, -1.0));
    }

    #[test]
    fn sub_f64() {
      let c = BC;
      assert_eq!(c - 5.0, Cubic::new(1.0, 2.0, 3.0, -1.0));
    }

    #[test]
    fn mul_lin() {
      use super::{Linear, Quartic};
      let l = Linear::new_i(5, 6);
      assert_eq!(BC * l, Quartic::new_i(5, 16, 27, 38, 24));
    }

    #[test]
    fn mul_f64() {
      let c = BC;
      assert_eq!(c * 5.0, Cubic::new(5.0, 10.0, 15.0, 20.0));
    }
    
    #[test]
    fn degree() {
      assert_eq!(BC.degree(), 3);
    }

    #[test]
    fn is_zero() {
      let c = Cubic::new(0.0, 0.0, 0.0, 0.0);
      assert!(c.is_zero());
      assert!(!BC.is_zero());
    }

    #[test]
    fn derive() {
      use super::Quadratic;
      assert_eq!(BC.derivative(), Quadratic::new_i(3, 4, 3));
    }
  }

  mod quartic {
    use super::{Polynomial, Quartic};

    static BQ: Quartic = Quartic {
      a: 1.0,
      b: 2.0,
      c: 3.0,
      d: 4.0,
      e: 5.0
    };

    #[test]
    fn eval() {
      assert_eq!(BQ.evaluate(6.0), 1296.0 + 432.0 + 108.0 + 24.0 + 5.0);
    }

    #[test]
    fn to_string() {
      assert_eq!(BQ.to_string(), String::from("x^4 + 2x^3 + 3x^2 + 4x + 5"));
    }

    #[test]
    fn add_qrt() {
      let q = Quartic::new_i(6, 7, 8, 9, 10);
      assert_eq!(BQ + q, Quartic::new_i(7, 9, 11, 13, 15));
    }

    #[test]
    fn add_cub() {
      use super::Cubic;
      let c = Cubic::new_i(6, 7, 8, 9);
      assert_eq!(BQ + c, Quartic::new_i(1, 8, 10, 12, 14));
    }

    #[test]
    fn add_qad() {
      use super::Quadratic;
      let q = Quadratic::new_i(6, 7, 8);
      assert_eq!(BQ + q, Quartic::new_i(1, 2, 9, 11, 13));
    }

    #[test]
    fn add_lin() {
      use super::Linear;
      let l = Linear::new_i(6, 7);
      assert_eq!(BQ + l, Quartic::new_i(1, 2, 3, 10, 12));
    }

    #[test]
    fn add_f64() {
      assert_eq!(BQ + 6.0, Quartic::new_i(1, 2, 3, 4, 11));
    }

    #[test]
    fn sub_qrt() {
      let q = Quartic::new_i(6, 7, 8, 9, 10);
      assert_eq!(BQ - q, Quartic::new_i(-5, -5, -5, -5, -5));
    }

    #[test]
    fn sub_cub() {
      use super::Cubic;
      let c = Cubic::new_i(6, 7, 8, 9);
      assert_eq!(BQ - c, Quartic::new_i(1, -4, -4, -4, -4));
    }

    #[test]
    fn sub_qad() {
      use super::Quadratic;
      let q = Quadratic::new_i(6, 7, 8);
      assert_eq!(BQ - q, Quartic::new_i(1, 2, -3, -3, -3));
    }

    #[test]
    fn sub_lin() {
      use super::Linear;
      let l = Linear::new_i(6, 7);
      assert_eq!(BQ - l, Quartic::new_i(1, 2, 3, -2, -2));
    }

    #[test]
    fn mul_f64() {
      assert_eq!(BQ * 6.0, Quartic::new_i(6, 12, 18, 24, 30));
    }

    #[test]
    fn sub_f64() {
      assert_eq!(BQ - 6.0, Quartic::new_i(1, 2, 3, 4, -1));
    }
    
    #[test]
    fn degree() {
      assert_eq!(BQ.degree(), 4);
    }

    #[test]
    fn is_zero() {
      let q = Quartic::new_i(0, 0, 0, 0, 0);
      assert!(q.is_zero());
      assert!(!BQ.is_zero());
    }

    #[test]
    fn derive() {
      use super::Cubic;
      let c = Cubic::new_i(4, 6, 6, 4);
      assert_eq!(BQ.derivative(), c);
    }
  }
}
