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
        n:Chars
    }
    let mut ans = 0;
    let keta = n.len();
    for sum in 1..=9 * 14 {
        let mut dp = vec![vec![vec![vec![0; 2]; sum]; sum + 1]; keta + 1];
        dp[0][0][0][1] = 1;
        for d in 0..keta {
            for keta_sum in 0..=sum {
                for j in 0..sum {
                    for fixed in 0..2 {
                        for num in 0..10 {
                            if keta_sum + num > sum {
                                continue;
                            }
                            if fixed == 1 && n[d] as usize - 48 < num {
                                continue;
                            }
                            let check = if fixed == 1 && n[d] as usize - 48 == num {
                                1
                            } else {
                                0
                            };
                            dp[d + 1][keta_sum + num][(10 * j + num) % sum][check] +=
                                dp[d][keta_sum][j][fixed];
                        }
                    }
                }
            }
        }
        ans += dp[keta][sum][0].iter().sum::<usize>();
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
