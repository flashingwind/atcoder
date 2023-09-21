use proconio::input;

fn main() {
    input! {
        mut t_hp: u32,
        t_f: u32,
        mut a_hp: u32,
        a_f: u32,
    };
    for _ in 0..100 {
        if a_hp <= t_f {
            println!("Yes");
            break;
        }
        a_hp -= t_f;
        if t_hp <= a_f {
            println!("No");
            break;
        }
        t_hp -= a_f;
    }
}
