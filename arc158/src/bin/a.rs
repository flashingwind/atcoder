use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for _d in 0..n {
        input! {
            mut x: [u32;3],
        };
        x.sort();
        let min_x = x[0].min(x[1]).min(x[2]);
        x[0] -= min_x;
        x[1] -= min_x;
        x[2] -= min_x;
        let num_div = x[2] / 2;
        // println!("d={_d}:0: {:?}", x);
        if x[0] == 0 && x[1] == 0 && x[2] == 0 {
            println!("0");
            continue;
        }
        for t in 1..=num_div {
            if x[0] == 0 && x[1] == 0 && x[2] == 0 {
                println!("{}", t - 1);
                break;
            }
            // println!("{t}: {:?}", x);
            if 2 <= x[2] && 4 <= x[1] && x[2] <= x[1] {
                x[2] -= 2;
                x[1] -= 4;
            } else if 2 <= x[1] && 4 <= x[2] && x[1] < x[2] {
                x[1] -= 2;
                x[2] -= 4;
            } else {
                if x[1] == 2 && x[2] == 4 || x[1] == 4 && x[2] == 2 {
                    println!("{}", t + 1);
                } else {
                    println!("-1 {}", t);
                }
                break;
            }
            // println!("{t}: {:?}", x);
        }
    }
}
