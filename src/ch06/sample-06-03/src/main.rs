#[allow(dead_code)]     // 使われないコードを許容する
#[derive(Debug)]        // デバッグ表示を有効にする
enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRANCH,
} 

fn main() {
    let lang = LANG::JAPANESE ;
    // println!("lang is {}", lang as i32 );
    println!("lang is {:?}", lang );

    // 全ての候補を列挙する
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH  => "英語",
        LANG::CHINESE  => "中国語",
        LANG::FRANCH   => "フランス語",
    } ;
    println!("lang is {}", m );

    let lang = LANG::ENGLISH ;
    // 候補以外を _ で指定する
    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "日本語以外"
    } ;
    println!("lang is {}", m );

    /*
    let lang = LANG::CHINESE ;
    // 全ての候補を列記しないとコンパイルエラー
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH  => "英語",
        LANG::CHINESE  => "中国語",
        // LANG::FRANCH   => "フランス語",
    } ;
    println!("lang is {}", m );
    */

    // オプション型を利用する
    let x = Some(10) ;
    let v = match x {
        Some(i) => i, 
        None => -1,
    } ;
    println!("v is {}", v );

    let x = None ;
    let v = match x {
        Some(i) => i, 
        None => -1,
    } ;
    println!("v is {}", v );

    // match を if let に書き換える
    let x = Some(10) ;
    match x {
        Some(i) => println!("i is {}", i ),
        _ => (),
    } ;
    
    if let Some(i) = x {
        println!("i is {}", i ) ;
    }

    // 数値を指定したパターンマッチ
    let x = 3 ;
    let m = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    } ;
    println!("m is {}", m );

    // 数値の範囲を指定する
    let x = 5 ;
    let m = match x {
        0..=5 => "5以下",
        6..=10 => "6以上10以下",
        _ => "10より大きい",
    } ;
    println!("m is {}", m );


    // 文字を指定したパターンマッチ
    let x = 'C' ;
    let m = match x {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => -1,
    } ;
    println!("m is {}", m );

    // 文字列を指定したパターンマッチ
    let x = "three" ;
    let m = match x {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        _ => -1,
    } ;
    println!("m is {}", m );
}
