//! Fraction

use std::ops::{Mul, Add, Sub, Div, MulAssign, AddAssign, SubAssign, DivAssign};
use std::convert::TryInto;
use std::cmp::Ordering;
use std::{fmt, str::FromStr};

/// (Unsigned) Fraction of a measure.
#[derive(Copy, Clone, Debug)]
pub struct Fraction {
    pub num: u16,
    pub den: u16,
}

impl Fraction {
    /// Create a new fraction of a measure from a tuple.
    pub fn new(num: u16, den: u16) -> Self {
        assert_ne!(den, 0);
        Self { num, den }
    }

    /// Reciprocal (1 / self).
    pub fn recip(self) -> Self {
        Self { num: self.den, den: self.num }
    }

    /// Simpify the fraction (2/2) => (1/1).
    pub fn simplify(self) -> Self {
        let a = gcd_i(self.num, self.den);

        Self { num: self.num / a, den: self.den / a }
    }
}

impl Mul<i32> for Fraction {
    type Output = i32;

    fn mul(self, other: i32) -> Self::Output {
        let num = f32::from(self.num);
        let den = f32::from(self.den);
        (other as f32 * num * den.recip()) as i32
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Self::Output {
        let mut num: u32 = self.num.into();
        let mut den: u32 = self.den.into();
        let other_num: u32 = other.num.into();
        let other_den: u32 = other.den.into();

        num *= other_num;
        den *= other_den;

        let gcd = gcd_i(num, den);

        Fraction {
            num: (num / gcd).try_into().unwrap_or(0),
            den: (den / gcd).try_into().unwrap_or(0),
        }
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Self::Output {
        self * other.recip()
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Self::Output {
        if self.num == 0 {
            return other;
        }

        let (self_mul, other_mul, den) = if self.den % other.den == 0 {
            (1, self.den / other.den, self.den.into())
        } else if other.den % self.den == 0 {
            (other.den / self.den, 1, other.den.into())
        } else {
            (other.den, self.den, self.den * other.den)
        };

        let num: u32 = self.num as u32 * self_mul as u32 + other.num as u32 * other_mul as u32;
        let den: u32 = den.into();
        let gcd: u32 = gcd_i(num, den);
        Fraction {
            num: (num / gcd).try_into().unwrap_or_else(|_| {panic!("n {} {} {}", self, other, num/gcd)}),
            den: (den / gcd).try_into().unwrap_or_else(|_| {panic!("d {} {} {}", self, other, den/gcd)}),
        }
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Self::Output {
        let (self_mul, other_mul, den) = if self.den % other.den == 0 {
            (1, self.den / other.den, self.den)
        } else if other.den % self.den == 0 {
            (other.den / self.den, 1, other.den)
        } else {
            (other.den, self.den, self.den * other.den)
        };

        let num = self.num * self_mul - other.num * other_mul;
        let gcd = gcd_i(num, den);
        Fraction {
            num: num / gcd,
            den: den / gcd,
        }
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let simple = self.simplify();
        let other_simple = other.simplify();

        simple.den == other_simple.den
            && simple.num == other_simple.num
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_int = self.num as i32 * other.den as i32;
        let other_int = other.num as i32 * self.den as i32;

        (self_int-other_int).partial_cmp(&0)
    }
}

impl FromStr for Fraction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('/');
        let num = (iter.next().ok_or(())?).parse::<u16>().or(Err(()))?;
        let den = (iter.next().ok_or(())?).parse::<u16>().or(Err(()))?;

        if iter.next().is_some() { // Too many `/`s
            return Err(())
        }

        Ok(Fraction { num, den })
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

pub trait IsZero {
    fn is_zero(self) -> bool;
}

impl IsZero for u8 {
    fn is_zero(self) -> bool {
        self == 0
    }
}

impl IsZero for u16 {
    fn is_zero(self) -> bool {
        self == 0
    }
}


impl IsZero for u32 {
    fn is_zero(self) -> bool {
        self == 0
    }
}


impl IsZero for u64 {
    fn is_zero(self) -> bool {
        self == 0
    }
}


impl IsZero for u128 {
    fn is_zero(self) -> bool {
        self == 0
    }
}

impl IsZero for Fraction {
    fn is_zero(self) -> bool {
        self.num == 0 && self.den != 0
    }
}

// Iterative Greatest Common Divisor.
fn gcd_i<T>(mut a: T, mut b: T) -> T
    where T: PartialEq + std::ops::RemAssign + IsZero + Copy + Clone
{
    if a.is_zero() {
        return b;
    } else if b.is_zero() {
        return a;
    }

    loop {
        a %= b;
        if a.is_zero() {
            return b;
        }
        b %= a;
        if b.is_zero() {
            return a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_zero() {
        assert_eq!(Fraction::new(3, 8) + Fraction::new(0, 1), Fraction::new(3, 8));
        assert_eq!(Fraction::new(0, 1) + Fraction::new(3, 8), Fraction::new(3, 8));
    }

    #[test]
    fn sub_zero() {
        assert_eq!(Fraction::new(3, 8) - Fraction::new(0, 1), Fraction::new(3, 8));
    }

    #[test]
    fn add() {
        assert_eq!(Fraction::new(1, 2) + Fraction::new(3, 4), Fraction::new(5, 4));
        assert_eq!(Fraction::new(1, 8) + Fraction::new(1, 2), Fraction::new(5, 8));
        assert_eq!(Fraction::new(1, 1) + Fraction::new(10, 1), Fraction::new(11, 1));
        assert_eq!(Fraction::new(1, 3) + Fraction::new(1, 5), Fraction::new(8, 15));
        assert_eq!(Fraction::new(4, 4) + Fraction::new(2, 4), Fraction::new(3, 2));
    }

    #[test]
    fn sub() {
        assert_eq!(Fraction::new(5, 4) - Fraction::new(1, 2), Fraction::new(3, 4));
        assert_eq!(Fraction::new(1, 1) - Fraction::new(1, 64), Fraction::new(63, 64));
    }

    #[test]
    fn div() {
        assert_eq!(Fraction::new(1, 2) / Fraction::new(3, 4), Fraction::new(2, 3));
    }

    #[test]
    fn mul() {
        assert_eq!(Fraction::new(1, 2) * Fraction::new(3, 4), Fraction::new(3, 8));
    }

    #[test]
    fn simp() {
        assert_eq!(Fraction::new(4, 6).simplify(), Fraction::new(2, 3));
    }

    #[test]
    fn mult() {
        assert_eq!(0, Fraction::new(0, 1) * 32000);
    }

    #[test]
    fn more() {
        assert!(Fraction::new(50, 25) > Fraction::new(99, 50));
        assert!(Fraction::new(3, 4) > Fraction::new(1, 2));
        assert!(Fraction::new(1, 3) > Fraction::new(1, 4));
        assert_eq!(false, Fraction::new(0, 3) > Fraction::new(0, 4));
    }
}
