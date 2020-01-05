fn main() {
    go1();
}

// main 関数で panic する
fn go1() {
    panic!("このプログラムは動きません");
}

// 引数の有無で panic する
fn go2() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        panic!("パラメータは必須です");
    } else {
        for (i,s) in args.iter().enumerate() {
            println!("{}: {}", i, s );
        }
    }
}

use std::path::Path ;
// ファイルの有無で panic する
fn go3() {
    let path = "unknown.txt";
    if Path::new( path ).exists() {
        println!("ファイルが存在する");
    } else {
        panic!("ファイルが存在しない");
    }
}
