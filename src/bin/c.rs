use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        h:usize,
        w:usize,
        k:usize,
        c:[Chars; h]
    }

    let mut ans = 0;

    for select_row in 0..(1 << h){
        for select_column in 0..(1 << w){
            let mut test :Vec<Vec<char>> = c.clone();
            for i in 0..h{
                for j in 0..w {
                    if select_row & (1 << i) > 0{
                        test[i][j] = 'R'
                    }
                    if select_column & (1 << j) > 0{
                        test[i][j] = 'R'
                    }
                }
            }
            // dbg!(test);
            // println!("{:?}", test.into_iter().flatten().collect::<Vec<char>>());
            let cnt = test.iter()
                .flatten()
                .filter(|&x| x == &'#')
                .count();

            if cnt == k{
                ans += 1
            }
        }
    }

    println!("{}", ans);
}
