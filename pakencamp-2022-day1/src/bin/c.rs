#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        s:Chars
    }
    let MOD = 998244353;
    let n = s.len();
    let maxi = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
    let mut counter = 0;
    for i in 0..maxi {
        if s[i] == s[n - i - 1] && s[i] == '?' {
            counter += 1;
        } else if s[i] != s[n - i - 1] {
            if s[i] != '?' && s[n - i - 1] != '?' {
                println!("0");
                return;
            }
        }
    }
    let mut ans = 1i128;
    for i in 0..counter {
        ans *= 26;
        ans = ans.rem_euclid(MOD);
    }

    println!("{}", ans.rem_euclid(MOD));
}
fn main() {
    solve();
}
