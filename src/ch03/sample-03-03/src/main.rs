fn main() {
    let x = String::from("Hello") ;
    let len = string_length( x ) ;
    println!("len is {}", len );
    println!("x is {}", x ) ;
    /*
    let x = String::from("Hello") ;
    let y = x ;
    println!("x is {}", x );
    println!("y is {}", y );
    */
    /*
    let x = 100 ;
    let y = x ;
    println!("x is {}", x );
    println!("y is {}", y );
    */
}

fn string_length( s : String ) -> usize {
    let length = s.len();
    length 
}

