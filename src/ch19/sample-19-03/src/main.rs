fn main() {
    println!("Hello, Rust world!");

    let mut a = Person { id: 100, name: "masuda".to_string(), age: 50 };
    let b = Person { id: 100, name: "masuda".to_string(), age: 50 };
    let c = Person { id: 200, name: "kato".to_string(), age: 40 };

    // プログラムの事前チェックに使う
    assert_eq!( "masuda", a.name );
    assert_eq!( "kato", c.name );
    // assert_eq!( "unknown", c.name ); // ここでプログラムが止まる

    println!("a is {:?}", a );
    println!("c is {:?}", c );
    a.age += 1 ;
    println!("a is {:?}", a );

}

#[derive(Debug,PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32
}

// テストコードを書いておく
#[cfg(test)]
mod tests {
    use super::* ;
    #[test]
    fn test_equal_instance() {
        let mut a = Person { id: 100, name: "masuda".to_string(), age: 50 };
        let b = Person { id: 100, name: "masuda".to_string(), age: 50 };
        let c = Person { id: 200, name: "kato".to_string(), age: 40 };

        assert_eq!( a, a );
        assert_eq!( a, b ); // 内容が同じ
        assert_ne!( a, c ); // ここは failed 

        a.age += 1 ;
        assert_ne!( a, b ); // a.age が異なる

        let x = &a ;        // 参照
        assert_eq!( a, *x); // 同じものを示している
    }
}
