// ジェネリクスの活用

fn _main() {
    // マクロでベクタを定義
    let v = vec![1,2,3,4,5] ;
	// Vet::new でベクタを初期化 
	let mut v: Vec<i32> = Vec::new() ;
	// 型を指定してVet::new でベクタを初期化 
    let mut v = Vec::<i32>::new() ;
	// 文字列(&str)のベクタ
    let mut v: Vec::<&str> = Vec::new();
	// 文字列(&String)のベクタ
    let mut v: Vec::<&String> = Vec::new();
	// 実数(f64)のベクタ
    let mut v: Vec::<f64> = Vec::new();
}

fn main() {
    // i32, char, &str の配列を出力する
    let v = [10,20,30,40,50] ;
    print!("v is ") ;
    for i in &v {
        print!("{} ", i) ;
    }
    println!("");

    let v = ['A','B','C','D','E'] ;
    print!("v is ") ;
    for i in &v {
        print!("{} ", i) ;
    }
    println!("");

    let v = ["one","two","three","four","five"] ;
    print!("v is ") ;
    for i in &v {
        print!("{} ", i) ;
    }
    println!("");

    // 実直に3種類の関数を作る
    let v = [10,20,30,40,50] ;
    print_i32( &v );
    let v = ['A','B','C','D','E'] ;
    print_char( &v );
    let v = ["one","two","three","four","five"] ;
    print_str( &v );

    // ジェネリックを使い1つにまとめる
    let v = [10,20,30,40,50] ;
    print( &v );
    let v = ['A','B','C','D','E'] ;
    print( &v );
    let v = ["one","two","three","four","five"] ;
    print( &v );
}

fn print_i32( a: &[i32] ) {
    print!("a is ") ;
    for i in a {
        print!("{} ", i) ;
    }
    println!("");
}
fn print_char( a: &[char] ) {
    print!("a is ") ;
    for i in a {
        print!("{} ", i) ;
    }
    println!("");
}
fn print_str( a: &[&str] ) {
    print!("a is ") ;
    for i in a {
        print!("{} ", i) ;
    }
    println!("");
}

fn print<T>( a: &[T] ) where T: std::fmt::Debug {
    print!("a is ") ;
    for i in a {
        print!("{:?} ", i) ;
    }
    println!("");
}

