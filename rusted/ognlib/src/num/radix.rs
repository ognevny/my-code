//! Structs and impls for radix numbers (String nums and int nums).
//! All numbers are unsigned ints.

// TODO: write more ariphmetic functions. error handling. tests

use std::cmp::Ordering;

pub const RADIX: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Radix {
    pub number: usize,
    pub radix: u8,
}

impl PartialOrd for Radix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        usize::from_str_radix(&self.number.to_string(), self.radix.into())
            .unwrap()
            .partial_cmp(
                &usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap(),
            )
    }
}

impl Ord for Radix {
    fn cmp(&self, other: &Self) -> Ordering {
        usize::from_str_radix(&self.number.to_string(), self.radix.into())
            .unwrap()
            .cmp(&usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap())
    }
}

impl std::ops::Add for Radix {
    type Output = Self;

    /// Performs a `+` operation
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    ///
    /// let res = (n1 + n2).to_radix(8);
    /// assert_eq!(res, Radix::from_radix(227, 8));
    /// ```

    fn add(self, other: Self) -> Self::Output {
        Self {
            number: Self::from(
                usize::from_str_radix(&self.number.to_string(), self.radix.into()).unwrap()
                    + usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap(),
            )
            .to_radix(self.radix)
            .number,
            radix: self.radix,
        }
    }
}

impl std::ops::AddAssign for Radix {
    /// Performs a `+=` operation
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let n2 = Radix::from_radix(444, 5);
    ///
    /// n1 += n2;
    /// n1 = n1.to_radix(8);
    /// assert_eq!(n1, Radix::from_radix(227, 8));
    /// ```

    fn add_assign(&mut self, other: Self) {
        self.number = Self::from(
            usize::from_str_radix(&self.number.to_string(), self.radix.into()).unwrap()
                + usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap(),
        )
        .to_radix(self.radix)
        .number;
    }
}

impl std::ops::Sub for Radix {
    type Output = Self;

    /// Performs a `-` operation
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    ///
    /// let res = (n1 - n2).to_radix(8);
    /// assert_eq!(res, Radix::from_radix(141, 8));
    /// ```

    fn sub(self, other: Self) -> Self::Output {
        if self > other {
            Self {
                number: Self::from(
                    usize::from_str_radix(&self.number.to_string(), self.radix.into()).unwrap()
                        - usize::from_str_radix(&other.number.to_string(), other.radix.into())
                            .unwrap(),
                )
                .to_radix(self.radix)
                .number,
                radix: self.radix,
            }
        } else {
            Self {
                number: Self::from(
                    usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap()
                        - usize::from_str_radix(&self.number.to_string(), self.radix.into())
                            .unwrap(),
                )
                .to_radix(other.radix)
                .number,
                radix: other.radix,
            }
        }
    }
}

impl std::ops::Mul for Radix {
    type Output = Self;

    /// Performs a `*` operation
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    ///
    /// let res = (n1 * n2).to_radix(8);
    /// assert_eq!(res, Radix::from_radix(6424, 8));
    /// ```

    fn mul(self, other: Self) -> Self::Output {
        Self {
            number: Self::from(
                usize::from_str_radix(&self.number.to_string(), self.radix.into()).unwrap()
                    * usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap(),
            )
            .to_radix(self.radix)
            .number,
            radix: self.radix,
        }
    }
}

impl std::ops::MulAssign for Radix {
    /// Performs a `*=` operation
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let n2 = Radix::from_radix(444, 5);
    ///
    /// n1 *= n2;
    /// n1 = n1.to_radix(8);
    /// 
    /// assert_eq!(n1, Radix::from_radix(6424, 8));
    /// ```

    fn mul_assign(&mut self, other: Self) {
        self.number = Self::from(
            usize::from_str_radix(&self.number.to_string(), self.radix.into()).unwrap()
                * usize::from_str_radix(&other.number.to_string(), other.radix.into()).unwrap(),
        )
        .to_radix(self.radix)
        .number;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringRadix {
    pub number: String,
    pub radix: u8,
}

impl PartialOrd for StringRadix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        usize::from_str_radix(&self.number, self.radix.into())
            .unwrap()
            .partial_cmp(&usize::from_str_radix(&other.number, other.radix.into()).unwrap())
    }
}

impl Ord for StringRadix {
    fn cmp(&self, other: &Self) -> Ordering {
        usize::from_str_radix(&self.number, self.radix.into())
            .unwrap()
            .cmp(&usize::from_str_radix(&other.number, other.radix.into()).unwrap())
    }
}

impl Radix {
    /// Creates a new `Radix`.
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let n = Radix::new(2);
    /// assert_eq!(n.number, 0);
    /// assert_eq!(n.radix, 2);
    /// ```

    pub fn new(k: u8) -> Self {
        Self {
            number: 0,
            radix: k,
        }
    }

    /// Creates a new `Radix` with radix 10 and given number
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let n = Radix::from(123);
    /// assert_eq!(n.number, 123);
    /// assert_eq!(n.radix, 10);
    /// ```

    pub fn from(n: usize) -> Self {
        Self {
            number: n,
            radix: 10,
        }
    }

    /// Creates a new `Radix` with given number and radix
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let n = Radix::from_radix(11010000, 2);
    /// assert_eq!(n.number, 11010000);
    /// assert_eq!(n.radix, 2);
    /// ```

    pub fn from_radix(n: usize, k: u8) -> Self {
        Self {
            number: n,
            radix: k,
        }
    }

    /// Translate `Radix` to another `Radix` (2 <= k <= 10)
    /// # Panics
    ///
    /// Panics if k > 10
    ///
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from(123);
    /// let mut new1 = n1.to_radix(8);
    ///
    /// let mut n2 = Radix::from_radix(173, 8);
    /// let mut new2 = n2.to_radix(10);
    ///
    /// assert_eq!(new1, Radix::from_radix(173, 8));
    /// assert_eq!(new2, Radix::from_radix(123, 10));
    /// ```

    pub fn to_radix(&mut self, k: u8) -> Self {
        fn to_dec(n: &mut Radix) -> Radix {
            Radix::from(usize::from_str_radix(&n.number.to_string(), n.radix.into()).unwrap())
        }
        fn from_dec(n: &mut Radix, k: u8) -> Radix {
            let mut res = String::new();
            while n.number != 0 {
                res.push(RADIX[n.number % (k as usize)]);
                n.number /= k as usize;
            }
            Radix::from_radix(res.chars().rev().collect::<String>().parse().unwrap(), k)
        }
        if self.radix == 10 {
            from_dec(self, k)
        } else if k == 10 {
            to_dec(self)
        } else {
            from_dec(&mut to_dec(self), k)
        }
    }

    /// Translate `Radix` to another `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let mut n = Radix::from_radix(11010000, 2);
    /// let res = n.to_string_radix(16);
    ///
    /// assert_eq!(res, StringRadix::from_radix("D0", 16));
    /// ```

    pub fn to_string_radix(&mut self, k: u8) -> StringRadix {
        fn to_dec(n: &mut Radix) -> Radix {
            Radix::from(usize::from_str_radix(&n.number.to_string(), n.radix.into()).unwrap())
        }
        fn from_dec(n: &mut Radix, k: u8) -> StringRadix {
            let mut res = String::new();
            while n.number != 0 {
                res.push(RADIX[n.number % (k as usize)]);
                n.number /= k as usize;
            }
            StringRadix::from_radix(&res.chars().rev().collect::<String>(), k)
        }
        if self.radix == 0 {
            from_dec(self, k)
        } else if k == 10 {
            StringRadix::from(&to_dec(self).number.to_string())
        } else {
            from_dec(&mut to_dec(self), k)
        }
    }

    /// Sum 2 `Radix` to new `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let n1 = Radix::from_radix(123, 4);
    /// let n2 = Radix::from_radix(444, 5);
    ///
    /// let res = Radix::add_to_string(n1, n2, 16);
    /// assert_eq!(res, StringRadix::from_radix("97", 16));
    /// ```

    pub fn add_to_string(self, a: Radix, k: u8) -> StringRadix {
        (self + a).to_string_radix(k)
    }

    /// Sub 2 `Radix` to new `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let n1 = Radix::from_radix(123, 4);
    /// let n2 = Radix::from_radix(444, 5);
    ///
    /// let res = Radix::sub_to_string(n2, n1, 16);
    /// assert_eq!(res, StringRadix::from_radix("61", 16));
    /// ```

    pub fn sub_to_string(self, a: Radix, k: u8) -> StringRadix {
        (self - a).to_string_radix(k)
    }

    /// Mul 2 `Radix` to new `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let n1 = Radix::from_radix(123, 4);
    /// let n2 = Radix::from_radix(444, 5);
    ///
    /// let res = Radix::mul_to_string(n1, n2, 16);
    /// assert_eq!(res, StringRadix::from_radix("D14", 16));
    /// ```

    pub fn mul_to_string(self, a: Radix, k: u8) -> StringRadix {
        (self * a).to_string_radix(k)
    }
}

impl StringRadix {
    /// Creates a new `StringRadix`.
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let n = StringRadix::new(2);
    /// assert_eq!(n.number, "0");
    /// assert_eq!(n.radix, 2);
    /// ```

    pub fn new(k: u8) -> Self {
        Self {
            number: String::from("0"),
            radix: k,
        }
    }

    /// Creates a new `StringRadix` with radix 10 and given str number
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let n = StringRadix::from("123");
    /// assert_eq!(n.number, "123");
    /// assert_eq!(n.radix, 10);
    /// ```

    pub fn from(n: &str) -> Self {
        Self {
            number: n.to_string(),
            radix: 10,
        }
    }

    /// Creates a new `StringRadix` with given number and radix
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let n = StringRadix::from_radix("11010000", 2);
    /// assert_eq!(n.number, "11010000");
    /// assert_eq!(n.radix, 2);
    /// ```

    pub fn from_radix(n: &str, k: u8) -> Self {
        Self {
            number: n.to_string(),
            radix: k,
        }
    }

    /// Translate `StringRadix` to another `StringRadix`
    /// # Panics
    ///
    /// Panics if k > 36
    ///
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n = StringRadix::from_radix("11010000", 2);
    /// let res = n.to_radix(16);
    /// assert_eq!(res, StringRadix::from_radix("D0", 16));
    /// ```

    pub fn to_radix(&mut self, k: u8) -> Self {
        fn to_dec(n: &mut StringRadix) -> Radix {
            Radix::from(usize::from_str_radix(&n.number, n.radix.into()).unwrap())
        }
        fn from_dec(n: &mut Radix, k: u8) -> StringRadix {
            let mut res = String::new();
            while n.number != 0 {
                res.push(RADIX[n.number % (k as usize)]);
                n.number /= k as usize;
            }
            StringRadix::from_radix(&res.chars().rev().collect::<String>(), k)
        }
        if k == 10 {
            Self::from(&to_dec(self).number.to_string())
        } else if self.radix == 10 {
            from_dec(&mut Radix::from(self.number.parse().unwrap()), k)
        } else {
            from_dec(&mut to_dec(self), k)
        }
    }

    /// Translate `StringRadix` to another `Radix`
    /// # Panics
    ///
    /// Panics if k > 36
    ///
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let mut n = StringRadix::from_radix("D14", 16);
    /// let mut new = n.to_int_radix(2);
    ///
    /// assert_eq!(new, Radix::from_radix(110100010100, 2));
    /// ```

    pub fn to_int_radix(&mut self, k: u8) -> Radix {
        fn to_dec(n: &mut StringRadix) -> Radix {
            Radix::from(usize::from_str_radix(&n.number, n.radix.into()).unwrap())
        }
        fn from_dec(n: &mut Radix, k: u8) -> Radix {
            let mut res = String::new();
            while n.number != 0 {
                res.push(RADIX[n.number % (k as usize)]);
                n.number /= k as usize;
            }
            Radix::from_radix(res.chars().rev().collect::<String>().parse().unwrap(), k)
        }
        if self.radix == 10 {
            from_dec(&mut Radix::from(self.number.parse().unwrap()), k)
        } else if k == 10 {
            to_dec(self)
        } else {
            from_dec(&mut to_dec(self), k)
        }
    }

    /// Sum 2 `StringRadix` to new `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix::from_radix("123", 4);
    /// let mut n2 = StringRadix::from_radix("444", 5);
    ///
    /// let res = StringRadix::add(&mut n1, &mut n2, 16);
    /// assert_eq!(res, StringRadix::from_radix("97", 16));
    /// ```

    pub fn add(&mut self, b: &mut StringRadix, k: u8) -> Self {
        let mut dec =
            Self::from(&(self.to_int_radix(10).number + b.to_int_radix(10).number).to_string());
        dec.to_radix(k)
    }

    /// Sum 2 `StringRadix` to new `Radix` (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let mut n1 = StringRadix::from_radix("123", 4);
    /// let mut n2 = StringRadix::from_radix("444", 5);
    ///
    /// let res = StringRadix::add_to_int(&mut n1, &mut n2, 8);
    /// assert_eq!(res, Radix::from_radix(227, 8));
    /// ```

    pub fn add_to_int(&mut self, a: &mut StringRadix, k: u8) -> Radix {
        let mut dec = Radix::from(self.to_int_radix(10).number + a.to_int_radix(10).number);
        dec.to_radix(k)
    }

    /// Dif 2 `StringRadix` to new `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix::from_radix("123", 4);
    /// let mut n2 = StringRadix::from_radix("444", 5);
    ///
    /// let res = StringRadix::dif(&mut n2, &mut n1, 16);
    /// assert_eq!(res, StringRadix::from_radix("61", 16));
    /// ```

    pub fn dif(&mut self, a: &mut StringRadix, k: u8) -> Self {
        let mut dec =
            Self::from(&(self.to_int_radix(10).number - a.to_int_radix(10).number).to_string());
        dec.to_radix(k)
    }

    /// Dif 2 `StringRadix` to new `Radix` (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let mut n1 = StringRadix::from_radix("123", 4);
    /// let mut n2 = StringRadix::from_radix("444", 5);
    ///
    /// let res = StringRadix::dif_to_int(&mut n2, &mut n1, 8);
    /// assert_eq!(res, Radix::from_radix(141, 8));
    /// ```

    pub fn dif_to_int(&mut self, a: &mut StringRadix, k: u8) -> Radix {
        let mut dec = Radix::from(self.to_int_radix(10).number - a.to_int_radix(10).number);
        dec.to_radix(k)
    }

    /// Multiply 2 `StringRadix` to new `StringRadix` (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix::from_radix("123", 4);
    /// let mut n2 = StringRadix::from_radix("444", 5);
    ///
    /// let res = StringRadix::mul(&mut n1, &mut n2, 16);
    /// assert_eq!(res, StringRadix::from_radix("D14", 16));
    /// ```

    pub fn mul(&mut self, a: &mut StringRadix, k: u8) -> Self {
        let mut dec =
            Self::from(&(self.to_int_radix(10).number * a.to_int_radix(10).number).to_string());
        dec.to_radix(k)
    }

    /// Multiply 2 `StringRadix` to new `Radix` (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::*;
    ///
    /// let mut n1 = StringRadix::from_radix("123", 4);
    /// let mut n2 = StringRadix::from_radix("444", 5);
    ///
    /// let res = StringRadix::mul_to_int(&mut n1, &mut n2, 8);
    /// assert_eq!(res, Radix::from_radix(6424, 8));
    /// ```

    pub fn mul_to_int(&mut self, a: &mut StringRadix, k: u8) -> Radix {
        let mut dec = Radix::from(self.to_int_radix(10).number * a.to_int_radix(10).number);
        dec.to_radix(k)
    }
}
