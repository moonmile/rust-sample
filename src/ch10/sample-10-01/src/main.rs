fn main() {
    let a = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    // print_a 関数に参照で借用(borrow)する
    print_a( &a );
    // 所有権が戻っているので、代入はできる
    let x = a ;
    print_a( &x );
    // 所有権が移動(move)しているので、代入はできない
    // let y = a ;
    // print_a( &y );

    // もう一度 a を作る
    let a = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    // x に借用するだけの場合は、所有権がaに残る
    let x = &a ;
    print_a( &x );
    // 所有権が移動(move)してないので、代入ができる
    let y = a ;
    print_a( &y );

    // 可変な a を作る
    let mut a = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    println!("a.age is {}", a.age );
    // add_age 関数に借用(borrow)させる
    add_age( &mut a );
    // 所有権は残っているので参照は可能
    println!("a.age is {}", a.age ) ;
    // x を参照させる
    let x = &a ;
    println!("x.age is {}", x.age );
    // x は所有権がないので add_age には渡せない
    // add_age( &mut x );
    // 所有権が y に移る
    let y = a ;
    println!("y.age is {}", y.age );
    // 既に a に所有権がないので、z で参照できない
    // let z = &a ;
    // println!("z.age is {}", z.age );

    // 数値の場合
    let a = 100 ;
    println!("a is {}", a );
    // 数値の場合は x に値が copy されている
    let x = a ;
    println!("x is {}", x );
    // 所有権は a のままなので、copy できる
    let y = a ;
    println!("y is {}", y );

    // タプルの場合
    let a = (100,"masuda") ;
    println!("a is {}", a.1 );
    // タプルの場合は x に値が copy されている
    let x = a ;
    println!("x is {}", x.1 );
    // 所有権は a のままなので、copy できる
    let y = a ;
    println!("y is {}", y.1 );

    // ベクタの場合
    let a = vec!["one","two","three"];
    println!("a[0] is {}", a[0] );
    // ベクタの場合は x に所有権が移動（move）される
    let x = a ;
    println!("x[0] is {}", x[0] );
    // 所有権がないので、y に代入できない
    // let y = a ;
    // println!("y[0] is {}", y[0] );
}

struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn print_a( a: &Person ) {
    println!("a.name is {}", a.name ) ;
}

fn add_age( a: &mut Person ) {
    a.age += 1 ;
    println!("a.age is {}", a.age ) ;
}

