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
        h:usize,w:usize,field:[String;h]
    }
    let mut tmp = vec![vec!['0'; w]; h];
    for i in 0..h {
        let cv: Vec<char> = field[i].chars().collect();
        for j in 0..w {
            tmp[i][j] = cv[j];
        }
    }
    let field = tmp;
    let mut visited = vec![vec![false; w]; h];
    let (mut x, mut y) = (0, 0);
    loop {
        let dir = field[y][x];
        if visited[y][x] {
            println!("-1");
            return;
        }
        visited[y][x] = true;
        if dir == 'R' && x != w - 1 {
            if visited[y][x + 1] {
                println!("-1");
                return;
            }
            x += 1;
            continue;
        } else if dir == 'L' && x != 0 {
            if visited[y][x - 1] {
                println!("-1");
                return;
            }
            x -= 1;
            continue;
        } else if dir == 'U' && y != 0 {
            if visited[y - 1][x] {
                println!("-1");
                return;
            }
            y -= 1;
            continue;
        } else if dir == 'D' && y != h - 1 {
            if visited[y + 1][x] {
                println!("-1");
                return;
            }
            y += 1;
            continue;
        }
        println!("{} {}", y + 1, x + 1);
        return;
    }
}

fn main() {
    solve();
}
