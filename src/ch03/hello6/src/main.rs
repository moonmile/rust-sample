fn add_two( x: i32, y: i32 ) -> i32 {
    x + y 
}
fn add_one( x: i32 ) -> i32 {
    x + 1 
}

fn main() {
    let ans = add_two( 10, 20 ) ;
    println!("ans is {}", ans );
    let ans = add_one( 30 ) ;
    println!("ans is {}", ans );

    test() ;
    test2() ;
}

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

fn test() {
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



