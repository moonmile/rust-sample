fn main() {
    // 変数のポインタを取得する
    let a: i32 = 10 ;
    let a_ptr: *const i32 = &a ;
    println!("a is {:?}", a );
    println!("a_ptr is {:?}", a_ptr );
    // 文字列のポインタを取得する
    let a = "rust" ;
    println!("a is {:?}", a );
    println!("a_ptr is {:?}", a.as_ptr() );
}

fn main2() {
    let a = Person{name: "masuda", age: 50 };
    println!("a is {:?}", a );
    // 変数b に move する
    let b = a ;
    println!("b is {:?}", b );
    // 既に move しているので使えない
    // println!("a is {:?}", a );
}

fn main3(){
    let a = Person{name: "masuda", age: 50 };
    let a_ptr: *const Person = &a;
    println!("a is {:?}", a );
    println!("a_ptr is {:?}", a_ptr );
    println!("a.name.ptr is {:?}", a.name.as_ptr());
    // 変数b に move する
    let b = a ;
    let b_ptr: *const Person = &b;
    println!("b is {:?}", b );
    println!("b_ptr is {:?}", b_ptr );
    println!("b.name.ptr is {:?}", b.name.as_ptr());
    // 既に move しているので使えない
    // println!("a is {:?}", a );
}

fn main4() {
    let a = Person{name: "masuda", age: 50 };
    let a_ptr: *const Person = &a;
    println!("a is {:?}", a );
    println!("a_ptr is {:?}", a_ptr );
    println!("a.name.ptr is {:?}", a.name.as_ptr());
    // 変数b に borrow する
    let b = &a ;
    let b_ptr: *const Person = b;
    println!("b is {:?}", b );
    println!("b_ptr is {:?}", b_ptr );
    println!("b.name.ptr is {:?}", b.name.as_ptr());
    // 借用なので変数aが利用できる
    println!("a is {:?}", a );
}

fn main5() {
    let a: i32 = 100;
    println!("a is {:?}", a );
    // 変数b に copy する
    let b = a ;
    let b_ptr: *const i32 = &b;
    println!("b is {:?}", b );
    // copy なので変数aが利用できる
    println!("a is {:?}", a );
}

fn main6() {
    let a: i32 = 100;
    let a_ptr: *const i32 = &a;
    println!("a is {:?}", a );
    println!("a_ptr is {:?}", a_ptr );
    // 変数b に copy する
    let b = a ;
    let b_ptr: *const i32 = &b;
    println!("b is {:?}", b );
    println!("b_ptr is {:?}", b_ptr );
    // copy なので変数aが利用できる
    println!("a is {:?}", a );
}
#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}
