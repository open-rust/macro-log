use macro_log::*;

fn main() {
    let b = read_dir!("app/src");
    for (name, bin) in b {
        wtf!(name, bin);
    }
}