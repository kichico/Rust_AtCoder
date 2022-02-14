#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn grid<T: Clone + Copy>(h: usize, w: usize, ini: T) -> Vec<Vec<T>> {
    let ret: Vec<Vec<T>> = vec![vec![ini; w]; h];
    return ret;
}

fn dfs(
    x: i64,
    y: i64,
    field: Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    sx: i64,
    sy: i64,
) -> i64 {
    let mut ans = 0;

    ans
}
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        h: usize,
        w: usize,
        f: [String;h],
    }
    let mut field = grid::<char>(h, w, '0');
    for i in 0..h {
        let s: Vec<char> = f[i].as_str().chars().collect();
        for j in 0..w {
            field[i][j] = s[j];
        }
    }
    let mut ans = -1;
    for p in 0..h * w {
        let x = p % (h);
        let y = p / (h);
        let mut visited = grid::<bool>(h, w, false);
        ans = max(
            dfs(x as i64, y as i64, field, &mut visited, x as i64, y as i64),
            ans,
        );
        dbg!(ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
