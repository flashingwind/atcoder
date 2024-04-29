use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    //3√(10^18)=1_000_001_usize
    let mut max = 0;
    let mut tri = vec![false; n];
    let mut tri_stack: Vec<usize> = vec![];
    for i in 0..=n {
        let i3 = i.pow(3);
        let tri_stack_old = tri_stack.clone();
        // println!("{}:", i);
        for &j in tri_stack_old.iter() {
            let k = j * i3;
            if n < k {
                break;
            }
            tri[k] = true;
            tri_stack.push(k);
            if max < k {
                let str1 = k.to_string();
                let str2: String = str1.chars().rev().collect();
                if str2 == str1 {
                    max = k;
                    println!("  {}, max={max}", k);
                }
            }
        }
    }
    println!("{}", max);
}
