use proconio::input;

fn main() {
    input! {
        y: u32
    };
    if y%4!=0{
        //Y が 4 の倍数でない年は365 日
        println!("365");
    }else if y%4==0 && y%100!=0 {
        // Y が 4 の倍数で、かつ 100 の倍数でない年は366 日
        println!("366");
    }else if y%100==0 && y%400!=0 {
        //Y が 100 の倍数で、かつ 400 の倍数でない年は365 日
        println!("365");
    }else if y%400==0 {
        //Y が 400 の倍数である年は 366 日
        println!("366");
    }
}