use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    };
    const V_A: i32 = 500;
    const V_B: i32 = 100;
    const V_C: i32 = 50;
    let mut num_a = 0;
    let mut count = 0;

    while num_a <= a {
        let mut num_b = 0;
        while num_b <= b {
            let mut num_c = 0;
            while num_c <= c {
                if x - V_A * num_a - V_B * num_b - V_C * num_c == 0 {
                    count += 1;
                }
                //println!("{}x{} {}x{} {}x{}", V_A, num_a, V_B, num_b, V_C, num_c);
                num_c += 1;
            }
            num_b += 1;
        }
        num_a += 1;
    }
    println!("{}", count)
}
