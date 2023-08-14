#![allow(unused)]

use macro_log::*;

#[param]
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

impl Arg {
    #[debug]
    fn print(self: Self, some_arg: u8, _: (), _: u32) {
        i!("Arg: a = {:?}, b = {:?}, c = {:?}, some_arg = {:?}", self.a, self.b, self.c, some_arg);
    }
}

#[debug]
pub extern "C" fn test<'a, T>(value: &'a T, another: (i8, i8), arg: Arg, ) -> &'a T
where T: std::fmt::UpperHex + std::fmt::Debug,
{
    i!("test run, value is: {value:X}");
    // For v0.1:
    // `format_args!` cannot capture variables when the format string is expanded from a macro
    // println!("test run, value is: {value:X}"); // error: there is no argument named `value`
    arg.print(88, (), 0xff);
    i!("test ok");
    value
}

#[debug]
async fn async_fn() {
    i!("Hello, async fn!");
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