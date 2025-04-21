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
        k:usize,s:Chars,t:Chars
    }
    let mut card = vec![k; 9];
    for i in 0..4 {
        let c = s[i];
        let n = (c as u32 - 48) as usize - 1;
        card[n] -= 1;
    }
    for i in 0..4 {
        let c = t[i];
        let n = (c as u32 - 48) as usize - 1;
        card[n] -= 1;
    }
    let all_card = (9 * k - 8) * (9 * k - 9);
    let mut win = 0;
    for i in 1..10 {
        let mut takahasi = vec![0; 9];
        for k in 0..4 {
            let c = s[k];
            let n = (c as u32 - 48) as usize - 1;
            takahasi[n] += 1;
        }
        takahasi[i - 1] += 1;
        for j in 1..10 {
            let mut aoki = vec![0; 9];
            for k in 0..4 {
                let c = t[k];
                let n = (c as u32 - 48) as usize - 1;
                aoki[n] += 1;
            }
            aoki[j - 1] += 1;
            let takahasi_score = takahasi
                .iter()
                .enumerate()
                .map(|x| (x.0 + 1) * 10.pow(*x.1 as u32))
                .sum::<usize>();
            let aoki_score = aoki
                .iter()
                .enumerate()
                .map(|x| (x.0 + 1) * 10.pow(*x.1 as u32))
                .sum::<usize>();
            if takahasi_score > aoki_score {
                let p = if i == j {
                    if card[i - 1] == 0 {
                        0usize
                    } else {
                        card[i - 1] * (card[j - 1] - 1)
                    }
                } else {
                    card[i - 1] * card[j - 1]
                };
                win += p;
            }
        }
    }
    println!("{}", win as f64 / all_card as f64);
}
fn main() {
    solve();
}
