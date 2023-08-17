use proconio::input;

fn main() {
    input! {
        mut card: [u32;5],
    };
    card.sort();
    // println!("{:?}", card);

    let mut is_fullhouse = true;
    let mut is_eq = true;
    for c in card.iter().take(3) {
        if card[0] != *c {
            is_eq = false;
        }
    }
    is_fullhouse = is_fullhouse && is_eq;
    for c in card.iter().rev().take(2) {
        if *card.last().unwrap() != *c {
            is_eq = false;
        }
    }
    is_fullhouse = is_fullhouse && is_eq;
    if is_fullhouse {
        println!("Yes");
        return;
    }
    // println!("not fullhouse");
    is_fullhouse = true;
    is_eq = true;
    for c in card.iter().rev().take(3) {
        if *card.last().unwrap() != *c {
            is_eq = false;
        }
    }
    is_fullhouse = is_fullhouse && is_eq;
    for c in card.iter().take(2) {
        if card[0] != *c {
            is_eq = false;
        }
    }
    is_fullhouse = is_fullhouse && is_eq;
    if is_fullhouse {
        println!("Yes");
        return;
    }
    println!("No");
}
