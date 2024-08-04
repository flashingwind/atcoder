use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n],
    };
    let mut v = vec![];
    println!("{:?}", v);
    for i in 0..n {
        v.push(a[i]);
        println!("ADD a[{i}]={} {:?}", a[i], v);
        while let Some(b1) = v.pop() {
            if let Some(b2) = v.pop() {
                if b1 != b2 {
                    println!("  {b1}!={b2}");
                    println!("  {:?}", v);
                    println!("   pushed {b1},{b2} {:?}", v);
                    v.push(b2);
                    v.push(b1);
                    break;
                } else {
                    println!("  {b1}=={b2}");
                    println!("  {b1}!={b2}");
                    println!("  {:?}", v);
                    v.push(b1 + b2);
                    println!("   pushed {} {:?}", b1 + b2, v);
                }
            } else {
                v.push(b1);
                break;
            }
            println!("{:?}", v);
        }
    }
    println!("{:?}", v);
    println!("{}", v.len());
}
