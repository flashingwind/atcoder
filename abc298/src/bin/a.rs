use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    // let mut is_o = false;
    // for c in s.iter() {
    //     if *c == 'o' {
    //         is_o = true;
    //         break;
    //     }
    // }
    // if is_o {
    //     for c in s.iter() {
    //         if *c == 'x' {
    //             is_o = false;
    //             break;
    //         }
    //     }
    // }
    // if is_o {
    //     println!("Yes");
    // } else {
    //     println!("No");
    // }
    if s.contains(&'o') && !s.contains(&'x') {
        println!("Yes");
    } else {
        println!("No");
    }
}
