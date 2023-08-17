use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [u32;n],
    };
    for a in arr.iter() {
        if *a % 2 == 0 {
            if *a % 3 == 0 || *a % 5 == 0 {
            } else {
                println!("DENIED");
                return;
            }
        }
    }
    println!("APPROVED");
}
