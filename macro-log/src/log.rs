#![allow(unused)]

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Level {
    VERBOSE = 2,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    WTF,
}

impl Level {
    fn tag(self: Self) -> &'static str {
        let index = self as usize;
        ["", "", "V", "D", "I", "W", "E", "WTF"][index]
    }
}

#[cfg(not(target_os = "android"))]
pub mod platform {
    use super::Level;
    use crate::time::get_time;

    pub fn print(level: Level, file: &str, line: u32, str: String) {
        let time = get_time();
        match level {
            Level::ERROR => {
                eprintln!("{time} - [{tag}] - {}:{} -> {}", file, line, str, tag = level.tag());
            },
            _ => {
                println!("{time} - [{tag}] - {}:{} -> {}", file, line, str, tag = level.tag());
            },
        }
    }

    #[cfg(debug_assertions)]
    #[macro_export]
    macro_rules! wtf {
        ($($arg: expr $(,)?)+) => {
            $(
                $crate::log!($crate::log::Level::WTF, "{} = {:?}", stringify!($arg), $arg);
            )+
        }
    }
}

#[cfg(target_os = "android")]
pub mod platform {
    use super::Level;
    use crate::time::get_time;
    pub const FMT: *const u8 = "%s\0".as_ptr();

    extern {
        // int __android_log_write(int prio, const char* tag, const char* text);
        // int __android_log_print(int prio, const char* tag, const char* fmt, ...) __attribute__((__format__(printf, 3, 4)));
        pub fn __android_log_print(level: i32, tag: *const u8, fmt: *const u8, ...);
    }

    pub fn print(level: Level, file: &str, line: u32, mut str: String) {
        let tag = format!("{}:{}\0", file, line);
        str.push('\0');
        unsafe {
            __android_log_print(level as i32, tag.as_ptr(), FMT, str.as_ptr());
        }
    }
    
    #[cfg(debug_assertions)]
    #[macro_export]
    macro_rules! wtf {
        ($($arg: expr $(,)?)+) => {
            $(
                $crate::log!($crate::log::Level::ERROR, "{} = {:?}", stringify!($arg), $arg);
            )+
        }
    }
}

#[cfg(debug_assertions)]
mod debug {
    #[macro_export]
    macro_rules! d {
        ($($arg: tt)+) => {
            $crate::log!($crate::log::Level::DEBUG, $($arg)+);
        }
    }
}

#[cfg(not(debug_assertions))]
mod release {
    #[macro_export]
    macro_rules! wtf {
        ($($arg: tt)+) => (())
    }

    #[macro_export]
    macro_rules! d {
        ($($arg: tt)+) => (())
    }
}

mod common {
    #[macro_export]
    macro_rules! log {
        ($type: expr, $($arg: tt)+) => {
            $crate::log::platform::print($type, file!(), line!(), format!($($arg)+));
        }
    }

    #[macro_export]
    macro_rules! i {
        ($($arg: tt)+) => {
            $crate::log!($crate::log::Level::INFO, $($arg)+);
        }
    }

    #[macro_export]
    macro_rules! w {
        ($($arg: tt)+) => {
            $crate::log!($crate::log::Level::WARN, $($arg)+);
        }
    }

    #[macro_export]
    macro_rules! e {
        ($($arg: tt)+) => {
            $crate::log!($crate::log::Level::ERROR, $($arg)+);
        }
    }
}