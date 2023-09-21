use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32;n],
    };
    input! {
        num_q: usize,
    };
    for _ in 0..num_q {
        input! {
            cat: i32,
        };
        if cat == 1 {
            //edit
            input! {
                mut q_i: usize,
                x: i32,
            };
            q_i -= 1;
            a[q_i] = x;
        } else if cat == 2 {
            //show
            input! {
                mut q_i: usize,
            };
            q_i -= 1;
            println!("{}", a[q_i]);
        }
    }
}
