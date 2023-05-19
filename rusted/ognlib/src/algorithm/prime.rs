pub enum PrimeStatus {
    Prime,
    NotPrime,
    ProbablyPrime,
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
/// if let PrimeStatus::Prime = wilson_th(13) { prime1 = true; }
/// else { prime1 = false; };
/// 
/// if let PrimeStatus::NotPrime = wilson_th(444) { prime2 = false; }
/// else { prime2 = true; };
///
/// assert!(prime1);
/// assert!(!prime2);
/// ```

pub fn wilson_th(n: usize) -> PrimeStatus {
    use num_bigint::BigInt;

    fn factorial(n: usize) -> BigInt {
        if n == 0 {
            BigInt::from(1u8)
        } else {
            BigInt::from(n) * factorial(n - 1)
        }
    }
    if (factorial(n - 1) % BigInt::from(n)) - BigInt::from(n) == BigInt::from(-1) {
        PrimeStatus::Prime
    } else {
        PrimeStatus::NotPrime
    }
}
