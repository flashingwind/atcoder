use proconio::input;

fn main() {
    input! {
        mut a: u32,
        b: u32,
        k: u32,
    };
    let mut cnt = 1;
    for n in a..=b {
        println!("{}", n);
        if cnt == k || n == b {
            a = n + 1;
            // println!("a={a}");
            break;
        }
        cnt += 1;
    }
    cnt = 1;
    let mut out = Vec::new();
    for n in (a..=b).rev() {
        out.push(n);
        if cnt == k {
            break;
        }
        cnt += 1;
    }
    out.sort();
    for n in out.iter() {
        println!("{}", n);
    }
}
