/*
enum Result<T,E> {
    Ok(T),          // 正常値
    Err(E),         // エラー値
}
*/

use std::num::ParseIntError ;

// 文字列を数値に変換して半分にする
fn half_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap() / 2 
}
// Resultを返す修正版
fn half_number2(s: &str) -> Result<i32, ParseIntError> {
    // ここで Result を受ける
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}

/**
 * Result<T> にエイリアスすることで、戻り値の書き方を省略できる
 */
type MyResult<T> = Result<T, ParseIntError> ;
// Result<T>を返す修正版
fn half_number3(s: &str) -> MyResult<i32> {
    // ここで Result を受ける
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}

// コンピネーターを使うバージョン
fn half_number4(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n / 2 )
}

// エラー委譲「?」を使うバージョン
fn half_number5(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()? ;
    Ok(n / 2)
}

// 独自の Result<i32,&str> を返す
// Result<正常値, エラーメッセージ> で返すと良い
fn half_number6(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err("実行エラー：これは数値ではありません"),
    }
}

fn main() {
    println!("call str.parse");
    let r = "100".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n ),
        Err(e) => println!("error: {:?}", e ),
    }
    let r = "xxx".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n ),
        Err(e) => println!("error: {:?}", e ),
    }

    let n = "100".parse::<i32>().unwrap();
    println!("n is {}", n );
    // 実行エラーとなる
    // let n = "xxx".parse::<i32>().unwrap();
    // println!("n is {}", n );



    println!("call half_number");
    let n: i32 = half_number("100");
    println!("n is {}", n );
    // 数値に変換できないので実行エラー
    // let n: i32 = half_number("xxx");

    println!("call half_number2");
    // half_number2 は Result を返す
    match half_number2("100") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number2("xxx") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }

    // Resultをエイリアスする
    println!("call half_number3");
    match half_number3("100") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number3("xxx") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }

    // コンピネーターを使う
    println!("call half_number4");
    match half_number4("100") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number4("xxx") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }

    // エラー委譲を使う
    println!("call half_number5");
    match half_number5("100") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number5("xxx") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }

    // 独自のResultを作る
    println!("call half_number6");
    match half_number6("100") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number6("xxx") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }

    // except を使う
    let n = "100".parse::<i32>()
        .expect("これは数値ではありません");
    println!("n is {}", n );
    let n = "xxx".parse::<i32>()
        .expect("これは数値ではありません");
    println!("n is {}", n );
}


fn _main() -> Result<(), Box<dyn std::error::Error>> {
    // ファイルを一気に読み込む
    let path = "unknown.txt" ;
    let data = std::fs::read_to_string( path )? ;
    println!("data is {}", data);
    Ok(())
}

