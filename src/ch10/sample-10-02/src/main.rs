fn main() {
    // ダングリングポインタを回避する

    // 変数 x を用意する
    let x: &Person;
    // ブロックを開始する
    {
        // 変数 a にメモリ領域を割り当てる
        let a = Person {
            id: 100,
            name: String::from("masuda"),
            age: 50,
            addr: String::from("Tokyo"),
        };
        // 変数 x に参照させる
        x = &a ;
        // ブロックを抜ける
    }
    // これは参照できるか？
    // 既に a のライフタイムが切れているので、
    // a への参照は無効になる
    // println!("x.name is {}", x.name );

    // 関数 new_person から所有権を受け取る
    let a = new_person( 100, "masuda") ;
    println!("a.name is {}", a.name );
}

struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}


/*
fn new_person( id: i32, name: &str ) -> &Person {
    let p = Person {
        id: id,
        name: name,
        age: -1,
        addr: String::from("Unknown"),
    };
    // 確保したメモリの参照を返そうと試みるが、
    // コンパイルすると &Person のライフタイム指定がないとの
    // エラーがでるのでコンパイルができない。
    &p 
}
*/

fn new_person( id: i32, name: &str ) -> Person {
    let p = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    // 正しく実体を返すようにする
    p 
}


