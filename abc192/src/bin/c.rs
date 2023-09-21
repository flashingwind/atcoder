use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
        k: usize,
    };
    // a_k=f(a_-1)
    let mut a = n.to_owned();
    let mut a_int = n.iter().collect::<String>().parse::<u32>().unwrap();
    for _i in 1..=k {
        a.sort();
        // println!("{_i}:");
        // println!("{_i}:{:?}", a);
        let a1 = a.iter().collect::<String>().parse::<u32>().unwrap();
        let a2 = a.iter().rev().collect::<String>().parse::<u32>().unwrap();
        a_int = a2 - a1;
        // println!("{a2} - {a1} = {}", a_int);
        a.clear();
        if a_int == 0 {
            a.push('0');
            break;
        }
        let mut tmp = a_int;
        while tmp != 0 {
            a.insert(0, std::char::from_digit(tmp % 10, 10).unwrap());
            tmp /= 10;
        }
    }
    println!("{}", a_int);
}
