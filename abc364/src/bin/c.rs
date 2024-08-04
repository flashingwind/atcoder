use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        mut a: [u64;n],
        mut b: [u64;n],
    };
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    let mut sum_a=0;
    let mut sum_b=0;
    for i in 0..n{
        sum_a+=a[i];
        sum_b+=b[i];
        // println!("sum_a={sum_a} sum_b={sum_b}");
        if x<sum_a{
            println!("{}",i+1);
            return;
        }else if y<sum_b{
            println!("{}",i+1);
            return;
        }
    }
    println!("{}", n);
}