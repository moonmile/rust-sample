fn main() {
    // 関数の色々
    no_param() ;
    one_param( 10 );
    two_param( 10, 20 );

    let ret = two_param_and_return( 10, 20 );
    println!("ret is {}", ret );

    str_param( "rust" ) ;

    let mut s = String::new();
    str_param_complex( &mut s );
    println!("s is {}", s );


    let ret = str_param_and_return( "rust" ) ;
    println!("ret is {}", ret );

    let v = vec![1,2,3,4,5] ;
    let sum = vec_param( &v );
    println!("sum is {}", sum );

    let v = vec_return( 10 );
    for i in v {
        print!("{} ", i );
    }
    println!("");

    let mut v = vec![1,2,3,4,5] ;
    vec_change( &mut v );
    for i in v {
        print!("{} ", i );
    }
    println!("");
}


/// 引数も戻り値もない関数
fn no_param() {
    println!("called no_param");
}

/// 引数が1つだけの関数
fn one_param( x: i32 ) {
    println!("called one_param, x is {}", x );
}

/// 引数が2つの関数
fn two_param( x: i32, y: i32 ) {
    println!("called two_param, {} and {}", x, y );
}

/// 戻り値のある関数
fn two_param_and_return( x: i32, y: i32 ) -> i32 {
    println!("called two_param_and_return, {} and {}", x, y );
    x + y 
}

/// 文字列を受け取る関数
fn str_param( s: &str ) {
    println!("called str_param, s is {}", s );
}

/// 文字列を受け取る関数
fn str_param_complex( s: &mut String ) {
    *s = String::from("hello") ;
}


/// 文字列を返す関数
fn str_param_and_return( s: &str ) -> String {
    println!("called str_param_and_return, s is {}", s );
    let ret = format!("hello {} world.", s ) ;
    ret 
}

// ベクタを受け取る関数
fn vec_param( v: &Vec<i32> ) -> i32 {
    println!("called vec_param");
    let mut sum = 0;
    for i in v {
        sum += i ;
    }
    sum 
}

// ベクタを返す関数
fn vec_return( max: i32 ) -> Vec<i32> {
    println!("called vec_return");
    let mut v = Vec::new() ;
    for i in 0..max {
        v.push( i );
    }
    v 
}

// ベクタの中味を変更する関数
fn vec_change( v: &mut Vec<i32> )  {
    println!("called vec_change");
    for i in v {
        *i = *i * 10 ;
    }
}
