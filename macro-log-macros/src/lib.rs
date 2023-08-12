use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, FnArg, punctuated::Punctuated, token::Comma};

// see: https://dengjianping.github.io/2019/02/28/%E5%A6%82%E4%BD%95%E7%BC%96%E5%86%99%E4%B8%80%E4%B8%AA%E8%BF%87%E7%A8%8B%E5%AE%8F(proc-macro).html
#[proc_macro_attribute]
pub fn debug(_: TokenStream, func: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis; // pub
    let func_block = &func.block; // { code block }

    let sig = &func.sig;
    let func_constness = &sig.constness; // const
    let func_abi = &sig.abi; // extern "abi"
    let func_name = &sig.ident; // fn name
    let func_generics = &sig.generics; // <'a, T>
    let func_where_clause = &func_generics.where_clause; // where
    let func_inputs = &sig.inputs; // arguments
    let func_output = &sig.output; // return value
    
    let args = parse_args(func_inputs);
    let (format, values) = get_log_format_values(&func_name.to_string(), args);
    let log = match func_output {
        syn::ReturnType::Default => quote! {
            macro_log::d!("{}", call);
        },
        syn::ReturnType::Type(_, _) => quote! {
            macro_log::d!("{} => {:?}", call, return_value);
        },
    };

    let caller = quote! {
        #func_vis #func_constness #func_abi fn #func_name #func_generics(#func_inputs) #func_output #func_where_clause {
            let call = format!(#format, #values);
            let return_value = #func_block;
            #log
            return_value
        }
    };
    // println!("compile result: \n---------------------\n{}\n---------------------", caller.to_string());
    caller.into()
}

fn parse_args(func_inputs: &Punctuated<FnArg, Comma>) -> Vec<String> {
    let mut args = vec![];
    for arg in func_inputs.into_iter() {
        match arg {
            // The self argument of an associated method
            FnArg::Receiver(arg) => {
                // This is self: Self or self: &Self
                let tokens = quote!(#arg).to_string();
                // println!("tokens -> {:?}", tokens); // tokens -> "value : & 'a T"
                let (name, _vartype) = tokens.split_once(" : ").unwrap();
                args.push(name.into());
            }
            // A function argument accepted by pattern and type
            FnArg::Typed(_) => { // _ same as arg
                let tokens = quote!(#arg).to_string();
                // println!("tokens -> {:?}", tokens); // tokens -> "value : & 'a T"
                let (name, _vartype) = tokens.split_once(" : ").unwrap();
                args.push(name.into());
            }
        }
    }
    args
}

/// for quote!{ macro_log::d!(#format, #values); }
fn get_log_format_values(func_name: &str, args: Vec<String>) -> (String, proc_macro2::TokenStream) {
    let format_args = args.iter()
        .map(|it| if it.as_str() != "_" {
            format!("{it} = {{:?}}")
        } else {
            "_ = ?".to_string()
        })
        .collect::<Vec<String>>().join(", ");
    let format = format!("call fn {func_name}({format_args})");
    // println!("format -> {format:?}"); // format -> "call fn test(value = {:?}, another = {:?}, arg = {:?})"

    let values = args.iter()
        .filter(|it| it.as_str() != "_")
        .map(|it| format!("{it}")).collect::<Vec<String>>().join(",");
    let values = values.parse::<proc_macro2::TokenStream>().unwrap();
    // println!("values -> {values}"); // values -> value, another, arg

    (format, values)
}