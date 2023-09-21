use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [Usize1;m],
    };
    let mut x_start = 0;
    let mut is_first = true;
    loop {
        let mut x_end = x_start;
        for e_i in edges.iter() {
            if *e_i == x_end {
                x_end = *e_i + 1;
            }
        }
        // println!("\nC:{x_start}..={x_end}");
        for i in (x_start..=x_end).rev() {
            if is_first {
                print!("{}", i + 1);
                is_first = false;
            } else {
                print!(" {}", i + 1);
            }
        }
        x_start = x_end + 1;
        if n <= x_start {
            break;
        }
    }
}
