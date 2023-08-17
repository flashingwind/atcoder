use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize;4*n-1],
    };
    let mut cnt = vec![0; n + 1];
    // println!("{:?}", arr);
    for a in arr.iter() {
        cnt[*a] += 1;
    }
    // println!("{:?}", cnt);
    for (i, c) in cnt.iter().enumerate().skip(1) {
        if *c < 4 {
            println!("{}", i);
            break;
        }
    }
}
