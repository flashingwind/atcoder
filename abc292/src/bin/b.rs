use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        eve: [(usize,usize);q],
    };
    let mut player = vec![0; n + 1];
    for e in eve.iter() {
        if e.0 == 1 {
            player[e.1] += 1;
        } else if e.0 == 2 {
            player[e.1] += 2;
        } else if e.0 == 3 {
            if player[e.1] < 2 {
                println!("No");
            } else {
                println!("Yes");
            }
        }
    }
}
