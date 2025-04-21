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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

fn longest_common_subsequence(s: Vec<char>, t: Vec<char>) -> String {
    let mut dp = vec![vec![0usize; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    let mut r = s.len();
    let mut c = t.len();
    let mut ans: Vec<char> = Vec::new();
    while r != 0 && c != 0 {
        if dp[r][c] == dp[r][c - 1] {
            c -= 1;
            continue;
        } else if dp[r][c] == dp[r - 1][c] {
            r -= 1;
            continue;
        }
        r -= 1;
        c -= 1;
        ans.push(s[r]);
    }
    ans.reverse();
    return ans.iter().collect::<String>();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        s:Chars,t:Chars
    }
    let ans = longest_common_subsequence(s, t);
    println!("{}", ans);
}
fn main() {
    solve();
}
