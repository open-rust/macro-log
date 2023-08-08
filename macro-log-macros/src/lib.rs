use proc_macro::TokenStream;

// 类函数宏
#[proc_macro]
pub fn make_compile_time_fn(item: TokenStream) -> TokenStream {
    println!("make_compile_time_fn item: {:?}", item);
    r#"fn compile_time_fn() -> () { macro_log::i!("This is compile_time_fn()"); }"#.parse().unwrap()
}

// 属性宏
#[proc_macro_attribute]
pub fn attr_test(args: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr_test args: {:?}", args);
    println!("attr_test item: {:?}", item);
    todo!()
}