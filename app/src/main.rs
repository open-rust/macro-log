#![allow(unused)]

use macro_log::*;

fn main() {
    test(&2147483647, (1, 2), Arg::default());
    test(&999, (2, 3), Arg::default());
}

#[derive(Default, Debug)]
pub struct Arg {
    a: String,
    b: Vec<u8>,
    c: Vec<Vec<String>>,
}

#[debug]
pub extern "C" fn test<'a, T>(value: &'a T, another: (i8, i8), arg: Arg, )
where T: std::fmt::UpperHex + std::fmt::Debug,
{
    i!("test run, value is: {:X}", value);
    // `format_args!` cannot capture variables when the format string is expanded from a macro
    // println!("test run, value is: {value:X}"); // error: there is no argument named `value`
    i!("test ok");
}

#[cfg(test)]
mod tests {
    use macro_log::*;

    #[test]
    #[debug]
    fn test() {
        i!("Hello, test");
    }
}