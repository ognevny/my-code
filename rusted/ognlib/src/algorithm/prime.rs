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
    fn new(message: &str) -> PrimeStatusError {
        PrimeStatusError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PrimeStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PrimeStatusError {}

pub enum PrimeStatus {
    Prime,
    NotPrime,
    ProbablyPrime,
}

/// Simple prime test. Takes ceil of sqrt(n) as upper bound and
/// checks if there is any divisor from 3 to ceil.
///
/// # Examples
/// ```
/// use ognlib::algorithm::prime::*;
///
/// let (prime1, prime2);
/// if let Ok(PrimeStatus::Prime) = sqrtest(13) { prime1 = true; }
/// else { prime1 = false; };
///
/// if let Ok(PrimeStatus::NotPrime) = sqrtest(444) { prime2 = false; }
/// else { prime2 = true; };
///
/// assert!(prime1);
/// assert!(!prime2);
/// ```

pub fn sqrtest(n: isize) -> Result<PrimeStatus, PrimeStatusError> {
    if n < 2 {
        return Err(PrimeStatusError::new(
            "This number is neither prime nor not prime",
        ));
    } else if n == 2 {
        return Ok(PrimeStatus::Prime);
    } else if n % 2 == 0 {
        return Ok(PrimeStatus::NotPrime);
    } else {
        let sqrt = (n as f64).sqrt().ceil() as usize;
        for i in (3..=sqrt).step_by(2) {
            if n as usize % i == 0 {
                return Ok(PrimeStatus::NotPrime);
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
/// let (prime1, prime2);
/// if let Ok(PrimeStatus::Prime) = wilson_th(13) { prime1 = true; }
/// else { prime1 = false; };
///
/// if let Ok(PrimeStatus::NotPrime) = wilson_th(444) { prime2 = false; }
/// else { prime2 = true; };
///
/// assert!(prime1);
/// assert!(!prime2);
/// ```

pub fn wilson_th(n: isize) -> Result<PrimeStatus, PrimeStatusError> {
    use num_bigint::BigInt;

    if n < 2 {
        return Err(PrimeStatusError::new(
            "This number is neither prime nor not prime",
        ));
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
        Ok(PrimeStatus::NotPrime)
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
/// let (prime1, prime2);
/// if let Ok(PrimeStatus::ProbablyPrime) = miller_rabin(13) { prime1 = true; }
/// else { prime1 = false; };
///
/// if let Ok(PrimeStatus::NotPrime) = miller_rabin(444) { prime2 = false; }
/// else { prime2 = true; };
///
/// assert!(prime1);
/// assert!(!prime2);
/// ```

pub fn miller_rabin(n: usize) -> Result<PrimeStatus, PrimeStatusError> {
    if n < 2 {
        return Err(PrimeStatusError::new(
            "This number is neither prime nor not prime",
        ));
    } else if n == 2 || n == 3 || n == 5 {
        return Ok(PrimeStatus::Prime);
    } else if n % 2 == 0 || n % 3 == 0 {
        return Ok(PrimeStatus::NotPrime);
    } else {
        use num_bigint::BigInt;
        use rand::Rng;

        let k = (n as f64).log2().ceil() as isize;
        let (k, mut t, mut s) = (k * k, n - 1, 0);
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let mut rng = rand::thread_rng();
        for _ in 0..k {
            let a = BigInt::from(rng.gen_range(2..n - 1));

            let mut x = a.modpow(&BigInt::from(t), &BigInt::from(n));
            if x == BigInt::from(1) || x == BigInt::from(n - 1) {
                continue;
            }
            for _ in 0..s - 1 {
                x = x.modpow(&BigInt::from(2), &BigUint::from(n));
                if x == BigInt::from(1) {
                    return Ok(PrimeStatus::NotPrime);
                }
                if x == BigInt::from(n - 1) {
                    break;
                }
            }
        }
    }
    Ok(PrimeStatus::ProbablyPrime)
}
