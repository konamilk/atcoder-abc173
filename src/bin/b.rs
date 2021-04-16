use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n:usize,
        sn:[String; n]
    }

    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;

    for s in sn {
        if s == String::from("AC") {
            ac += 1;
        }
        if s == String::from("WA") {
            wa += 1;
        }
        if s == String::from("TLE") {
            tle += 1;
        }
        if s == String::from("RE") {
            re += 1;
        }
    }

    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);

}
