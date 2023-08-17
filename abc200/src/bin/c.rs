use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize;n],
    };
    let mut cnt = vec![0usize; 200];
    for a in arr.iter() {
        cnt[*a % 200] += 1;
    }
    let mut sum = 0;
    for cnt_i in cnt.iter() {
        if 1 < *cnt_i {
            sum += (*cnt_i * (*cnt_i - 1)) / 2;
            // print!("{cnt_i}:{},", (*cnt_i * (*cnt_i + 1)) / 2);
        }
    }
    // println!();
    println!("{}", sum);
}
