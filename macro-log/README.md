# macro-log

## install

```
$ cargo add macro-log
```

## usage

```
use macro_log::*;

fn main() {
    let a = test1(0xff);
    i!("a = {a}");
    let b = test2(0xff);
    wtf!(b);
}

#[param]
fn test1(a: u32) -> u32 {
    a * 10
}

#[debug]
fn test2(a: u32) -> u32 {
    a * 100
}
```

output:
```
2023-08-14 12:29:41 - [D] - app\src\main.rs:10 -> call fn test1(a = 255)
2023-08-14 12:29:41 - [I] - app\src\main.rs:5 -> a = 2550
2023-08-14 12:29:41 - [D] - app\src\main.rs:15 -> called fn test2(a = 255) => 25500
2023-08-14 12:29:41 - [WTF] - app\src\main.rs:7 -> b = 25500
```