use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut stones: Chars,
    };
    let mut cnt = 0;
    let mut is_modifyed = true;
    while cnt <= stones.len() && is_modifyed {
        is_modifyed = false;
        for i in 0..stones.len() - 1 {
            if stones[i] == 'B' && stones[i + 1] == 'W' {
                stones[i] = 'W';
                stones[i + 1] = 'B';
                cnt += 1;
                is_modifyed = true;
                println!("{:?}", stones);
                WB
            }
        }
    }
    println!("{}", cnt);
}
