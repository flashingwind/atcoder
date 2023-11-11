use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u32;n],
    };
    let mut cnt = 0;
    for (i, &m) in d.iter().enumerate() {
        let mut zoro = vec![];
        if i + 1 < 10 {
            zoro.push(format!("{}{}", i + 1, i + 1));
            zoro.push(format!("{}{}{}", i + 1, i + 1, i + 1));
        } else if (i + 1) % 11 == 0 {
            zoro.push(format!("{}{}", i + 1, i + 1));
            zoro.push(format!("{}{}", i + 1, (i + 1) % 10));
        };
        // println!("{:?}:", zoro);
        for dd in 1..=m {
            if zoro.contains(&format!("{}{}", i + 1, dd)) {
                // println!("=={}{}", i + 1, dd);
                cnt += 1;
            }
        }
    }
    println!("{cnt}");
}
