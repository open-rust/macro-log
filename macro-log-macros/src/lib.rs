use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

// see: https://dengjianping.github.io/2019/02/28/%E5%A6%82%E4%BD%95%E7%BC%96%E5%86%99%E4%B8%80%E4%B8%AA%E8%BF%87%E7%A8%8B%E5%AE%8F(proc-macro).html
#[proc_macro_attribute]
pub fn debug(_: TokenStream, func: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis; // pub
    let func_block = &func.block; // { code block }

    let sig = &func.sig;
    let func_constness = &sig.constness; // const
    let func_name = &sig.ident; // fn name
    let func_generics = &sig.generics; // <'a, T>
    let func_where_clause = &func_generics.where_clause; // where
    let func_inputs = &sig.inputs; // arguments
    let func_output = &sig.output; // return value
    
    let caller = quote!{
        #func_vis #func_constness fn #func_name #func_generics(#func_inputs) #func_output #func_where_clause {
            macro_log::d!("call fn {}()", stringify!(#func_name));
            #func_block
        }
    };
    // println!("compile result: \n---------------------\n{}\n---------------------", caller.to_string());
    caller.into() 
}