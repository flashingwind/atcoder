use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    };
    //let mut diff=vec![(1,2),(2,1),(,),(,),(,),(,),(,),(,),];
    let mut is_ok = false;
    for x in x1 - 2..=x1 + 2 {
        for y in y1 - 2..=y1 + 2 {
            if (x - x1).pow(2) + (y - y1).pow(2) == 5 && (x - x2).pow(2) + (y - y2).pow(2) == 5 {
                is_ok = true;
            }
        }
    }
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
