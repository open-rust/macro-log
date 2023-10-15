use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

pub fn read_dir(args: TokenStream) -> TokenStream {
    let path = syn::parse_macro_input!(args as LitStr).value() + "/";
    // plan 1
    // let files = get_files(&path).iter().map(|it| format!("r#\"{it}\"#")).collect::<Vec<String>>().join(",");
    // plan 2
    // let files = get_files(&path).iter().map(|it| quote!(#it,).to_string()).collect::<Vec<String>>().join("");
    // plan 3
    // let files = get_files(&path).iter().map(|it| quote!(#it).to_string()).collect::<Vec<String>>().join(",");
    let (workspace, files) = get_files(&path);
    #[cfg(windows)]
    let workspace = workspace.replacen("\\\\?\\", "", 1);
    println!("wrokspace: {workspace}");
    let len = files.len();
    let files = files.iter()
        .map(|it| {
            let file = workspace.clone() + "/" + it;
            quote!((#it, include_bytes!(#file))).to_string()
        }).collect::<Vec<String>>().join(",");
    let files = files.parse::<proc_macro2::TokenStream>().unwrap();
    quote! {
        [#files] as [(&str, &[u8]); #len]
    }.into()
}

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(DirEntry, &mut Vec<DirEntry>), vec: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb, vec)?;
            } else {
                cb(entry, vec);
            }
        }
    }
    Ok(())
}

fn get_files(_path: &str) -> (String, Vec<String>) {
    let mut files = vec![];
    let path = Path::new(&_path);
    println!("visit dir: {:?}", path);
    // 规范化工作区路径
    let Ok(workspace_path) = path.canonicalize() else {
        eprintln!("Failed to canonicalize the directory!");
        return (Default::default(), vec![]);
    };
    let Ok(_) = visit_dirs(path, &|entry, files| {
        // println!("file => {}", entry.path().to_string_lossy());
        println!("file => {}", entry.path().to_string_lossy().replacen(_path, "", 1));
        files.push(entry);
    }, &mut files) else {
        eprintln!("Failed to read the directory!");
        return (Default::default(), vec![]);
    };
    (
        workspace_path.to_string_lossy().to_string(),
        files.iter()
            .map(|it| it.path().to_string_lossy().to_string().replacen(_path, "", 1))
            .collect::<Vec<String>>()
    )
}
