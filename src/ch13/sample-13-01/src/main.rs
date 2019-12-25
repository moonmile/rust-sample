// println! マクロの例
fn main() {

    println!("hello rust world.");
    println!("hello {} world.", "rust");
    // 数値の表示
    println!("nunber is {}.", 10 );
    println!("float is {}.", 10.234 );
    // タプルの表示
    println!("tuple is {:?}.", (10,20) );
    // オプションの表示
    println!("option is {:?}.", Some(10) );
    // 複数指定    
    println!("a is {}, b is {}", 100, "test" );
    // 名前付きで指定
    println!("a is {a}, b is {b}", a=100, b="test" );

    // 数値のフォーマット
    let n = 10 ;
    let f = 10.234 ;
    println!("{} {}", n, f );
    println!("{:x} ", n );      // 16進数（小文字）
    println!("{:X} ", n );      // 16進数（大文字）
    println!("{:o} ", n );      // 8進数
    println!("{:b} ", n );      // 2進数
    println!("{:e} ", f );      // 指数表現（小文字）
    println!("{:E} ", f );      // 指数表現（大文字）
    println!("{:.2}", f );      // 小数点以下の桁数を指定
    println!("{:4}", n );       // 桁数指定
    println!("{:04}", n );      // 0埋め

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
