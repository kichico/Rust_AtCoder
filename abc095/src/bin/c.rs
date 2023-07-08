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
        a:i64,b:i64,c:i64,mut x:i64,mut y:i64
    }
    let mut ans = 0;
    if a + b > 2 * c {
        let amount = x.min(y);
        x -= amount;
        y -= amount;
        ans += c * amount * 2;
    }
    if x <= 0 && y <= 0 {
        println!("{}", ans);
        return;
    }
    ans += if a > 2 * c { c * 2 * x } else { a * x };
    ans += if b > 2 * c { c * 2 * y } else { b * y };
    println!("{}", ans);
}
fn main() {
    solve();
}
