use itertools::Itertools;
#[allow(unused_imports)]
use itertools::*;
use maplit::{hashmap, hashset};
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use num_traits::Pow;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use rand::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
use std::mem::take;
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;
use std::time::Instant;
pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub ty: u64,
    pub n: usize,
    pub a: Vec<Vec<i32>>,
    pub vs: Vec<Vec<char>>,
    pub hs: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.ty, self.n)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.vs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n - 1 {
            writeln!(f, "{}", self.hs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n {
            writeln!(f, "{}", self.a[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
        a: [[i32; n]; n],
    }
    Input { ty, n, a, vs, hs }
}

pub fn parse_input_fixed(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
    }
    for i in 0..n {
        assert_eq!(vs[i].len(), n - 1);
    }
    for i in 0..n - 1 {
        assert_eq!(hs[i].len(), n);
    }
    Input {
        ty,
        n,
        a: vec![],
        vs,
        hs,
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const DIJ2: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
pub struct Output {
    pub start: (usize, usize, usize, usize),
    pub out: Vec<(bool, usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut ss = f.split_whitespace();
    let start = (
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
    );
    while let Some(mv) = ss.next() {
        let do_swap = if mv == "1" {
            true
        } else if mv != "0" {
            return Err(format!("Invalid action: {}", mv));
        } else {
            false
        };
        let dir1 = read(ss.next(), '.'..='Z')?;
        let dir2 = read(ss.next(), '.'..='Z')?;
        let dir1 = if dir1 == '.' {
            !0
        } else if let Some(dir1) = DIRS.iter().position(|&d| d == dir1) {
            dir1
        } else {
            return Err(format!("Invalid direction: {}", dir1));
        };
        let dir2 = if dir2 == '.' {
            !0
        } else if let Some(dir2) = DIRS.iter().position(|&d| d == dir2) {
            dir2
        } else {
            return Err(format!("Invalid direction: {}", dir2));
        };
        out.push((do_swap, dir1, dir2));
    }
    if out.len() > 4 * input.n * input.n {
        return Err("Too many actions".to_owned());
    }
    Ok(Output { start, out })
}

fn can_move(
    N: usize,
    h: &Vec<Vec<char>>,
    v: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    dir: usize,
) -> bool {
    let (di, dj) = DIJ[dir];
    // println!("i:{} j:{}", i, j);
    // println!("di:{} dj:{}", di, dj);
    let i2 = i + di;
    let j2 = j + dj;
    // println!("i2:{} j2:{}", i2, j2);
    // println!("----------");
    if i2 >= N || j2 >= N {
        return false;
    }
    let i2 = i2 as usize;
    let j2 = j2 as usize;
    if di == 0 {
        v[i][j.min(j2)] == '0'
    } else {
        h[i.min(i2)][j] == '0'
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out.start, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

fn compute_diff(input: &Input, a: &Vec<Vec<i32>>) -> i64 {
    let mut diff = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            for dir in 1..=2 {
                if can_move(input.n, &input.hs, &input.vs, i, j, dir) {
                    let d = (a[i][j] - a[i + DIJ[dir].0][j + DIJ[dir].1]) as i64;
                    diff += d * d;
                }
            }
        }
    }
    diff
}

pub fn compute_score_details(
    input: &Input,
    start: (usize, usize, usize, usize),
    out: &[(bool, usize, usize)],
) -> (i64, String, (Vec<Vec<i32>>, (usize, usize), (usize, usize))) {
    let mut a = input.a.clone();
    let mut p1 = (start.0, start.1);
    let mut p2 = (start.2, start.3);
    let before = compute_diff(&input, &a);
    for &(do_swap, dir1, dir2) in out {
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        if dir1 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                return (0, format!("Invalid move: {}", DIRS[dir1]), (a, p1, p2));
            }
            p1.0 += DIJ[dir1].0;
            p1.1 += DIJ[dir1].1;
        }
        if dir2 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                return (0, format!("Invalid move: {}", DIRS[dir2]), (a, p1, p2));
            }
            p2.0 += DIJ[dir2].0;
            p2.1 += DIJ[dir2].1;
        }
    }
    let after = compute_diff(&input, &a);
    let score =
        ((1e6 * (f64::log2(before as f64) - f64::log2(after as f64))).round() as i64).max(1);
    (score, String::new(), (a, p1, p2))
}
fn compute_around_diff(
    input: &Input,
    i: usize,
    j: usize,
    dir_converter: &HashMap<char, usize>,
) -> i32 {
    let mut diff_sum = 0;
    for dir in DIRS {
        let d = *dir_converter.get(&dir).unwrap();
        if can_move(input.n, &input.hs, &input.vs, i, j, d) {
            let ny = i + DIJ[d].0;
            let nx = j + DIJ[d].1;
            diff_sum += (input.a[i][j] - input.a[ny][nx]).pow(2);
        }
    }
    diff_sum
}
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: usize) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
fn process(
    takahashi: (usize, usize),
    aoki: (usize, usize),
    changeable_input: &mut Input,
    dir_converter: &HashMap<char, usize>,
    input: &Input,
    current_result: &mut Vec<Vec<char>>,
) -> ((usize, usize), (usize, usize)) {
    let mut new_takahashi = takahashi.clone();
    let mut new_aoki = aoki.clone();
    let mut rng = rand::thread_rng();
    let mut current_step = Vec::new();
    let (ty, tx) = takahashi;
    let (ay, ax) = aoki;
    let t_score = compute_around_diff(&changeable_input, takahashi.0, takahashi.1, &dir_converter);
    let a_score = compute_around_diff(&changeable_input, aoki.0, aoki.1, &dir_converter);
    let tmp1 = take(&mut changeable_input.a[ty][tx]);
    let tmp2 = take(&mut changeable_input.a[ay][ax]);
    changeable_input.a[ty][tx] = tmp2.clone();
    changeable_input.a[ay][ax] = tmp1.clone();
    let t_score_after =
        compute_around_diff(&changeable_input, takahashi.0, takahashi.1, &dir_converter);
    let a_score_after = compute_around_diff(&changeable_input, aoki.0, aoki.1, &dir_converter);
    if t_score + a_score < t_score_after + a_score_after {
        let tmp1 = take(&mut changeable_input.a[ty][tx]);
        let tmp2 = take(&mut changeable_input.a[ay][ax]);
        changeable_input.a[ty][tx] = tmp2.clone();
        changeable_input.a[ay][ax] = tmp1.clone();
        current_step.push('0');
    } else {
        current_step.push('1');
    }
    let takahashi_move = rng.gen_range(0..4);
    let aoki_move = rng.gen_range(0..4);
    if can_move(input.n, &input.hs, &input.vs, ty, tx, takahashi_move) {
        let nty = ty + DIJ[takahashi_move].0;
        let ntx = tx + DIJ[takahashi_move].1;
        new_takahashi = (nty, ntx);
        current_step.push(DIRS[takahashi_move]);
    } else {
        current_step.push('.');
    }
    if can_move(input.n, &input.hs, &input.vs, ay, ax, aoki_move) {
        let nay = ay + DIJ[aoki_move].0;
        let nax = ax + DIJ[aoki_move].1;
        new_aoki = (nay, nax);
        current_step.push(DIRS[aoki_move]);
    } else {
        current_step.push('.');
    }
    current_result.push(current_step.clone());
    (new_takahashi, new_aoki)
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        t:u64,N:usize,v_wall:[Chars;N],h_wall:[Chars;N-1],mut number:[[i32;N];N]
    }
    // pub struct Output {
    //     pub start: (usize, usize, usize, usize),
    //     pub out: Vec<(bool, usize, usize)>,
    // }
    let start = Instant::now();
    let input = Input {
        ty: t,
        n: N,
        a: number,
        vs: v_wall,
        hs: h_wall,
    };
    let mut ans: String = String::new();
    let mut highest = 0;
    while start.elapsed().as_millis() < 1950 {
        let mut changeable_input = input.clone();
        let mut output: String = "".to_string();
        let mut rng = rand::thread_rng();
        let mut takahashi = (rng.gen_range(0..N), rng.gen_range(0..N));
        let mut aoki = (rng.gen_range(0..N), rng.gen_range(0..N));
        let s = &(vec![
            takahashi.0.to_string(),
            takahashi.1.to_string(),
            aoki.0.to_string(),
            aoki.1.to_string(),
        ]
        .iter()
        .join(" ")
            + "\n");
        output += s;
        let mut first_result = Vec::new();
        let dir_converter = hashmap! {'U'=>0,'D'=>1,'L'=>2,'R'=>3};
        for _i in 0..4 * N * N {
            let mut current_step = Vec::new();
            let (ty, tx) = takahashi;
            let (ay, ax) = aoki;
            let t_score =
                compute_around_diff(&changeable_input, takahashi.0, takahashi.1, &dir_converter);
            let a_score = compute_around_diff(&changeable_input, aoki.0, aoki.1, &dir_converter);
            let tmp1 = take(&mut changeable_input.a[ty][tx]);
            let tmp2 = take(&mut changeable_input.a[ay][ax]);
            changeable_input.a[ty][tx] = tmp2.clone();
            changeable_input.a[ay][ax] = tmp1.clone();
            let t_score_after =
                compute_around_diff(&changeable_input, takahashi.0, takahashi.1, &dir_converter);
            let a_score_after =
                compute_around_diff(&changeable_input, aoki.0, aoki.1, &dir_converter);
            if t_score + a_score < t_score_after + a_score_after {
                let tmp1 = take(&mut changeable_input.a[ty][tx]);
                let tmp2 = take(&mut changeable_input.a[ay][ax]);
                changeable_input.a[ty][tx] = tmp2.clone();
                changeable_input.a[ay][ax] = tmp1.clone();
                current_step.push('0');
            } else {
                current_step.push('1');
            }
            let takahashi_move = rng.gen_range(0..4);
            let aoki_move = rng.gen_range(0..4);
            if can_move(input.n, &input.hs, &input.vs, ty, tx, takahashi_move) {
                let nty = ty + DIJ[takahashi_move].0;
                let ntx = tx + DIJ[takahashi_move].1;
                takahashi = (nty, ntx);
                current_step.push(DIRS[takahashi_move]);
            } else {
                current_step.push('.');
            }
            if can_move(input.n, &input.hs, &input.vs, ay, ax, aoki_move) {
                let nay = ay + DIJ[aoki_move].0;
                let nax = ax + DIJ[aoki_move].1;
                aoki = (nay, nax);
                current_step.push(DIRS[aoki_move]);
            } else {
                current_step.push('.');
            }
            first_result.push(current_step.clone());
            output += &(current_step.iter().join(" ") + "\n");
        }
        let op = parse_output(&input, &output).ok().unwrap();
        let this_step_score = compute_score(&input, &op).0;
        if this_step_score > highest {
            highest = this_step_score;
            ans = output.clone();
        }
    }
    println!("{}", ans);
    println!("{}", highest);
}
fn main() {
    solve();
}
