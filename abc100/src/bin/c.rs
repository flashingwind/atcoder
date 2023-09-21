use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    let mut cnt = 0;
    let mut ok_cnt = 1;
    while 1 <= ok_cnt {
        ok_cnt = 0;
        for i in 0..n {
            if a[i] % 2 != 0 {
            } else if ok_cnt == 0 {
                a[i] /= 2;
                ok_cnt += 1;
            }
        }
        cnt += 1;
        // println!("{cnt}: {:?}", a);
    }
    println!("{}", cnt - 1);
}
