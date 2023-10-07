use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.sort();
    let mut i = 0;
    let mut books = VecDeque::from(a);
    let mut money = 0;
    {
        let len_old = books.len();
        books = books.iter().unique().map(|v| *v).collect();
        money += len_old - books.len();
    }
    {
        let mut cnt = 0;
        for i in 0..n {
            if n < a[i] {
                money += 1;
            }
        }
    }
    loop {
        if let Some(b) = books.pop_front() {
            if b == i + 1 {
                i += 1;
            } else {
                books.push_front(b);
                break;
            }
        } else {
            println!("{}", i);
            return;
        }
    }
    // println!("*");
    // println!("**");
    for _ in 0..n {
        // sell & buy
        loop {
            if let Some(l1) = books.pop_back() {
                if let Some(l2) = books.pop_back() {
                    if let Some(&f) = books.front() {
                        // println!("{},{}->{}: {:?}", l1, l2, i + 1, books);
                        i += 1;
                    } else {
                        books.push_back(l2);
                        books.push_back(l1);
                        break;
                    }
                } else {
                    books.push_back(l1);
                    break;
                }
            } else {
                break;
            }
            if 2 < money {
                if let Some(&f) = books.front() {
                    books.pop_front();
                    // println!("{},{}->{}: {:?}", l1, l2, i + 1, books);
                    i += 1;
                } else {
                    break;
                }
            }
        }
        // read
        if let Some(b) = books.pop_front() {
            if b == i + 1 {
                i += 1;
            } else {
                println!("{}", i);
                return;
            }
        } else {
            println!("{}", i);
            return;
        }
    }
}
