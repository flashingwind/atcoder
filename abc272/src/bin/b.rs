use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    // let mut attendee = Vec::new();
    let mut is_met = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {
            k: usize,
            x: [usize;k],
        };
        for (i, xx) in x.iter().enumerate() {
            for xx2 in x.iter().skip(i) {
                is_met[*xx - 1][*xx2 - 1] = true;
                is_met[*xx2 - 1][*xx - 1] = true;
            }
        }
        // attendee.push(x);
    }
    for (i1, p) in is_met.iter().enumerate() {
        for (i2, b) in p.iter().enumerate() {
            if i1 != i2 && !b {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
