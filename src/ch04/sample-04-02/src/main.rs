fn main() {
    let s = "hello rust world." ;
    // 文字列を表示
    println!("s is {}", s ); 
    // 部分文字列を取得
    let hello = &s[0..5] ;
    let world = &s[11..] ;
    println!("hello is {}", hello ); 
    println!("world is {}", world ); 
    // 長さを取得する
    let len = s.len() ;
    println!("s.len is {}", len ); 
    // 空の文字列を作る
    let mut s = String::new() ;
    // 文字列を追加する
    s.push_str("hello ") ;
    s.push_str("rust ") ;
    s.push_str("world.") ;
    println!("s is {}", s ); 
    // format!マクロで連結する
    let hello = "HELLO" ;
    let rust = "RUST" ;
    let world = "WORLD." ;
    let s = format!("{} {} {}", hello, rust, world) ;
    println!("s is {}", s ); 
    // &String型の文字列を作る
    let s = "hello rust world.".to_string() ;
    println!("s is {}", s ); 
    let s = String::from("hello rust world.");
    println!("s is {}", s ); 

    let s = "こんにちは rust コードの世界" ;
    // 文字列を表示
    println!("s is {}", s ); 
    // 部分文字列を取得
    /* 5バイト目が文字の境界ではないので実行エラーになる
    let hello = &s[0..5] ;
    let world = &s[11..] ;
    println!("こんにちは is {}", hello ); 
    println!("コードの世界 is {}", world ); 
    */
    // 長さを取得する
    let len = s.len() ;
    println!("s.len is {}", len ); 
    // 空の文字列を作る
    let mut s = String::new() ;
    // 文字列を追加する
    s.push_str("こんにちは ") ;
    s.push_str("rust ") ;
    s.push_str("コードの世界") ;
    println!("s is {}", s ); 
    // format!マクロで連結する
    let hello = "こんにちは" ;
    let rust = "RUST" ;
    let world = "コードの世界" ;
    let s = format!("{} {} {}", hello, rust, world) ;
    println!("s is {}", s ); 
    // &String型の文字列を作る
    let s = "こんにちは rust コードの世界".to_string() ;
    println!("s is {}", s ); 
    let s = String::from("こんにちは rust コードの世界");
    println!("s is {}", s ); 


    let s = "hello rust world." ;
    for c in s.chars() {
        print!("{} ", c );
    }
    println!(".");
    // let mut v = s.chars() ;
    // println!("v[0] is {}", v.next().unwrap());

    let s = "こんにちは rust コードの世界" ;
    for c in s.chars() {
        print!("{} ", c );
    }
    println!("");
    // let mut v = s.chars() ;
    // println!("v[0] is {}", v.next().unwrap());


    let s = "This is ねこ🐱neko 文字列🐱" ;
    // ベクタに直す
    let mut v : Vec<char> = Vec::new() ;
    for c in s.chars() { 
        v.push( c );
    }
    // 8文字目から14文字目まで取得
    let v = &v[8..15] ;
    // 再び &String に直す
    let mut s = String::new() ;
    for c in v {
        s.push( *c )
    } 
    println!("s is {}", s );
}

