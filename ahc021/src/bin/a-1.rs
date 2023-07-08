#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
use rand::*;
#[allow(non_snake_case)]

fn calculate_score(balls: &Vec<Vec<i64>>, k: usize) -> i64 {
    let mut error = false;
    let mut cnt = 0;
    for i in 0..balls.len() - 1 {
        for j in 0..balls[i].len() {
            if balls[i][j] > balls[i + 1][j] {
                error = true;
                cnt += 1;
            }
            if balls[i][j] > balls[i + 1][j + 1] {
                error = true;
                cnt += 1;
            }
        }
    }
    if error {
        return (50000 - 50 * cnt).max(0);
    } else {
        return 100000 - 5 * k as i64;
    }
}

fn solve() {
    let n = 30;
    let mut balls_base: Vec<Vec<i64>> = vec![Vec::new(); n];
    let mut nums: BTreeSet<(i64, (usize, usize))> = BTreeSet::new();
    let mut rng = rand::thread_rng();

    for i in 1..=n {
        input! {ball: [i64; i]}
        for j in 0..i {
            nums.insert((ball[j], (i - 1, j)));
        }
        balls_base[i - 1] = ball;
    }
    let mut cumsum: Vec<usize> = Vec::new();
    let mut base = 0;
    for i in 0..n {
        base += i;
        cumsum.push(base);
    }
    let iteration = 100000;
    let mut stock: Vec<(i64, Vec<Vec<usize>>)> = vec![(0, Vec::new()); iteration];
    for it in 0..iteration {
        let mut ans: Vec<Vec<usize>> = Vec::new();
        let mut cnt = 0;
        let mut balls = balls_base.clone();
        while ans.len() < 40 && cnt <= 1e5 as i64 {
            let idx = rng.gen_range(0, 435) as usize;
            let row = cumsum.upper_bound(&idx) - 1;
            let col = idx - cumsum[row];
            let mut lower = balls[row + 1].clone();
            let mut higher = balls[row].clone();
            if higher[col] > lower[col] {
                swap(&mut higher[col], &mut lower[col]);
                ans.push(vec![row, col, row + 1, col]);
            } else if higher[col] > lower[col + 1] {
                swap(&mut higher[col], &mut lower[col + 1]);
                ans.push(vec![row, col, row + 1, col + 1]);
            } else if higher[col] > lower[col] && higher[col] > lower[col + 1] {
                if lower[col] > lower[col + 1] {
                    swap(&mut higher[col], &mut lower[col + 1]);
                    ans.push(vec![row, col, row + 1, col + 1]);
                } else {
                    swap(&mut higher[col], &mut lower[col]);
                    ans.push(vec![row, col, row + 1, col]);
                }
            }
            balls[row + 1] = lower;
            balls[row] = higher;
            cnt += 1;
        }
        let score = calculate_score(&balls, ans.len());
        stock[it] = (score, ans);
    }
    stock.sort_by(|x, y| x.0.cmp(&y.0));
    let sub = stock.iter().next_back().unwrap();

    println!("{}", sub.1.len());
    for i in 0..sub.1.len() {
        println!("{}", sub.1[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
