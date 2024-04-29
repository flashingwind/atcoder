use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    let mut mov = vec![vec![]; n + 1];
    for i in 1..=n {
        mov[a[i - 1]].push(i.to_string());
        mov[i].push((a[i - 1]).to_string());
    }
    {
        let mut i = 0usize;
        while i < mov.len() {
            if mov[i].len() != 2 {
                println!("ERR {i}: {}", mov[i].join(" "));
                i += 1;
            } else if mov[i][0] == mov[i][1] {
                println!("EQ {i}: {}", mov[i].join(" "));
                // mov.remove(i);
                i += 1;
            } else {
                i += 1;
            }
        }
    }
    println!("{}", mov.len() - 1);
    for pair in mov.iter().skip(1) {
        println!("{}", pair.join(" "));
    }
}
