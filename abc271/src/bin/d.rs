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
        n:usize,mut s:i64,card:[(i64,i64);n]
    }
    let mut dp: Vec<HashMap<i64, i64>> = vec![HashMap::new(); n];
    dp[0].insert(card[0].0, 0);
    dp[0].insert(card[0].1, 1);
    for i in 1..n {
        let mut tmp: HashMap<i64, i64> = HashMap::new();
        for v in &dp[i - 1] {
            tmp.insert(*v.0 + card[i].0, 0);
            tmp.insert(*v.0 + card[i].1, 1);
        }
        dp[i] = tmp;
    }
    if dp[n - 1].contains_key(&s) {
        let mut ans: Vec<char> = Vec::new();
        for i in (0..n).rev() {
            let flip = *dp[i].get_key_value(&s).unwrap().1;
            if flip == 0 {
                ans.push('H');
                s -= card[i].0;
            } else {
                ans.push('T');
                s -= card[i].1;
            }
        }
        ans.reverse();
        println!("Yes\n{}", ans.iter().join(""));
    } else {
        println!("No");
    }
}

fn main() {
    solve();
}
