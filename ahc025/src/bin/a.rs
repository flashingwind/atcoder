use proconio::{input, source::line::LineSource};
use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng};
use std::{
    io::{self, BufReader},
    time::Instant,
};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    let start = Instant::now();
    let mut rng = rand::thread_rng();
    input! {
        n: usize,
        d: usize,
        mut q: usize,
    };
    let mut w = vec![u32::MAX / (32 * n as u32); n];
    for qq in 0..q % (d - 1) {
        div_and_query(&mut w, 2, &mut rng);
        // terminate
        let t_diff = start.elapsed();
        if t_diff.as_nanos() > 1_900_000_000 {
            for _ in 0..q - qq {
                println!("1 1 1 2");
            }
            return;
        }
        println!("#q={qq}");
    }
    q -= q % (d - 1);
    while d - 1 <= q {
        println!("#q={q}");
        div_and_query(&mut w, d, &mut rng);
        // terminate
        let t_diff = start.elapsed();
        if t_diff.as_nanos() > 1_900_000_000 {
            for _ in 0..q {
                println!("1 1 1 2");
            }
            return;
        }
        q -= d - 1;
    }
    let div_i = div(&w, d, &mut rng);
    print_indices(&div_i, n, false);
    println!();
}

fn div(item: &Vec<u32>, d: usize, rng: &mut ThreadRng) -> Vec<Vec<usize>> {
    let uniif = Uniform::new(0, d);
    let n = item.len();
    // let mut unif = Uniform::from(0..d);
    let mut ret = vec![vec![]; d];
    let sum = item.iter().sum::<u32>();
    let th = sum / d as u32 + 1;
    let mut weights = vec![0; d];
    for i in 0..n {
        for cnt in 0..d {
            let dd = uniif.sample(rng) % d;
            if weights[dd] <= th || cnt == d - 1 {
                weights[dd] += item[i];
                ret[dd].push(i);
                break;
            }
        }
    }
    // println!("#c div: {:?}", ret);
    ret
}

fn recalc(sign: char, w: &mut Vec<u32>, div_i: &Vec<Vec<usize>>) {
    let d = div_i.len();
    let mut sum_w = vec![0; d];
    for dd in 0..d {
        for &i in div_i[dd].iter() {
            sum_w[dd] += w[i];
        }
    }
    if sign == '=' {
    } else if sign == '>' {
        // println!("sum: s1:{}-s0:{}", sum_w[1], sum_w[0]);
        let diff = (sum_w[1] - sum_w[0]) / (w.len() as u32 / 2) / 10;
        let avg = sum_w[1] / (w.len() as u32 / 2);
        for &i in div_i[1].iter() {
            w[i] += diff * (w[i] / sum_w[1]) + avg / 100;
        }
    } else if sign == '<' {
        // println!("sum: s0:{}-s1:{}", sum_w[0], sum_w[1]);
        let diff = (sum_w[0] - sum_w[1]) / (w.len() as u32 / 2) / 10;
        let avg = sum_w[0] / (w.len() as u32 / 2);
        for &i in div_i[0].iter() {
            w[i] += diff * (w[i] / sum_w[0]) + avg / 100;
        }
    }
}

fn div_and_query(w: &mut Vec<u32>, d: usize, rng: &mut ThreadRng) {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    let div_i = div(w, d, rng);
    for dd in 0..d - 1 {
        print_q(&div_i[dd], &div_i[dd + 1]);
        input! {
            sign: char,
        };
        recalc(sign, w, &div_i);
        print_indices(&div(&w, d, rng), w.len(), true);
    }
}

fn print_q(idx1: &Vec<usize>, idx2: &Vec<usize>) {
    print!("{} {}", idx1.len(), idx2.len());
    for &i in idx1.iter() {
        print!(" {}", i);
    }
    for &i in idx2.iter() {
        print!(" {}", i);
    }
    println!();
}

fn print_indices(div_i: &Vec<Vec<usize>>, n: usize, is_debug: bool) {
    // debug output
    let mut cat = vec![0; n];
    for dd in 0..div_i.len() {
        for &i in div_i[dd].iter() {
            cat[i] = dd;
        }
    }
    for (i, &c) in cat.iter().enumerate() {
        if i == 0 && is_debug {
            print!("#c {}", c);
        } else if i == 0 {
            print!("{}", c);
        } else {
            print!(" {}", c);
        }
    }
    println!();
}
