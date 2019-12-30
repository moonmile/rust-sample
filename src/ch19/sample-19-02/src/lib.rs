#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn test_equal() {
        assert_eq!(2, 1+1);
        assert_eq!(1.123, 1.0 + 0.123 );
        assert_eq!(true, 1 == 1);
        assert_eq!("rust", " rust ".trim());
    }

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

    #[test]
    fn test_not_equal() {
        assert_ne!( 0, 1+1 );
        assert_ne!( "Javascript", "Java");
    }

    #[test]
    fn test_assert() {
        assert!( true );
        assert!( 1 == 1 );
    }
}

#[derive(Debug,PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32
}
