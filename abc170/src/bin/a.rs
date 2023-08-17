use proconio::input;

fn main() {
    input! {
        x: [usize;5],
    };
    for (i, xi) in x.iter().enumerate() {
        if i + 1 != *xi {
            println!("{}", i + 1);
            break;
        }
    }
}
