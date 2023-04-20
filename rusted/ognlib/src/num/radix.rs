// TODO: write more ariphmetic functions.

pub const RADIX: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', 
    '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 
    'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub struct Radix {
    pub number: usize,
    pub radix: u8, }


pub struct StringRadix {
    pub number: String,
    pub radix: u8, }


impl Radix {
    /// Creates a new [`Radix`].
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let n1 = Radix::new(2);
    /// assert_eq!(n1.number, 0);
    /// assert_eq!(n1.radix, 2);
    /// ```

    pub fn new(k: u8) -> Radix {
        Radix { number: 0, radix: k } }


    /// Creates a new [`Radix`] with radix 10 and given number
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let n1 = Radix::from(123);
    /// assert_eq!(n1.number, 123);
    /// assert_eq!(n1.radix, 10);
    /// ```

    pub fn from(n: usize) -> Radix {
        Radix { number: n, radix: 10 } }


    /// Creates a new [`Radix`] with given number and radix
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let n1 = Radix::from_radix(11010000, 2);
    /// assert_eq!(n1.number, 11010000);
    /// assert_eq!(n1.radix, 2);
    /// ```

    pub fn from_radix(n: usize, k: u8) -> Radix {
        Radix { number: n, radix: k } }


    /// Translate [`Radix`] to another [`Radix`] (2 <= k <= 10)
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
    /// assert_eq!(new1.number, 173);
    /// assert_eq!(new1.radix, 8);
    /// 
    /// assert_eq!(new2.number, 123);
    /// assert_eq!(new2.radix, 10);
    /// ```

    pub fn to_radix(&mut self, k: u8) -> Radix {

        fn to_dec(n: &mut Radix)  -> Radix {
            let mut dec = Radix::new(10);
            let mut count = 0;
            while n.number != 0 {
                dec.number += (n.number % 10) * super::power::bin_pow(n.radix as f64, count) as usize;
                n.number /= 10;
                count += 1; }
            dec }

        fn from_dec(n: &mut Radix, k: u8) -> Radix {
            let mut res = String::new();
            while n.number != 0 {
                res.push(RADIX[n.number % (k as usize)]);
                n.number /= k as usize; }
            Radix {
                number: res.chars().rev().collect::<String>().parse().unwrap(),
                radix: k, } }
        
        if self.radix == 10 { return from_dec(self, k) }
        if k == 10 { return to_dec(self) }
        else { from_dec(&mut to_dec(self), k) } }

    
    /// Translate [`Radix`] to [`StringRadix`] (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n = Radix::from_radix(11010000, 2);
    /// let res = n.to_string_radix(16);
    /// assert_eq!(res.number, "D0");
    /// assert_eq!(res.radix, 16);
    /// ```

    pub fn to_string_radix(&mut self, k: u8) -> StringRadix {

        fn to_dec(n: &mut Radix) -> Radix {
            let mut dec = Radix::new(10);
            let mut count = 0;
            while n.number != 0 {
                dec.number += (n.number % 10) * super::power::bin_pow(n.radix as f64, count) as usize;
                n.number /= 10;
                count += 1; }
            dec }

        fn from_dec(n: &mut Radix, k: u8) -> StringRadix {
            let mut res = String::new();
            while n.number != 0 {
                res.push(RADIX[n.number % (k as usize)]);
                n.number /= k as usize; }
            StringRadix { 
                number: res.chars().rev().collect::<String>(), 
                radix: k } }

        if self.radix == 0 { return from_dec(self, k) }
        if k == 10 { return StringRadix::from(&to_dec(self).number.to_string()) }
        else { from_dec(&mut to_dec(self), k) } }


    /// Sum 2 [`Radix`] (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    /// 
    /// let result = Radix::add(&mut n1, &mut n2, 8);
    /// assert_eq!(result.number, 227);
    /// assert_eq!(result.radix, 8);
    /// ```

    pub fn add(&mut self, b: &mut Radix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.to_radix(10).number + b.to_radix(10).number,
            radix: 10, };
        dec.to_radix(k) }


    /// Sum 2 [`Radix`] to [`StringRadix`] (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    /// 
    /// let result = Radix::add_to_string(&mut n1, &mut n2, 16);
    /// assert_eq!(result.number, "97");
    /// assert_eq!(result.radix, 16);
    /// ```

    pub fn add_to_string(&mut self, a: &mut Radix, k: u8) -> StringRadix {
        let mut dec = Radix {
            number: self.to_radix(10).number + a.to_radix(10).number,
            radix: 10, };
        dec.to_string_radix(k) }


    /// Dif 2 [`Radix`] (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    /// 
    /// let result = Radix::dif(&mut n2, &mut n1, 8);
    /// assert_eq!(result.number, 141);
    /// assert_eq!(result.radix, 8);
    /// ```

    pub fn dif(&mut self, a: &mut Radix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.to_radix(10).number - a.to_radix(10).number,
            radix: 10, };
        dec.to_radix(k) }


    /// Dif 2 [`Radix`] to [`StringRadix`] (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    /// 
    /// let result = Radix::dif_to_string(&mut n2, &mut n1, 16);
    /// assert_eq!(result.number, "61");
    /// assert_eq!(result.radix, 16);
    /// ```

    pub fn dif_to_string(&mut self, a: &mut Radix, k: u8) -> StringRadix {
        let mut dec = Radix {
            number: self.to_radix(10).number - a.to_radix(10).number,
            radix: 10, };
        dec.to_string_radix(k) }


    /// Multiply 2 [`Radix`] (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    /// 
    /// let result = Radix::mul(&mut n1, &mut n2, 8);
    /// assert_eq!(result.number, 6424);
    /// assert_eq!(result.radix, 8);
    /// ```

    pub fn mul(&mut self, a: &mut Radix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.to_radix(10).number * a.to_radix(10).number,
            radix: 10, };
        dec.to_radix(k) } 
    
    
    /// Multiply 2 [`Radix`] to [`StringRadix`] (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix::from_radix(123, 4);
    /// let mut n2 = Radix::from_radix(444, 5);
    /// 
    /// let result = Radix::mul_to_string(&mut n1, &mut n2, 16);
    /// assert_eq!(result.number, "D14");
    /// assert_eq!(result.radix, 16);
    /// ```

    pub fn mul_to_string(&mut self, a: &mut Radix, k: u8) -> StringRadix {
        let mut dec = Radix {
            number: self.to_radix(10).number * a.to_radix(10).number,
            radix: 10, };
        dec.to_string_radix(k) } }


impl StringRadix {
    /// Creates a new [`StringRadix`].
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let n1 = StringRadix::new(2);
    /// assert_eq!(n1.number, "0");
    /// assert_eq!(n1.radix, 2);
    /// ```

    pub fn new(k: u8) -> StringRadix {
        StringRadix { number: String::from("0"), radix: k } }


    /// Creates a new [`StringRadix`] with radix 10 and given str number
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let n1 = StringRadix::from("123");
    /// assert_eq!(n1.number, "123");
    /// assert_eq!(n1.radix, 10);
    /// ```

    pub fn from(n: &str) -> StringRadix {
        StringRadix { number: n.to_string(), radix: 10 } }


    /// Creates a new [`StringRadix`] with given number and radix
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let n1 = StringRadix::from_radix("11010000", 2);
    /// assert_eq!(n1.number, "11010000");
    /// assert_eq!(n1.radix, 2);
    /// ```

    pub fn from_radix(n: &str, k: u8) -> StringRadix {
        StringRadix { number: n.to_string(), radix: k } }


    /*/// Translate from string DEC to string radix (2 <= k <= 36)
    /// # Examples
    /// 
    /// ```
    /// use ognlib::num::radix::StringRadix;
    /// 
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 10, };
    /// let new1 = n1.from_string_dec_to_string_radix(8);
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("176"),
    ///     radix: 10, };
    /// let new2 = n2.from_string_dec_to_string_radix(2);
    /// 
    /// assert_eq!(new1.number, "173");
    /// assert_eq!(new1.radix, 8);
    /// 
    /// assert_eq!(new2.number, "10110000");
    /// assert_eq!(new2.radix, 2)
    /// ```
 
    pub fn from_string_dec_to_string_radix(&mut self, k: u8) -> StringRadix {
        let mut res = String::new();
        let mut dec: usize = self.number.parse().unwrap();
        while dec != 0 {
            res.push(RADIX[dec % (k as usize)]);
            dec /= k as usize; }
        StringRadix {
            number: res.chars().rev().collect::<String>(),
            radix: k, } }


    /// Translate from string radix to DEC number (2 <= k <= 36)
    /// # Examples
    /// 
    /// ```
    /// use ognlib::num::radix::StringRadix;
    /// 
    /// let mut n1 = StringRadix {
    ///     number: String::from("10110000"),
    ///     radix: 2, };
    /// let new1 = n1.from_string_radix_to_dec();
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 10, };
    /// let new2 = n2.from_string_radix_to_dec();
    ///
    /// assert_eq!(new1.number, 176);
    /// assert_eq!(new1.radix, 10);
    /// 
    /// assert_eq!(new2.number, 444);
    /// assert_eq!(new2.radix, 10);
    /// ```

    pub fn from_string_radix_to_dec(&mut self) -> Radix {
        Radix {
            number: usize::from_str_radix(&self.number, self.radix.into()).unwrap(),
            radix: 10, } }
    

    /// Sum 2 string radix numbers (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 4, };
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 5, };
    /// 
    /// let result = StringRadix::add(&mut n1, &mut n2, 16);
    /// assert_eq!(result.number, "97");
    /// assert_eq!(result.radix, 16);
    /// ```

    pub fn add(&mut self, b: &mut StringRadix, k: u8) -> StringRadix {
        let mut dec = Radix {
            number: self.from_string_radix_to_dec().number + b.from_string_radix_to_dec().number,
            radix: 10, };
        dec.from_dec_to_string_radix(k) }


    /// Sum 2 string radix numbers to int (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 4, };
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 5, };
    /// 
    /// let result = StringRadix::add_to_int(&mut n1, &mut n2, 8);
    /// assert_eq!(result.number, 227);
    /// assert_eq!(result.radix, 8)
    /// ```

    pub fn add_to_int(&mut self, a: &mut StringRadix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.from_string_radix_to_dec().number + a.from_string_radix_to_dec().number,
            radix: 10, };
        dec.from_dec_to_radix(k) }


    /// Dif 2 string radix numbers (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 4, };
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 5, };
    /// 
    /// let result = StringRadix::dif(&mut n2, &mut n1, 16);
    /// assert_eq!(result.number, "61");
    /// assert_eq!(result.radix, 16);
    /// ```

    pub fn dif(&mut self, a: &mut StringRadix, k: u8) -> StringRadix {
        let mut dec = Radix {
            number: self.from_string_radix_to_dec().number - a.from_string_radix_to_dec().number,
            radix: 10, };
        dec.from_dec_to_string_radix(k) }


    /// Dif 2 string radix numbers to int radix (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 4, };
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 5, };
    /// 
    /// let result = StringRadix::dif_to_int(&mut n2, &mut n1, 8);
    /// assert_eq!(result.number, 141);
    /// assert_eq!(result.radix, 8);
    /// ```

    pub fn dif_to_int(&mut self, a: &mut StringRadix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.from_string_radix_to_dec().number - a.from_string_radix_to_dec().number,
            radix: 10, };
        dec.from_dec_to_radix(k) }


    /// Multiply 2 string radix numbers (2 <= k <= 36)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 4, };
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 5, };
    /// 
    /// let result = StringRadix::mul(&mut n1, &mut n2, 16);
    /// assert_eq!(result.number, "D14");
    /// assert_eq!(result.radix, 16);
    /// ```

    pub fn mul(&mut self, a: &mut StringRadix, k: u8) -> StringRadix {
        let mut dec = Radix {
            number: self.from_string_radix_to_dec().number * a.from_string_radix_to_dec().number,
            radix: 10, };
        dec.from_dec_to_string_radix(k) } 
    
    
    /// Multiply 2 string radix numbers to int radix (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::StringRadix;
    ///
    /// let mut n1 = StringRadix {
    ///     number: String::from("123"),
    ///     radix: 4, };
    /// 
    /// let mut n2 = StringRadix {
    ///     number: String::from("444"),
    ///     radix: 5, };
    /// 
    /// let result = StringRadix::mul_to_int(&mut n1, &mut n2, 8);
    /// assert_eq!(result.number, 6424);
    /// assert_eq!(result.radix, 8);
    /// ```

    pub fn mul_to_int(&mut self, a: &mut StringRadix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.from_string_radix_to_dec().number * a.from_string_radix_to_dec().number,
            radix: 10, };
        dec.from_dec_to_radix(k) } */}
