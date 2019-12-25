fn main() {
    // スライスのパターン
    let s = "hello rust world." ;
    // 先頭の文字
    let a = &s[0..1] ; 
    println!("a is {}", a );
    // 先頭の5文字
    let a = &s[0..5] ;
    println!("a is {}", a );
    // 0を省略できる
    let a = &s[..5] ;
    println!("a is {}", a );
    // 途中の文字
    let a = &s[6..10] ;
    println!("a is {}", a );
    // 6文字目から4文字分取り出す
    let a = &s[6..(6+4)] ;
    println!("a is {}", a );
    // 11文字目から最後まで
    let a = &s[11..] ;
    println!("a is {}", a );
    // 明示的に最後を示す
    let n = s.len();
    let a = &s[11..n] ;
    println!("a is {}", a );
    // 範囲を超えるとエラー
    // let a = &s[11..20] ;
    // println!("a is {}", a );
    // .. だけの場合は全体を返す
    let a = &s[..] ;
    println!("a is {}", a );
    // 明示的に全体を取得する
    let n = s.len();
    let a = &s[0..n] ;
    println!("a is {}", a );
}
