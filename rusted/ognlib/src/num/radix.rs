// TODO: write ariphmetic functions and make existing functions more useful

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
    /// assert_eq!(n1.from_int_to_radix_string(16), "7B");
    /// assert_eq!(n2.from_int_to_radix_string(2), "10110000")
    /// ```
    pub fn from_int_to_radix_string(&mut self, k: u8) -> String {
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
    pub fn from_radix_to_dec(&mut self) -> Radix {
        let mut dec = Radix {
            number: 0,
            radix: 10, };
        let mut count = 0;
        while self.number != 0 {
            dec.number += (self.number % 10) * super::power::bin_pow(self.radix as f64, count) as usize;
            self.number /= 10; 
            count += 1; }
        dec } }
