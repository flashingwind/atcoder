use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        mut v: [(usize,usize);m],
    };
    // let mut nodes = vec![0usize; n + 1];
    for (a1, a2) in v.iter_mut() {
        if *a1 > *a2 {
            std::mem::swap(a2, a1);
        }
    }
    v.sort();
    // println!("{:?}", v);
    let mut cnt = 0;
    for (a1, b1) in v.iter() {
        for (a2, b2) in v.iter() {
            if *b1 == *a2 {
                // a1--b1=a2--b2
                for (a3, b3) in v.iter() {
                    if *a1 == *a3 && *b2 == *b3 && *a1 < *a2 && *b1 < *b2 {
                        // println!("a1:{a1}--b1:{b1}=a2:{a2}--b2:{b2}=b3:{b3}--a3:{a3}->");
                        cnt += 1;
                    }
                }
            }
        }
    }
    println!("{}", cnt);
}
