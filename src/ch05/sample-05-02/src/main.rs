fn main() {
    // 一般的な四則演算子
    let a = 20 + 10 ;
    println!("a is {}", a );
    let a = 20 - 10 ;
    println!("a is {}", a );
    let a = 20 * 10 ;
    println!("a is {}", a );
    let a = 20 / 10 ;
    println!("a is {}", a );
    let a = 20 % 3 ;
    println!("a is {}", a );

    // 割り算で整数と実数の違い
    let a = 10 ;
    let b = 3 ;
    let ans = a / b ;
    println!("a / b is {}", ans );
    let a = 10.0 ;
    let b = 3.0 ;
    let ans = a / b ;
    println!("a / b is {}", ans );

    // 加算しながら代入ができる
    let mut a = 10 ;
    a += 20 ;
    // a = a + 20 ; と同じ
    println!("a is {}", a );

    let mut sum = 0;
    for i in 0..10 {
        sum += i ;
    }
    println!("sum is {}", sum );

    // シフト演算子
    let a : u8 = 0x02 ;
    println!("a << 1 is {}", a << 1 );
    println!("a >> 1 is {}", a >> 1 );

    // ビット演算子
    let a : u8 = 0b1111 ;
    let b : u8 = 0b0011 ;
    println!("a & b is {:04b}", a & b );
    println!("a | b is {:04b}", a | b );

    // 論理演算子
    let a = true ;
    let b = false ;
    println!("a && b is {}", a && b );
    println!("a || b is {}", a || b );

    // 単項演算子
    let a = true ;
    let b = !a ;
    println!("a is {}, b is {}", a, b );

}
