#[allow(unused_imports)]
use itertools::Itertools;
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
        n:usize
    }
    let mut ans = vec!['0'; 9];
    let mut cnt = 0;
    for a in 1..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            ans[0] = to_char(a);
                            ans[1] = to_char(a);
                            ans[2] = to_char(b);
                            ans[3] = to_char(c);
                            ans[4] = to_char(d);
                            ans[5] = to_char(d);
                            ans[6] = to_char(e);
                            ans[7] = to_char(f);
                            ans[8] = to_char(e);
                            cnt += 1;
                            if cnt == n {
                                println!("{}", ans.iter().join(""));
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
fn main() {
    solve();
}
