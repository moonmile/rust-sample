#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main1() {
    // 変数xを用意する
    let x: &Person;
    // ブロックを開始する
    {
        // 変数aにメモリ領域を割り当てる
        let a = Person {
            name: String::from("masuda"),
            age: 50,
        };
        // 変数xに参照させる
        x = &a ;
        // ブロックを抜ける
    }
    // これは参照できない
    // println!("x is {:?}", x );
}
fn main2() {
    // 変数xを用意する
    let x: Person;
    // ブロックを開始する
    {
        // 変数aにメモリ領域を割り当てる
        let a = Person {
            name: String::from("masuda"),
            age: 50,
        };
        x = a ;
    };
    // 所有権を貰う
    println!("x is {:?}", x );
}


fn main3() {
    // 関数 new_person から所有権を受け取る
    let a = new_person( "masuda", 50 ) ;
    println!("a is {:?}", a );
}

/*
fn new_person( name: &str, age: i32 ) -> &Person {
    let p = Person {
        name: String::from(name), 
        age: age,
    };
    // 確保したメモリの参照を返そうと試みるが、
    // コンパイルすると &Person のライフタイム指定がないとの
    // エラーがでるのでコンパイルができない。
    &p 
}
*/

fn new_person( name: &str, age: i32 ) -> Person {
    let p = Person {
        name: String::from(name), 
        age: age,
    };
    // 正しく実体を返すようにする
    p 
}

/*
fn main() {
    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a );
    // 可変で参照する
    let mut x = &mut a ;
    let mut y = &mut a ;
    x.age = 0;
    y.name = String::from("kato");
    println!("a is {:?}", a );
    println!("x is {:?}", x );
    println!("y is {:?}", y );
}
*/

fn main4() {
    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a );
    // 可変で参照する
    let mut x = &mut a ;
    x.age = 0;
    println!("x is {:?}", x );
    let mut y = &mut a ;
    y.name = String::from("kato");
    println!("y is {:?}", y );
    println!("a is {:?}", a );
}

/*
fn main() {
    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a );
    // 可変で参照する
    let mut x = &mut a ;
    println!("x is {:?}", x );
    x.name = String::from("kato");
    x.age = 0;
    // 借用する順序に注意する
    println!("a is {:?}", a );
    println!("x is {:?}", x );
}
*/

fn main() {
    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a );
    // 可変で参照する
    let mut x = &mut a ;
    println!("x is {:?}", x );
    x.name = String::from("kato");
    x.age = 0;
    // 借用する順序に注意する
    println!("x is {:?}", x );
    println!("a is {:?}", a );
}



