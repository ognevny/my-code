//! Some macros. Not all of them are written by me,
//! but by Rust community ❤️🦀.

/// Read a line from stdin and parse it to a certain type. Took from
/// [stackoverflow](https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)
///
/// # Panics
/// Panics if the line contains incorrect symbols.
///
/// # Examples
/// ```
/// use ognlib::macros::read;
///
/// read!(x as i32); // this creates a variable `x` from the line and parse it into i32
/// ```
#[macro_export]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner.trim().parse::<$type>().unwrap();
    };
}

/// Read a [`String`] from stdin and trim it. Took from
/// [stackoverflow](https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)
///
/// # Panics
/// Panics if the line contains incorrect symbols.
///
/// # Examples
/// ```
/// use ognlib::macros::read_str;
///
/// read!(x); // this creates a String `x` from the line and trim it
/// ```
#[macro_export]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner.trim();
    };
}

/// Read a line from stdin and parse it to a [`Vec`]. Took from
/// [stackoverflow](https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)
///
/// # Panics
/// Panics if the line contains incorrect symbols.
///
/// # Examples
/// ```
/// use ognlib::macros::read_str;
///
/// read_vec!(x as i32); // this creates a Vec `x` from the line and parse every number into i32
/// ```
#[macro_export]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}
