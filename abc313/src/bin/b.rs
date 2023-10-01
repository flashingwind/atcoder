use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m],
    };
    let mut set = vec![true; n];
    for (_a, b) in ab.iter() {
        set[*b - 1] = false;
    }
    let mut cnt = 0;
    let mut strongest = 0;
    for (i, b) in set.iter().enumerate() {
        if *b {
            cnt += 1;
            strongest = i;
        }
    }
    if cnt == 1 {
        println!("{}", strongest + 1);
    } else {
        println!("-1");
    }
}
