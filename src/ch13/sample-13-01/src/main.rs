fn main() {
    println!("hello rust world.");
    println!("hello {} world.", "rust");
    // 数値の表示
    println!("nunber is {}.", 10 );
    println!("float is {}.", 10.234 );
    // タプルの表示
    println!("tuple is {:?}.", ("masuda",20));
    // オプションの表示
    let n = Option::<i32>::Some(10);
    println!("option is {:?}.", n );
    let n = Option::<i32>::None;
    println!("option is {:?}.", n );
    // 複数指定    
    println!("a is {}, b is {}", 100, "test" );
    // 名前付きで指定
    println!("a is {a}, b is {b}", a=100, b="test" );

    // 数値のフォーマット
    let n = 10 ;
    println!("10進数 {}", n );
    println!("16進数 {:x} ", n );      // 16進数（小文字）
    println!("16進数 {:X} ", n );      // 16進数（大文字）
    println!(" 8進数 {:o} ", n );      // 8進数
    println!(" 2進数 {:b} ", n );      // 2進数

    let n = 10 ;
    println!("通常   {}", n );         // 通常
    println!("桁揃え {:4}", n );       // 桁数指定
    println!("10進数 {:04}", n );      // 0埋め(10進数)
    println!("16進数 {:02X}", n );     // 0埋め(16進数)
    println!(" 2進数 {:08b}", n );     // 0埋め(2進数)

    let f = 10.234 ;
    println!("{} ", f );        // 標準
    println!("{:e} ", f );      // 指数表現（小文字）
    println!("{:E} ", f );      // 指数表現（大文字）
    println!("{:.2}", f );      // 小数点以下の桁数を指定
    let f = 0.0234 ;
    println!("{} ", f );        // 標準
    println!("{:e} ", f );      // 指数表現（小文字）
    println!("{:E} ", f );      // 指数表現（大文字）
    println!("{:.2}", f );      // 小数点以下の桁数を指定

    // 配置
    println!("hello, `{:8}` world.", "rust");
    println!("hello, `{:<8}` world.", "rust");
    println!("hello, `{:>8}` world.", "rust");
    println!("hello, `{:^8}` world.", "rust");
    println!("hello, `{:8}` world.", 123);
    println!("hello, `{:<8}` world.", 123);
    println!("hello, `{:>8}` world.", 123);
    println!("hello, `{:^8}` world.", 123);
}
