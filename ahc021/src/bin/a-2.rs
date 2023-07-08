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
    let n = 30;
    let mut balls: Vec<Vec<i64>> = vec![Vec::new(); n];
    let mut nums: BTreeSet<(i64, (usize, usize))> = BTreeSet::new();
    for i in 1..=n {
        input! {ball: [i64; i]}
        for j in 0..i {
            nums.insert((ball[j], (i - 1, j)));
        }
        balls[i - 1] = ball;
    }
    let mut ans: Vec<Vec<usize>> = Vec::new();
    for (num, (mut row, mut col)) in nums {
        while row > 0 {
            let mut muted = false;
            if col == 0 {
                if num > balls[row - 1][col] {
                    let tmp = balls[row - 1][col].clone();
                    balls[row - 1][col] = balls[row][col].clone();
                    balls[row][col] = tmp;
                    ans.push(vec![row, col, row - 1, col]);
                    row -= 1;
                    muted = true;
                }
            } else if col == balls[row].len() - 1 {
                if num > balls[row - 1][col - 1] {
                    let tmp = balls[row - 1][col - 1].clone();
                    balls[row - 1][col - 1] = balls[row][col].clone();
                    balls[row][col] = tmp;
                    ans.push(vec![row, col, row - 1, col - 1]);
                    row -= 1;
                    col -= 1;
                    muted = true;
                }
            } else {
                if num > balls[row - 1][col] && num > balls[row - 1][col - 1] {
                    if balls[row - 1][col - 1] < balls[row - 1][col] {
                        let tmp = balls[row - 1][col].clone();
                        balls[row - 1][col] = balls[row][col].clone();
                        balls[row][col] = tmp;
                        ans.push(vec![row, col, row - 1, col]);
                        row -= 1;
                        muted = true;
                    } else {
                        let tmp = balls[row - 1][col - 1].clone();
                        balls[row - 1][col - 1] = balls[row][col].clone();
                        balls[row][col] = tmp;
                        ans.push(vec![row, col, row - 1, col - 1]);
                        col -= 1;
                        muted = true;
                    }
                } else if num > balls[row - 1][col] {
                    let tmp = balls[row - 1][col].clone();
                    balls[row - 1][col] = balls[row][col].clone();
                    balls[row][col] = tmp;
                    ans.push(vec![row, col, row - 1, col]);
                    row -= 1;
                    muted = true;
                } else if num > balls[row - 1][col - 1] {
                    let tmp = balls[row - 1][col - 1].clone();
                    balls[row - 1][col - 1] = balls[row][col].clone();
                    balls[row][col] = tmp;
                    ans.push(vec![row, col, row - 1, col - 1]);
                    row -= 1;
                    col -= 1;
                    muted = true;
                }
            }
            if !muted {
                break;
            }
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{}", ans[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
