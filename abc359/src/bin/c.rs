use proconio::input;

fn main() {
    input! {
        mut s: (usize,usize),
        mut t: (usize,usize),
    };
    if s.1.abs_diff(t.1) <= s.0.abs_diff(t.0) * 2 {
        println!("{}", s.1.abs_diff(t.1));
    } else {
        let mut cost = 0;
        cost += s.1.abs_diff(t.1);
        cost += s.0.abs_diff(t.0) - s.1.abs_diff(t.1) / 2;
        println!("cost");
    }
}
