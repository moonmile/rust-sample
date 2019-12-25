use std::mem;

fn main() {
    // 文字列と数値を宣言する
    let name = "masuda tomoaki"  ;
    let age  = 30 ;

    println!("name is {}, age: {} ", name, age );

    // 明示的に型を宣言する
    let name : &str = "masuda tomoaki" ;
    let age : i32 = age ;

    println!("name is {}, age: {} ", name, age );

    // 関数の戻り値の推論される
    let ans = add( 10, 20 ) ;
    println!( "ans is {}", ans );

    // 数値のビット数を処理系のサイズに合わせる
    let name = "masuda tomoaki"  ;
    let age : isize = 30 ;
    println!("name is {}, age: {} ", name, age );

    // 浮動小数点を扱う
    let x = 100.234 ;
    println!("x is {}", x );
    let x : f64 = 100.234 ;
    println!("x is {}", x );

    // 論理型を使う
    let f = true ;
    let f : bool = true ;
    println!("f is {}", f ) ;
    println!("bool sizeof : {}", mem::size_of::<bool>() );
    // u8にキャストして 1/0 を確認する
    let b : u8 = f as u8 ;
    println!("b is {}, f is {}", b, f ) ;

    // 文字を扱う
    let c = 'A' ;
    let c = 'あ' ;
    let dog = '🐶' ;
    let cat : char = '🐱' ;
    println!("{} and {}", dog, cat ) ;
    // 文字列で絵文字を使う
    let dog_and_cat = "🐶と🐱" ;
    println!("{}", dog_and_cat ) ;

    // 文字列を表示する
    let s = "Hello Rust world." ;
    println!("{}", s ) ;
    let dog = "DOG" ;
    let cat = "CAT" ;
    println!("{} and {}", dog, cat );
    // String型を使う
    let s = String::from("Hello Rust world.");
    println!("{}", s ) ;
    // 文字列を更新する
    let s1 = String::from("Hello") ;
    let s2 = String::from("Rust") ;
    let s3 = String::from("world.") ;
    let s = s1 + " " + &s2 + " " + &s3 ;
    println!("{}", s ) ;

    let s1 = String::from("Hello") ;
    let s2 = String::from("Rust") ;
    let s3 = String::from("world.") ;
    let s = format!("{} {} {}", s1, s2, s3) ;
    println!("{}", s ) ;

    let s1 = "Hello" ;
    let s2 = "Rust" ;
    let s3 = "world." ;
    let s = format!("{} {} {}", s1, s2, s3) ;
    println!("{}", s ) ;
}


fn add( x : i32 , y : i32 ) ->  i32 {
    // println!("call add") ;
    x + y 
}