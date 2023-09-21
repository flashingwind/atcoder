use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    for tsuru in 0..=x {
        if tsuru * 2 + (x - tsuru) * 4 == y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
