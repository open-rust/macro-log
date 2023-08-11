use proc_macro::{TokenStream, TokenTree};

// 属性宏
#[proc_macro_attribute]
pub fn prototype(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut new_fn = item.clone().into_iter().collect::<Vec<TokenTree>>();
    let body = new_fn.pop().unwrap();
    let new_stream = r##"
        $fn {
            macro_log::d!(r#"call $fn"#);
            $body
        }
    "##
        .replace("$fn", TokenStream::from_iter(new_fn.clone()).to_string().as_str())
        .replace("$body", body.to_string().as_str())
        .parse::<TokenStream>().unwrap();
    // println!("stream: {}", new_stream);
    return new_stream;
    // new_fn.push(new_stream.into_iter().last().unwrap());
    // return TokenStream::from_iter(new_fn);
}

// 属性宏
#[proc_macro_attribute]
pub fn debug(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = item.clone().into_iter();
    let mut func = "".to_owned();
    let mut input = "".to_owned();
    while let Some(token) = tokens.next() {
        let symbol = token.to_string();
        // println!("symbol -> {}", symbol);
        if symbol == "fn" {
            func = tokens.next().unwrap().to_string();
        } else if symbol.starts_with("(") {
            input = symbol.replacen("(", "", 1);
            input.pop();
            break;
        }
        // println!("token -> {token} {:?}", token.span());
    }
    let arguments = input.split(",").filter(|&it| it != "").map(|it| it.trim()).collect::<Vec<&str>>();
    // println!("arguments -> {:?}", arguments);
    let mut new_fn = item.clone().into_iter().collect::<Vec<TokenTree>>();
    let body = new_fn.pop().unwrap();
    let mut values = "".to_owned();
    let new_stream = r##"
        $fn {
            macro_log::d!(r#"call fn $func($arguments)"# $values);
            $body
        }
    "##
        .replace("$fn", TokenStream::from_iter(new_fn.clone()).to_string().as_str())
        .replace("$func", &func)
        .replace("$arguments", {
            &arguments.iter().map(
                |it| {
                    let (name, vartype) = it.split_once(" : ").unwrap();
                    // println!("var: {name}, type: {vartype}");
                    values.push_str(&format!(",{name}"));
                    format!("{name} = {display}", display = "{:?}")
                }
            ).collect::<Vec<String>>().join(", ")
        })
        .replace("$values", &values)
        .replace("$body", body.to_string().as_str())
        .parse::<TokenStream>().unwrap();
    // println!("stream: {}", new_stream);
    // return new_stream;
    new_fn.push(new_stream.into_iter().last().unwrap());
    return TokenStream::from_iter(new_fn);
}