pub mod time;
pub mod log;
pub use macro_log_macros::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        crate::i!("Hello {}", "World!");
    }

    #[test]
    fn test_import() {
        use crate::wtf;
        let mut str = "Hello".to_owned();
        wtf!(str);
        wtf!(str.pop());
    }
}