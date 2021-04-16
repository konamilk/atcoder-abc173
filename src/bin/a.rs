use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n: i32
    }

    let ans: i32 = (100000 - n) % 1000;

    println!("{}", ans);
}
