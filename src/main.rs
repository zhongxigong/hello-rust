//use ferris_says::say; // from the previous step
extern crate libc;

use std::io::{stdout, stdin, BufWriter};
use libc::{sleep, c_int};

extern {
    fn hello();
}

//#![feature(async_await, await_macro, futures_api)]
fn main() {
    unsafe {
        type c_int = i32;
        sleep(5);
        hello();
    }
//    let stdout = stdout();
//    let out = b"Hello fellow Rustaceans!";
//    let width = 24;
//
//    let mut writer = BufWriter::new(stdout.lock());
//    say(out, width, &mut writer).unwrap();
    let mut x = 5;
//    let (i,b) = (5,'b');

    x = 6;
    let str0 = "hello world";
    let str1 = String::from("hello this is the heap string");
    println!("{},{}", str0, str1);
    print_value(x);
    println!("Hello, world!");

    let v1 = vec![1, 2, 3, 4, 5, 6];
    println!("v1.size is {}", v1.len());

    if 6 == v1.len() {
        println!("true")
    }

    loop {
        println!("{}", x);
        x += 1;
        x = increase_by_one(x);
        if x == 10 {
            break;
        }
    }

    for tmp in &v1 {
        println!("{}", tmp);
    }
}


fn print_value(value: i32) {
    println!("this value is {}", value)
}

fn increase_by_one(value: i32) -> i32 {
    return value + 1;
}

fn div_mod(x: i32, y: i32) -> (i32, i32) {
    return (x / y, x % y);
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        super::print_value(5);
        super::print_value(5);
        super::print_value(5);
    }

    #[test]
    fn testthread() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    }
}
