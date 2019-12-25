// Result型の使い方
// https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/error-handling.html#result-%E5%9E%8B://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/error-handling.html#%E6%95%B4%E6%95%B0%E3%82%92%E3%83%91%E3%83%BC%E3%82%B9%E3%81%99%E3%82%8B
// https://doc.rust-jp.rs/book/second-edition/ch09-02-recoverable-errors-with-result.html
/*
enum Result<T,E> {
    Ok(T),          // 正常値
    Err(E),         // エラー値
}
*/


// 文字列を数値に変換して倍する
fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}
// Resultを返す修正版
fn double_number2(number_str: &str) -> Result<i32, std::num::ParseIntError> {
    // ここで Result を受ける
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}

/**
 * Result<T> にエイリアスすることで、戻り値の書き方を省略できる
 */
/*
type Result<T> = Result<T, std::num::ParseIntError> ;
// Result<T>を返す修正版
fn double_number3(number_str: &str) -> Result<i32> {
    // ここで Result を受ける
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}
*/

// コンピネーターを使うバージョン
fn double_number4(number_str: &str) -> Result<i32, std::num::ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n )
}

// エラー委譲「?」を使うバージョン
fn double_number5(number_str: &str) -> Result<i32, std::num::ParseIntError> {
    let n = number_str.parse::<i32>()? ;
    Ok(2 * n)
}

// 独自の Result<i32,&str> を返す
// Result<正常値, エラーメッセージ> で返すと良い
fn double_number3(number_str: &str) -> Result<i32, &str> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err("実行エラー：これは数値ではありません"),
    }
}



fn main() {
    let n: i32 = double_number("10");
    // assert でチェックする
    assert_eq!(n, 20);
    // 数値に変換できないので実行エラー
    // let n: i32 = double_number("xxx");

    // double_number2 は Result を返す
    match double_number2("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
    // double_number2 は Result を返す
    match double_number2("xxx") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }

    // sub 関数内で結果を表示
    println!("call sub");
    sub("10") ;
    sub("xxx") ;

    println!("call sub2");
    sub2("10") ;
    sub2("xxx") ; // ここのエラーは無視される

    // double_number3 は Result<i32,String> を返す
    match double_number3("10") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
    match double_number3("xxx") {
        Ok(n) => println!("Ok: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }

}


fn sub(s: &str) {
    match double_number2(s) {
        Ok(n) => println!("OK: {}", n ),
        Err(err) => println!("Error: {:?}", err),
    }
}

// double_number2 関数のエラーをそのまま返すことで、
// double_number2(s)? のように正常値だけ受け取る「?」が使える
fn sub2(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = double_number2(s)? ;
    println!("OK: {}", n ) ;
    Ok(n)
}



