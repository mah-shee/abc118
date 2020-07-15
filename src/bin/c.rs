use num_integer::gcd;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans = 0;
    for i in a.iter() {
        ans = gcd(*i, ans);
    }
    println!("{:?}", ans);
}
