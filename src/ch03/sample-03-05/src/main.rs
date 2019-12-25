
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


    let ans = add_two( 10, 20 ) ;
    println!("ans is {}", ans );
    let ans = add_one( 30 ) ;
    println!("ans is {}", ans );

    test1() ;
    test2() ;

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

/// 構造体の例
struct Sample {
    x: i32,
}
impl Sample {
    fn new( x: i32 ) -> Sample {
        Sample { x: x }
    }
    fn inc(&self) -> i32 {
        self.x + 1 
    }
    fn add(&self, x: i32) -> i32 {
        self.x + x  
    }
}

fn add_two( x: i32, y: i32 ) -> i32 {
    x + y 
}
fn add_one( x: i32 ) -> i32 {
    x + 1 
}

fn test1() {
    let a = Sample::new(10) ;
    let ans = a.inc();
    println!("ans is {}", ans );
    let ans = a.add(20);
    println!("ans is {}", ans );
}

/// クロージャの例
fn test2() {
    let num = 10 ;
    let add_one = |x| { num + x };
    let add_two = |x ,y| { x + y };

    let ans = add_one(1);
    println!("ans is {}", ans );
    let ans = add_two(10,20) ;
    println!("ans is {}", ans );

    fn add( x: i32, y: i32 ) -> i32 {
        // 関数外の変数は参照できない
        // num + x + y 
        x + y 
    }
    let ans = add( 10, 20 );
    println!("ans is {}", ans );
}



