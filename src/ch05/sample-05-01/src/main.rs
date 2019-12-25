fn main() {
    // セミコロンが文の区切りになる
    let a = 10 ;
    let b = 20 ;
    println!("a is {}, b is {}", a, b );
    // これも3つの文
    let a = 10 ; let b = 20 ; println!("a is {}, b is {}", a, b );
    // 文の中に改行が入っても良い
    let a 
      = 10 ;
    let b 
      = 20 ;
    println!("a is {}, b is {}", 
      a, b );
    // 式は値を返す
    let a = 10 + 20 ;
    println!("a is {}", a );
    // ブロックの場合も、値を返すので式にできる
    let a = { 10 + 20 } ;
    println!("a is {}", a );
    // ブロック内にセミコロンを入れると文になるので、コンパイルエラー
    // a は値を持たない
    let a = { 10 + 20 ; } ;
    println!("a is {}", a );
    // 関数は式になる
    let a = add( 10, 20 ) ;
    println!("a is {}", a );
    // if 文は bool 値を持つ式を条件に付けられる
    let a = 10 ;
    if a > 0 {
        println!("a is {}", a );
    }
    // 関数もbool値を返せば if文に渡せる
    if plus( a ) {
        println!("plus(a) is {}", a );
    }
}

fn add( x: i32, y: i32 ) -> i32 {
    x + y 
}
fn plus( x: i32 ) -> bool {
    x > 0 
}

