use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8;w];h],
    };
    // let alph = [
    //     'A', 'B', 'C', 'D', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    //     's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    // ];
    for l in a.iter() {
        for c in l.iter() {
            if *c == 0 {
                print!(".");
            } else {
                print!("{}", (*c + 'A' as u8 - 1) as char);
            }
        }
        println!();
    }
}
