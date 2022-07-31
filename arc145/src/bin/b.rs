#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:i64,a:i64,b:i64
    }
    if a <= b {
        println!("{}", max(n - a + 1, 0));
        return;
    } else if n < a {
        println!("0");
        return;
    }
    let coe = n / a;
    let mut ans = (coe - 1) * b;
    let nn = n % a;
    if nn >= b {
        ans += b;
    } else {
        ans += nn + 1;
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
