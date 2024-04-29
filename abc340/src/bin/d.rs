use proconio::input;

fn main() {
    input! {
        n: usize,
        mut abx: [(u64,u64,usize);n-1],
    };
    abx.insert(0, (0, 0, 0));
    let mut stage = vec![1000000000000000; n + 1];
    stage[0] = 0;
    stage[1] = 0;
    for i in 1..=(n - 1) {
        let (a, b, x) = abx[i];
        stage[i + 1] = stage[i + 1].min(stage[i] + a);
        // println!("stage[{}]=stage[{i}]+{a}={}", i + 1, stage[i] + a);
        stage[x] = stage[x].min(stage[i] + b);
        // stage[i + 1] = stage[i + 1].min(stage[i] + a);
        // println!("stage[{x}]=stage[{i}]+{b}={}", stage[i] + b);
        // println!("{i}:{:?}", stage);
    }
    for i in (1..=(n - 1)).rev() {
        let (a, b, x) = abx[i];
        stage[i + 1] = stage[i + 1].min(stage[i] + a);
        // println!("stage[{}]=stage[{i}]+{a}={}", i + 1, stage[i] + a);
        stage[x] = stage[x].min(stage[i] + b);
        // stage[i + 1] = stage[i + 1].min(stage[i] + a);
        // println!("stage[{x}]=stage[{i}]+{b}={}", stage[i] + b);
        // println!("{i}:{:?}", stage);
    }
    println!("{}", stage[n]);
}
