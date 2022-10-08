use proconio::input;
use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let mut template: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
            .iter()
            .map(|s| s.chars().rev().collect())
            .collect();
    

    // s.iter().rev();
    
    for temp in &template {
        println!("{:?}", temp);
        println!("{:?}", s[0..4]);
        // if s[0..4] == temp{
        //     s = s[]
        // }
    }


    // for i in s.iter().rev() {
    //     println!("{}", i);
    // }

}