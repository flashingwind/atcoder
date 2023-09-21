use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [u32;n],
        t: [u32;m],
    };
    let mut cnt = 0;
    for ss in s.iter() {
        //println!("ss={ss}");
        let s_lower = format!("{:06}", *ss).get(3..6).unwrap().to_string();
        // println!("s_lower={s_lower}");
        for tt in t.iter() {
            let t_lower = format!("{:03}", *tt);
            // println!("t_lower={t_lower}");
            if t_lower == s_lower {
                cnt += 1;
                break;
            }
        }
    }
    println!("{}", cnt);
}
