use proconio::{input, marker::Chars};
use rustc_hash::FxHashSet;

fn main() {
    let n = 9;
    input! {
        map: [Chars;n],
    };
    let mut is_checked = FxHashSet::default();
    let mut cnt = 0;
    for y_lt in 0..n {
        for x_lt in 0..n {
            for y_rt in y_lt..n {
                for x_rt in x_lt + 1..n {
                    let dx = x_rt - x_lt;
                    let dy = y_rt - y_lt;
                    let x_rb = x_lt + dx;
                    let y_rb = y_lt + dx;
                    if x_lt < dy {
                        continue;
                    }
                    let x_lb = x_lt - dy;
                    let y_lb = y_lt + dx;

                    if n <= y_lb.max(y_rb) || n <= x_lb.max(x_rb) {
                        continue;
                    }
                    if is_checked.contains(&(y_rb, x_rb))
                        && is_checked.contains(&(y_lb, x_lb))
                        && is_checked.contains(&(y_lt, x_lt))
                        && is_checked.contains(&(y_rt, x_rt))
                    {
                        println!(
                            "  chd:{},{}--{},{}--{},{}--{},{}",
                            y_lt, x_lt, y_rt, x_rt, y_lb, x_lb, y_rb, x_rb
                        );
                        continue;
                    }
                    is_checked.insert((y_rb, x_rb));
                    is_checked.insert((y_lb, x_lb));
                    is_checked.insert((y_lt, x_lt));
                    is_checked.insert((y_rt, x_rt));
                    println!("--{},{}--{},{}--", y_lt, x_lt, y_rt, x_rt);
                    println!("  {},{}--{},{}", y_lb, x_lb, y_rb, x_rb);
                    // println!("if cnt");
                    if map[y_lt][x_lt] == '#'
                        && map[y_rt][x_rt] == '#'
                        && map[x_lb][y_lb] == '#'
                        && map[y_rb][x_rb] == '#'
                    {
                        println!("cnt++");
                        cnt += 1;
                    }
                }
            }
        }
    }
    println!("{}", cnt);
}
