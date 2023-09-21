use proconio::input;

fn main() {
    input! {
        pp1: char,
        pp2: char,
    };
    let p1 = (pp1 as u8 - 'A' as u8) as usize;
    let p2 = (pp2 as u8 - 'A' as u8) as usize;
    let v = vec![0, 3, 4, 8, 9, 14, 23];
    if p1 < p2 {
        println!("{}", v[p2] - v[p1]);
    } else {
        println!("{}", v[p1] - v[p2]);
    }
}
