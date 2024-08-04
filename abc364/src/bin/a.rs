use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    };
    let mut cnt_sweet=0;
    for i in 0..n {
        if s[i]=="sweet"{
            cnt_sweet+=1;
        }else if  s[i]=="salty"{
            cnt_sweet=0;
        }
        // println!("{i} cnt_sweet={cnt_sweet}");
        if i==n-1{
            println!("Yes");
            return;
        } else if cnt_sweet==2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}