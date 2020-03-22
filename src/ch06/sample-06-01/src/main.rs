fn main() {

    let a = 10 ;
    let b = 20 ;
    if a < b {
        println!("a < b is ok.");
    }
    // 括弧を付けてると warning が出る
    if ( a < b ) {
        println!("a < b is ok.");
    }

    // if-else を使う
    if  a < b  {
        println!("a < b is ok.");
    } else {
        println!("a < b is ng.");
    }


    // 複数のifを使う
    if  a == b  {
        println!("a == b is ok.");
    } else if a < b {
        println!("a < b is ok.");
    } else {
        println!("a > b is ok.");
    }


    // 関数の戻り値で直接比較する
    if test( a, b )  {
        println!("test is ok.");
    }

    // 論理積
    if a == 10 && b == 20 {
        println!("AND is ok.");
    }
    // 論理和
    if a == 0 || b == 20 {
        println!("OR is ok.");
    }

    // if で値を返す
    let x = if a < b { 1 } else { 0 } ;
    println!("x is {}", x );
    // if で値を返す（コンパイルエラー）
    // let x = if a < b { 1 } else { "hello" } ;
    // println!("x is {}", x );

}

fn test( x: i32, y: i32 ) -> bool {
    x < y
}

