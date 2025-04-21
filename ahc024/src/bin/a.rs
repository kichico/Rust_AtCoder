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

#[derive(Debug, Clone)]
pub struct RollbackUnionFind {
    parent: Vec<i64>,
    history: Vec<(i64, i64)>,
    inner_snap: usize,
}

impl RollbackUnionFind {
    pub fn new(n: usize) -> Self {
        let parent = vec![-1i64; n];
        let history: Vec<(i64, i64)> = Vec::new();
        let inner_snap = 0;
        return RollbackUnionFind {
            parent,
            history,
            inner_snap,
        };
    }
    pub fn find_root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        self.parent[x] = self.find_root(self.parent[x] as usize) as i64;
        return self.parent[x] as usize;
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let mut xpar = self.find_root(x);
        let mut ypar = self.find_root(y);
        if xpar == ypar {
            return;
        }
        self.history.push((xpar as i64, self.parent[xpar]));
        self.history.push((ypar as i64, self.parent[ypar]));
        if self.parent[xpar] > self.parent[y] {
            std::mem::swap(&mut xpar, &mut ypar);
        }
        let x = xpar;
        let y = ypar;
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i64;
    }
    pub fn size(&mut self, x: usize) -> i64 {
        let x = self.find_root(x);
        return -self.parent[x];
    }
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        let x = self.find_root(x);
        let y = self.find_root(y);
        return x == y;
    }
    pub fn undo(&mut self) {
        if let Some((x, data)) = self.history.pop() {
            self.parent[x as usize] = data;
        }
        if let Some((x, data)) = self.history.pop() {
            self.parent[x as usize] = data;
        }
    }
    pub fn snapshot(&mut self) {
        self.inner_snap = self.history.len() >> 1;
    }
    pub fn recorded_state(&self) -> usize {
        return self.history.len() >> 1;
    }
    pub fn roll_back(&mut self, state: i64) {
        let mut target = if state == -1 {
            self.inner_snap
        } else {
            state as usize
        };
        target <<= 1;
        assert!(target <= self.history.len());
        while target < self.history.len() {
            self.undo();
        }
    }
}

struct Rectangle {
    right: HashSet<usize>,
    left: HashSet<usize>,
    top: HashSet<usize>,
    bottom: HashSet<usize>,
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,color:[[usize;n];n]
    }
    let mut ans = color.clone();

    //let mut cnt = ans.clone();
    let mut dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let mut dy = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let mut field = vec![vec![0; n + 2]; n + 2];
    for r in 0..n {
        for c in 0..n {
            field[r + 1][c + 1] = color[r][c];
        }
    }
    for i in 0..10 {
        for r in 1..=n {
            'next: for c in 1..=n {
                if field[r][c] == 0 {
                    continue;
                }
                let mut outside = false;
                for k in 0..8 {
                    let nx = c as i64 + dx[k];
                    let ny = r as i64 + dy[k];
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if field[ny][nx] != field[r][c] && field[ny][nx] != 0 {
                        continue 'next;
                    } else if field[ny][nx] == 0 {
                        outside = true;
                    }
                }
                if outside {
                    field[r][c] = 0;
                }
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            print!("{} ", field[i][j]);
        }
        println!();
    }
}
use std::{
    fmt::format,
    fs,
    io::{self, BufRead, BufWriter, Write},
};

fn local() -> Result<(), Box<dyn std::error::Error>> {
    for case in 0..100 {
        let mut buffer = String::new();
        let n = 50;
        let m = 100;
        let mut color: Vec<Vec<usize>> = vec![Vec::new(); n];
        let case_ = format!("{:>04}", case);
        for (i, result) in
            enumerate(io::BufReader::new(fs::File::open(format!("input/{}.txt", case_))?).lines())
        {
            let line = result?;
            let it = line.split_whitespace();
            if i == 0 {
                continue;
            }
            for val in it {
                color[i - 1].push(val.to_string().parse().unwrap());
            }
        }
        let mut ans = color.clone();
        //let mut cnt = ans.clone();
        let mut dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
        let mut dy = vec![0, 1, 1, 1, 0, -1, -1, -1];
        let mut field = vec![vec![0; n + 2]; n + 2];
        for r in 0..n {
            for c in 0..n {
                field[r + 1][c + 1] = color[r][c];
            }
        }
        for i in 0..10 {
            for r in 1..=n {
                'next: for c in 1..=n {
                    if field[r][c] == 0 {
                        continue;
                    }
                    let mut outside = false;
                    for k in 0..8 {
                        let nx = c as i64 + dx[k];
                        let ny = r as i64 + dy[k];
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if field[ny][nx] != field[r][c] && field[ny][nx] != 0 {
                            continue 'next;
                        } else if field[ny][nx] == 0 && k % 2 == 0 {
                            outside = true;
                        }
                    }
                    if outside {
                        field[r][c] = 0;
                    }
                }
            }
        }
        //let mut writer = io::BufWriter::new(fs::File::create(format!("output/{}.txt", case_))?);
        let mut file = fs::File::create(format!("output/{}.txt", case_))?;
        for i in 1..=n {
            for j in 1..=n {
                write!(file, "{} ", field[i][j]);
            }
            writeln!(file);
        }
    }
    Ok(())
}
fn main() {
    //solve();
    local();
}
