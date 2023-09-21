use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut f: [(usize,usize);n],
    };
    f.sort();
    let mut i = 0;
    for (fi, addi) in f.iter() {
        // print!("i={i} k={k}");
        if fi - i <= k {
            k -= *fi - i;
            k += *addi;
            i = *fi;
            // println!("->{i} k={k}");
        } else {
            break;
        }
    }
    i += k;
    println!("{}", i);
}
