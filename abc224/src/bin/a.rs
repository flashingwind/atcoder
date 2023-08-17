use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        //d: [i32;n],
    };
    let mut binding = s.iter();
    let mut s_last = binding.by_ref().rev().take(3).rev();
    if *s_last.next().unwrap() == 'i'
        && *s_last.next().unwrap() == 's'
        && *s_last.next().unwrap() == 't'
    {
        println!("ist");
    } else {
        println!("er");
    }
}
