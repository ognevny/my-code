// TODO: write ariphmetic functions and make existing functions more useful. add string radix struct.

pub const RADIX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

pub struct Radix {
    pub number: usize,
    pub radix: u8, }


impl Radix {
    /// Translate int number from DEC to radix (2 <= k <= 10)
    /// # Examples
    /// 
    /// ```
    /// use ognlib::num::radix::Radix;
    /// 
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 10, };
    /// let new1 = n1.from_int_to_radix(8);
    /// 
    /// let mut n2 = Radix {
    ///     number: 176,
    ///     radix: 10, };
    /// let new2 = n2.from_int_to_radix(2);
    /// 
    /// assert_eq!(new1.number, 173);
    /// assert_eq!(new1.radix, 8);
    /// 
    /// assert_eq!(new2.number, 10110000);
    /// assert_eq!(new2.radix, 2)
    /// ```
    /// 
    pub fn from_int_to_radix(&mut self, k: u8) -> Radix {
        let mut res = String::new();
        while self.number != 0 {
            res.push(RADIX[(self.number % (k as usize)) as usize]);
            self.number /= k as usize; }
        let res = Radix {
            number: res.chars().rev().collect::<String>().parse().unwrap(),
            radix: k, };
        res }


    /// Translate int number from DEC to radix string (2 <= k <= 16)
    /// # Examples
    /// 
    /// ```
    /// use ognlib::num::radix::Radix;
    /// 
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 10, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 176,
    ///     radix: 10, };
    ///
    /// assert_eq!(n1.from_int_to_string_radix(16), "7B");
    /// assert_eq!(n2.from_int_to_string_radix(2), "10110000")
    /// ```
    /// 
    pub fn from_int_to_string_radix(&mut self, k: u8) -> String {
        let mut res = String::new();
        while self.number != 0 {
            res.push(RADIX[(self.number % (k as usize)) as usize]);
            self.number /= k as usize; }
        res.chars().rev().collect::<String>() }


    /// Translate int number from radix to int DEC number (2 <= k <= 10)
    /// # Examples
    /// 
    /// ```
    /// use ognlib::num::radix::Radix;
    /// 
    /// let mut n1 = Radix {
    ///     number: 10110000,
    ///     radix: 2, };
    /// let new1 = n1.from_radix_to_dec();
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 10, };
    /// let new2 = n2.from_radix_to_dec();
    ///
    /// assert_eq!(new1.number, 176);
    /// assert_eq!(new1.radix, 10);
    /// 
    /// assert_eq!(new2.number, 444);
    /// assert_eq!(new2.radix, 10);
    /// ```
    /// 
    pub fn from_radix_to_dec(&mut self) -> Radix {
        let mut dec = Radix {
            number: 0,
            radix: 10, };
        let mut count = 0;
        while self.number != 0 {
            dec.number += (self.number % 10) * super::power::bin_pow(self.radix as f64, count) as usize;
            self.number /= 10; 
            count += 1; }
        dec } 
    

    /// Sum 2 radix numbers (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 4, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 5, };
    /// 
    /// let result = Radix::add(&mut n1, &mut n2, 8);
    /// assert_eq!(result.number, 227);
    /// assert_eq!(result.radix, 8);
    /// ```
    /// 
    pub fn add(&mut self, b: &mut Radix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.from_radix_to_dec().number + b.from_radix_to_dec().number,
            radix: 10, };
        dec.from_int_to_radix(k) }


    /// Sum 2 radix numbers to string radix (2 <= k <= 16)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 4, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 5, };
    /// 
    /// let result = Radix::add_to_string(&mut n1, &mut n2, 16);
    /// assert_eq!(result, "97")
    /// ```
    /// 
    pub fn add_to_string(&mut self, a: &mut Radix, k: u8) -> String {
        let mut dec = Radix {
            number: self.from_radix_to_dec().number + a.from_radix_to_dec().number,
            radix: 10, };
        dec.from_int_to_string_radix(k) }


    /// Dif 2 radix numbers (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 4, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 5, };
    /// 
    /// let result = Radix::dif(&mut n2, &mut n1, 8);
    /// assert_eq!(result.number, 141);
    /// assert_eq!(result.radix, 8);
    /// ```
    /// 
    pub fn dif(&mut self, a: &mut Radix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.from_radix_to_dec().number - a.from_radix_to_dec().number,
            radix: 10, };
        dec.from_int_to_radix(k) }


    /// Dif 2 radix numbers to string radix (2 <= k <= 16)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 4, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 5, };
    /// 
    /// let result = Radix::dif_to_string(&mut n2, &mut n1, 16);
    /// assert_eq!(result, "61");
    /// ```
    /// 
    pub fn dif_to_string(&mut self, a: &mut Radix, k: u8) -> String {
        let mut dec = Radix {
            number: self.from_radix_to_dec().number - a.from_radix_to_dec().number,
            radix: 10, };
        dec.from_int_to_string_radix(k) }


    /// Multiply 2 radix numbers (2 <= k <= 10)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 4, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 5, };
    /// 
    /// let result = Radix::mul(&mut n1, &mut n2, 8);
    /// assert_eq!(result.number, 6424);
    /// assert_eq!(result.radix, 8);
    /// ```
    /// 
    pub fn mul(&mut self, a: &mut Radix, k: u8) -> Radix {
        let mut dec = Radix {
            number: self.from_radix_to_dec().number * a.from_radix_to_dec().number,
            radix: 10, };
        dec.from_int_to_radix(k) } 
    
    
    /// Multiply 2 radix numbers to string radix (2 <= k <= 16)
    /// # Examples
    ///
    /// ```
    /// use ognlib::num::radix::Radix;
    ///
    /// let mut n1 = Radix {
    ///     number: 123,
    ///     radix: 4, };
    /// 
    /// let mut n2 = Radix {
    ///     number: 444,
    ///     radix: 5, };
    /// 
    /// let result = Radix::mul_to_string(&mut n1, &mut n2, 16);
    /// assert_eq!(result, "D14");
    /// ```
    /// 
    pub fn mul_to_string(&mut self, a: &mut Radix, k: u8) -> String {
        let mut dec = Radix {
            number: self.from_radix_to_dec().number * a.from_radix_to_dec().number,
            radix: 10, };
        dec.from_int_to_string_radix(k) } }
