use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    };
    let mut for_cnt = 0;
    for ss in s.iter() {
        if ss == "For" {
            for_cnt += 1;
        }
    }
    if n <= for_cnt * 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
