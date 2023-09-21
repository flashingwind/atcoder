use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
                a: [i32; n],
        };
        let mut cnt = 0;
        for a_i in a.iter() {
            if *a_i % 2 == 1 {
                cnt += 1;
            }
        }
        println!("{}", cnt);
    }
}
