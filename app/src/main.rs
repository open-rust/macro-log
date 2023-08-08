use macro_log::*;

#[attr_test]
fn main() {
}

mod tests {
    use macro_log::*;

    #[test]
    fn test_make_compile_time_fn() {
        make_compile_time_fn!(a, b);
        compile_time_fn();
    }
}