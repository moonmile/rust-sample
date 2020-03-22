fn typename<T>(_: T) -> &'static str {
    std::any::type_name::<T>()

}


fn main() {
    // 型をチェックする
    let s = "masuda";
    println!("s type is {}", typename(s));
    let a = 100 ;
    println!("a type is {}", typename(a));
    let a = ("masuda", 50 );
    println!("a type is {}", typename(a));
    let a = [1,2,3,4];
    println!("a type is {}", typename(a));
    let v = vec![1,2,3,4];
    println!("v type is {}", typename(v));
    let a = Person { name: "masuda", age: 50 };
    println!("a type is {}", typename(a));
    let func = |name, age| {
        println!("name: {}, age: {}", name, age );
    };
    func("masuda", 50);
    println!("func type is {}", typename(func));

}

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

