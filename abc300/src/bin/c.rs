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
        h:usize,w:usize,a:[Chars;h]
    }
    let mut center: Vec<(usize, usize)> = Vec::new();
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if a[i][j] == '#'
                && a[i + 1][j + 1] == '#'
                && a[i - 1][j - 1] == '#'
                && a[i - 1][j + 1] == '#'
                && a[i + 1][j - 1] == '#'
            {
                center.push((i, j));
            }
        }
    }
    let mut ans: VecDeque<i64> = vec![0; h.min(w) + 1].into_iter().collect();
    for (i, j) in center {
        let mut size = 0;
        loop {
            let up = i as i64 - (size + 1);
            let down = i as i64 + (size + 1);
            let left = j as i64 - (size + 1);
            let right = j as i64 + (size + 1);
            if up < 0 || down >= h as i64 || left < 0 || right >= w as i64 {
                ans[size as usize] += 1;
                break;
            }
            let up = up as usize;
            let down = down as usize;
            let left = left as usize;
            let right = right as usize;
            if a[up][left] == '#'
                && a[up][right] == '#'
                && a[down][left] == '#'
                && a[down][right] == '#'
            {
                size += 1;
            } else {
                ans[size as usize] += 1;
                break;
            }
        }
    }
    ans.pop_front();
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
