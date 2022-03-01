#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn dig(r: usize, c: usize, dx: i32, dy: i32, area: &Vec<Vec<char>>, n: usize) -> bool {
    let mut nr = r.clone() as i32;
    let mut nc = c.clone() as i32;
    let mut cnt = 0;
    for _ in 0..6 {
        if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
            return false;
        }
        if area[nr as usize][nc as usize] == '#' {
            cnt += 1;
        }
        nr += dy;
        nc += dx;
    }
    let res = if cnt >= 4 { true } else { false };
    return res;
}

fn solve() {
    input! {
        n: usize,
        mut ss: [String;n],
    }
    let mut area: Vec<Vec<char>> = Vec::new();
    for i in ss {
        let str: Vec<char> = i.chars().collect();
        area.push(str);
    }
    let dx: Vec<i32> = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let dy: Vec<i32> = vec![0, -1, -1, -1, 0, 1, 1, 1];
    for i in 0..n * n {
        let r = i / n;
        let c = i % n;
        for j in 0..8 {
            if dig(r, c, dx[j], dy[j], &area, n) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn main() {
    solve();
}
