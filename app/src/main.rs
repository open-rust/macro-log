use macro_log::*;

#[log_when_call]
fn main() {
    test(&2147483647, (1), Arg::default());
    test(&999, (2), Arg::default());
}

#[derive(Default, Debug)]
struct Arg {
    a: String,
    b: Vec<u8>,
    c: Vec<Vec<String>>,
}

#[log_call_info]
/* 参数中不能出现逗号，例如(i8, i8)不被允许 */
extern "C" fn test<'a, T>(value: &'a T, another: (i8), Arg: Arg, )
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
    fn test_make_compile_time_fn() {
        make_compile_time_fn!(a, b);
        compile_time_fn();
    }
}