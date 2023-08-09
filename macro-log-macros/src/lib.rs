use proc_macro::{TokenStream, TokenTree};

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
    for token in item.clone().into_iter() {
        println!("token -> {token} {:?}", token.span());
    }
    let mut new_fn = item.clone().into_iter().collect::<Vec<TokenTree>>();
    let body = new_fn.pop().unwrap();
    let new_stream = r#"
        $fn {
            macro_log::i!("This is new $fn");
            $body
        }
    "#
        .replace("$fn", TokenStream::from_iter(new_fn.clone()).to_string().as_str())
        .replace("$body", body.to_string().as_str())
        .parse::<TokenStream>().unwrap();
    println!("stream: {}", new_stream);
    return new_stream;
    // new_fn.push(new_stream.into_iter().last().unwrap());
    // return TokenStream::from_iter(new_fn);
}