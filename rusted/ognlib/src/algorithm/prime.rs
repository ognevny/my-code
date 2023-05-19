/// Wilson's theory.
/// From [Wikipedia](https://en.wikipedia.org/wiki/Wilson%27s_theorem):
/// "Wilson's theorem states that a natural number n > 1 is a prime
/// number if and only if the product of all the positive integers
/// less than n is one less than a multiple of n. That is the factorial
/// (n - 1)! satisfies (n - 1)! % n == -1."
/// 
/// # Examples
/// ```
/// use ognlib::algorithm::prime::wilson_th;
/// 
/// assert_eq!(wilson_th(13), true);
/// assert_eq!(wilson_th(444), false);
/// ```

pub fn wilson_th(n: usize) -> bool {
    use num_bigint::BigInt;

    fn factorial(n: usize) -> BigInt {
        if n == 0 {
            BigInt::from(1u8)
        } else {
            BigInt::from(n) * factorial(n - 1)
        }
    }
    (factorial(n - 1) % BigInt::from(n)) - BigInt::from(n) == BigInt::from(-1)
}
