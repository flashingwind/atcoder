use proconio::input;

fn main() {
    input! {
        w: u64,
        h: u64,
        n: usize,
        mut sb: [(u64,u64);n],
        num_a: usize,
        mut a:[u64;num_a],
        num_b: usize,
        mut b:[u64;num_b],
    };
    a.sort();
    b.sort();
    a.insert(0, 0);
    b.insert(0, 0);
    a.push(w);
    a.push(h);
    let mut blk = vec![vec![0; a.len()]; b.len()];
    sb.sort_by(|a, b| a.0.cmp(&b.0));
    for &(x, y) in sb.iter() {
        let i = a.partition_point(|v| *v < x);
        let j = b.partition_point(|v| *v < y);
        println!("{i},{j}");
        println!("{i},{j}: {},{}", a[i], b[j]);
        blk[i][j] += 1;
    }
    let mut min = std::u64::MAX;
    let mut max = 0;
    for bb in blk.iter() {
        for b in bb.iter() {
            if *b < min {
                min = *b;
            }
            if max < *b {
                max = *b;
            }
        }
    }
    // println!("{:?}", blk);
    println!("{} {}", min, max);
}
