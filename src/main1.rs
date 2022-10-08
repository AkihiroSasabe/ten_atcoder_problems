use io_atcoder::io_atcoder;
use proconio::input;

fn main() {
    // let a: u32 = io_atcoder::read();
    // let b: u32 = io_atcoder::read();
    input! {
        a: u32,
        b: i32,
    }
    let mut ans: String;
    if (a*b) % 2 == 0{
        ans = "Even".to_string();
    } else{
        ans = "Odd".to_string();
    }

    println!("{ans}")
}