use macro_log::*;

#[attr_test]
fn main() {
    test(2147483647);
}

#[attr_test]
fn test(value: i32) {
    println!("test run, value is: {:X}", value);
    // `format_args!` cannot capture variables when the format string is expanded from a macro
    // println!("test run, value is: {value:X}"); // error: there is no argument named `value`
    println!("test ok");
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