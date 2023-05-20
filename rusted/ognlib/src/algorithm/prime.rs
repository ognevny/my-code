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
/// if let PrimeStatus::Prime = sqrtest(13) { prime1 = true; }
/// else { prime1 = false; };
///
/// if let PrimeStatus::NotPrime = sqrtest(444) { prime2 = false; }
/// else { prime2 = true; };
///
/// assert!(prime1);
/// assert!(!prime2);
/// ```

pub fn sqrtest(n: usize) -> PrimeStatus {
    if n % 2 == 0 {
        return PrimeStatus::NotPrime;
    }
    let sqrt = (n as f64).sqrt().ceil() as usize;
    for i in 3..=sqrt {
        if n % i == 0 {
            return PrimeStatus::NotPrime;
        }
    }
    PrimeStatus::Prime
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

/// Miller-Rabbin's prime test.
///
/// # Examples
/// ```
/// use ognlib::algorithm::prime::*;
///
/// let (prime1, prime2);
/// if let PrimeStatus::ProbablyPrime = miller_rabbin(13) { prime1 = true; }
/// else { prime1 = false; };
///
/// if let PrimeStatus::NotPrime = miller_rabbin(444) { prime2 = false; }
/// else { prime2 = true; };
///
/// assert!(prime1);
/// assert!(!prime2);
/// ```

pub fn miller_rabbin(n: usize) -> PrimeStatus {
    if n == 2 || n == 3 || n == 5 {
        return PrimeStatus::Prime;
    } else if n % 2 == 0 || n % 3 == 0 {
        return PrimeStatus::NotPrime;
    } else {
        use num_bigint::BigUint;
        use rand::Rng;

        let k = (n as f64).log2().ceil() as usize;
        let (k, mut t, mut s) = (k * k, n - 1, 0);
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let mut rng = rand::thread_rng();

        let a: Vec<usize> = (0..k).map(|_| rng.gen_range(2..n - 1)).collect();
        for elem in a {
            let mut x = BigUint::from(elem).modpow(&BigUint::from(t), &BigUint::from(n));
            if x == BigUint::from(1u8) || x == BigUint::from(n - 1) {
                continue;
            }
            for _ in 0..s - 1 {
                x = x.modpow(&BigUint::from(2u8), &BigUint::from(n));
                if x == BigUint::from(1u8) {
                    return PrimeStatus::NotPrime;
                }
                if x == BigUint::from(n - 1) {
                    break;
                }
            }
        }
    }
    PrimeStatus::ProbablyPrime
}
