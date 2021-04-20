use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
use std::cmp::Reverse;

fn main() {
    input! {
        n:usize,
        mut a: [i64;n]
    }

    a.sort_by_key(|&x| Reverse(x));

    let mut ans = 0i64;

    for i in 0..n {
        if i == 0{
            continue
        }
        ans += a[i/2]
    }

    println!("{}", ans);
}
