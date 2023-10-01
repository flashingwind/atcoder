use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars;n],
    };
    let len = s.len();
    s.sort();
    if s.iter().unique().count() != len {
        println!("No");
        return;
    }
    for ss in s.iter() {
        if !['H', 'D', 'C', 'S'].contains(&ss[0]) {
            println!("No");
            return;
        }
        if ![
            'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
        ]
        .contains(&ss[1])
        {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
