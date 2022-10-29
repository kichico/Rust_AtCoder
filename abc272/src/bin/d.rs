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
        n:usize,m:i64
    }
    let mut que: VecDeque<(i64, i64)> = VecDeque::new();
    let mut field = vec![vec![-1; n]; n];
    let mut dict: HashMap<i64, i64> = HashMap::new();
    for i in 0..=m.sqrt() + 1 {
        let j = m - (i * i);
        if j < 0 {
            break;
        }
        if j.sqrt() * j.sqrt() == j {
            dict.insert(i * i, j);
            dict.insert(j, i * i);
        }
    }
    field[0][0] = 0;
    let n = n as i64;
    que.push_back((0, 0));
    while !que.is_empty() {
        let (x, y) = que.front().unwrap().clone();
        que.pop_front();
        for (s, t) in &dict {
            let nx = x - s.sqrt();
            let ny = y - t.sqrt();
            if nx >= 0
                && ny >= 0
                && nx < n as i64
                && ny < n as i64
                && field[ny as usize][nx as usize] == -1
            {
                field[ny as usize][nx as usize] = field[y as usize][x as usize] + 1;
                que.push_back((nx, ny));
            }
            let nx = x + s.sqrt();
            let ny = y + t.sqrt();
            if nx >= 0
                && ny >= 0
                && nx < n as i64
                && ny < n as i64
                && field[ny as usize][nx as usize] == -1
            {
                field[ny as usize][nx as usize] = field[y as usize][x as usize] + 1;
                que.push_back((nx, ny));
            }
            let nx = x + s.sqrt();
            let ny = y - t.sqrt();
            if nx >= 0
                && ny >= 0
                && nx < n as i64
                && ny < n as i64
                && field[ny as usize][nx as usize] == -1
            {
                field[ny as usize][nx as usize] = field[y as usize][x as usize] + 1;
                que.push_back((nx, ny));
            }
            let nx = x - s.sqrt();
            let ny = y + t.sqrt();
            if nx >= 0
                && ny >= 0
                && nx < n as i64
                && ny < n as i64
                && field[ny as usize][nx as usize] == -1
            {
                field[ny as usize][nx as usize] = field[y as usize][x as usize] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    for i in 0..n as usize {
        println!("{}", field[i].iter().join(" "));
    }
}

fn main() {
    solve();
}
