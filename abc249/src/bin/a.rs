use proconio::input;

fn main() {
    input! {
        a_time: u32,
        a_speed: u32,
        a_stop: u32,
        b_time: u32,
        b_speed: u32,
        b_stop: u32,
        x: u32,
    };
    let mut d_a = 0;

    let mut t_b = x;
    d_a += a_speed * (t_b / (a_time + a_stop)) * a_time;
    t_b %= a_time + a_stop;
    // println!("d_a={d_a}");
    if a_time < t_b {
        d_a += a_time * a_speed;
        // t_a -= a_time;
    } else {
        d_a += t_b * a_speed;
        // t_a = 0;
    }
    // println!("d_a={d_a}");

    let mut d_b = 0;

    let mut t_b: u32 = x;
    d_b += b_speed * (t_b / (b_time + b_stop)) * b_time;
    t_b %= b_time + b_stop;
    // println!("d_b={d_b}");
    if b_time < t_b {
        d_b += b_time * b_speed;
        // t_a -= b_time;
    } else {
        d_b += t_b * b_speed;
        // t_a = 0;
    }
    // println!("d_b={d_b}");

    if d_a == d_b {
        println!("Draw");
    } else if d_a < d_b {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
