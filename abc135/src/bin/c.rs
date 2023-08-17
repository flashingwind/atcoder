use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n+1],
        mut b: [u64;n],
    };
    let mut cnt = 0;
    for i in 0..n {
        {
            let min_ab = b[i].min(a[i]);
            cnt += min_ab;
            a[i] -= min_ab;
            b[i] -= min_ab;
            // println!("{i}:");
            // println!("m ={:?}", a);
            // println!("hp={:?}", b);
            // println!("cnt={}", cnt);
        }
        {
            let min_ab = b[i].min(a[i + 1]);
            cnt += min_ab;
            a[i + 1] -= min_ab;
            b[i] -= min_ab;
            // println!("m2={:?}", a);
            // println!("hp={:?}", b);
            // println!("cnt={}", cnt);
        }
    }
    println!("{}", cnt);
}
