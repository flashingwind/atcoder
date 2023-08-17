use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(String,u32);n],
    };
    let mut min_age = p[0].1;
    let mut min_age_i = 0;
    for (i, (_, a)) in p.iter().enumerate() {
        if *a < min_age {
            min_age = *a;
            min_age_i = i;
        }
    }
    for ii in min_age_i..min_age_i + n {
        let i = ii % n;
        let name = p[i].0.to_owned();
        let a = p[i].1;
        println!("{}", name);
    }
}
