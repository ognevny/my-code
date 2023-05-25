//! Primality tests. These tests are divided into 2 major
//! groups: first group of tests gives exact results; second
//! group is for probabilistic tests, which can only suppose
//! whether number is probably prime or not.
//! This code uses enum of 3: Prime, NotPrime and ProbablyPrime.

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct PrimeStatusError {
    message: String,
}

impl PrimeStatusError {
    fn new() -> Self {
        Self {
            message: String::from("This number is neither prime nor composite"),
        }
    }
}

impl fmt::Display for PrimeStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PrimeStatusError {}

#[derive(Debug, PartialEq)]
pub enum PrimeStatus {
    Prime,
    Composite,
    ProbablyPrime,
}

/// Simple prime test. Takes ceil of sqrt(n) as upper bound and
/// checks if there is any divisor from 3 to ceil.
///
/// # Examples
/// ```
/// use ognlib::algorithm::prime::*;
///
/// assert_eq!(sqrtest(13).unwrap(), PrimeStatus::Prime);
/// assert_eq!(sqrtest(444).unwrap(), PrimeStatus::Composite);
/// ```

pub fn sqrtest(n: isize) -> Result<PrimeStatus, PrimeStatusError> {
    if n < 2 {
        return Err(PrimeStatusError::new());
    } else if n == 2 {
        return Ok(PrimeStatus::Prime);
    } else if n % 2 == 0 {
        return Ok(PrimeStatus::Composite);
    } else {
        let sqrt = (n as f64).sqrt().ceil() as usize;
        for i in (3..=sqrt).step_by(2) {
            if n as usize % i == 0 {
                return Ok(PrimeStatus::Composite);
            }
        }
    }
    Ok(PrimeStatus::Prime)
}

/// Wilson's theory.
/// From [Wikipedia](https://en.wikipedia.org/wiki/Wilson%27s_theorem):
/// "Wilson's theorem states that a natural number n > 1 is a prime
/// number if and only if the product of all the positive integers
/// less than n is one less than a multiple of n. That is the factorial
/// (n - 1)! satisfies (n - 1)! % n == -1."
///
/// # Examples
/// ```
/// use ognlib::algorithm::prime::*;
///
/// assert_eq!(sqrtest(13).unwrap(), PrimeStatus::Prime);
/// assert_eq!(sqrtest(444).unwrap(), PrimeStatus::Composite);
/// ```

pub fn wilson_th(n: isize) -> Result<PrimeStatus, PrimeStatusError> {
    use num_bigint::BigInt;

    if n < 2 {
        return Err(PrimeStatusError::new());
    }
    fn factorial(n: isize) -> BigInt {
        match n {
            0 | 1 => BigInt::from(1),
            _ => BigInt::from(n) * factorial(n - 1),
        }
    }
    if (factorial(n - 1) % BigInt::from(n)) - BigInt::from(n) == BigInt::from(-1) {
        Ok(PrimeStatus::Prime)
    } else {
        Ok(PrimeStatus::Composite)
    }
}

/// Miller-Rabin's prime test.
/// From [Wikipedia](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test):
/// the Miller–Rabin primality test or Rabin–Miller primality test is a probabilistic primality test:
/// an algorithm which determines whether a given number is likely to be prime.
/// # Examples
/// ```
/// use ognlib::algorithm::prime::*;
///
/// assert_eq!(sqrtest(13).unwrap(), PrimeStatus::Prime);
/// assert_eq!(sqrtest(444).unwrap(), PrimeStatus::Composite);
/// ```

pub fn miller_rabin(n: isize) -> Result<PrimeStatus, PrimeStatusError> {
    if n < 2 {
        return Err(PrimeStatusError::new());
    } else if n == 2 || n == 3 || n == 5 {
        return Ok(PrimeStatus::Prime);
    } else if n % 2 == 0 || n % 3 == 0 {
        return Ok(PrimeStatus::Composite);
    } else {
        use crate::num::power::modpow;
        use rand::Rng;

        let k = (n as f64).log2().ceil() as isize;
        let (k, mut t, mut s) = (k * k, n - 1, 0);
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let mut rng = rand::thread_rng();
        for _ in 0..k {
            let a = rng.gen_range(2..n - 1);

            let mut x = modpow(a, t as usize, n);
            if x == 1 || x == n - 1 {
                continue;
            }
            for _ in 0..s - 1 {
                x = modpow(x, 2, n);
                if x == 1 {
                    return Ok(PrimeStatus::Composite);
                }
                if x == n - 1 {
                    break;
                }
            }
        }
    }
    Ok(PrimeStatus::ProbablyPrime)
}
