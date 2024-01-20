use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(u32,u32);n],
    };
    let mut sumt = 0;
    let mut suma = 0;
    for &(tak, aok) in a.iter() {
        sumt += tak;
        suma += aok;
    }
    if suma < sumt {
        println!("Takahashi");
    } else if suma > sumt {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
