use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    let mut num_d = 0u32;
    {
        let mut nn = n;
        while 0 < nn {
            num_d += 1;
            nn /= 10;
        }
        // println!("num_d={num_d}");
    }
    let mut cnt = 0;
    if num_d % 2 != 0 {
        cnt += n - 10u32.pow(num_d - 1) + 1;
        // println!("num_d={num_d}");
        // println!("cnt={cnt}");
        // println!("cnt+={}", n - 10u32.pow(num_d - 1) + 1);
        num_d -= 1;
    }
    while 1 <= num_d {
        // println!("num_d={num_d}");
        if num_d % 2 != 0 {
            cnt += 10u32.pow(num_d) - 10u32.pow(num_d - 1);
            // println!("cnt={cnt}");
            // println!("cnt+={}", 10u32.pow(num_d) - 10u32.pow(num_d - 1));
        }
        num_d -= 1;
    }
    println!("{}", cnt);
}
