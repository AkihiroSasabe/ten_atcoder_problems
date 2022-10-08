// use io_atcoder::io_atcoder;
use proconio::input;
use proconio::marker::{Bytes, Chars};

fn main(){
    input! {
        // 解法1(添字アクセス不可)
        s: String

        // 解法2(添字アクセス可) Vec<char>型である
        // s: Chars,


    }

    // 解法1
    // println!("{}", s.chars().filter(|&c| c == '1').count());
    // 参考 charsメソッドはイテレータ
    // for i in s.chars() {
    //     println!("{}", i);
    // }


    // 解法2
    // 参考: 解法1のString型をVec<char>に変換してもよい
    // let s: Vec<char> = s.chars().collect(); // String型をVec<char>に変換
    let mut cnt = 0;
    if s[0] == '1' {cnt += 1}
    if s[1] == '1' {cnt += 1}
    if s[2] == '1' {cnt += 1}

    println!("{}", cnt)
}