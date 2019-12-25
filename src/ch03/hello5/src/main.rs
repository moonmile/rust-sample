
// グローバル変数は使えないが、定数ならば大丈夫
// let x : i32  = 999 ;
const MAX : i32 = 999 ;

fn main() {
    println!("MAX is {}", MAX );

    let x = 100 ;
    println!("x is {}", x );
    {
        let x = 200 ;
        println!("x is {}", x );
    }
    println!("x is {}", x );

    let ans = test(50) ;
    println!("ans is {}", ans );
    let ans = test_s(50) ;
    println!("ans is {}", ans );
    let ans = test_ss(50) ;
    println!("ans is {}", ans );

}

fn test(x: i32) -> i32 {
    let mut ans = x ;
    if x < 0 {
        ans = 0 ;
    } 
    if x > 100 {
        ans = 100 ;
    }
    ans 
}

fn test_s( x: i32 ) -> i32 {
    if x < 0 { 
        0 
    } else if x > 100 { 
        100 
    } else { 
        x 
    }
}

fn test_ss( x: i32 ) -> i32 {
    let ans = if x < 0 { 
        0 
    } else if x > 100 { 
        100 
    } else { 
        x 
    } ;
    ans 
}


/* これはコンパイルエラーになる
fn test_err(x: i32) -> i32 {
    if x < 0 {
        let ans = 0 ;
    } 
    if x > 100 {
        let ans = 100 ;
    }
    ans 
}
*/
