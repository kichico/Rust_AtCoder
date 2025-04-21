#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
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
        n:usize,date:[usize;n]
    }
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=date[i - 1] {
            let mut st: HashSet<char> = HashSet::new();
            let month = i.to_string().chars().collect::<Vec<char>>();
            let day = j.to_string().chars().collect::<Vec<char>>();
            for m in month {
                st.insert(m);
            }
            for d in day {
                st.insert(d);
            }
            if st.len() == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
