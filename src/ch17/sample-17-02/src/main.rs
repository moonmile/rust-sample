fn main() {
    // クロージャの定義
    let print_name = |name, age| {
        println!("name: {}, age: {}", name, age );
    };
    // クロージャの呼び出しは関数と同じ
    println!("call closure");
    print_name("masuda", 50); 

    println!("use for statement");
    let a = [("masuda", 50), ("kato", 40), ("yamada", 30)];
    for it in &a {
        print_name( it.0, it.1 );
    }

    // map にクロージャを渡す
    println!("use map");
    let b = a.iter().map(|(name,age)| {
        format!("name: {}, age: {}", name, age )
    });
    for it in b {
        println!("{}", it);
    }
}
