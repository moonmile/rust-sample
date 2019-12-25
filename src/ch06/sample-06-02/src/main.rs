fn main() {

    let v = vec![10,20,30,40,50] ;

    // 全ての要素を繰り返す
    print!("v is ");
    for i in &v {
        print!("{} ", i );
    }
    println!("");

    // イテレータを使う
    print!("v is ");
    for i in v.iter() {
        print!("{} ", i );
    }
    println!("");

    // インデックス付きで繰り返す
    print!("v is ");
    for (i, x) in v.iter().enumerate() {
        print!("{}:{} ", i, x );
    }
    println!("");

    // 繰り返し数を指定する
    print!("FOR is ");
    for i in 0..10 {
        print!("{} ", i );
    }
    println!("");

    // 途中で繰り返しを止める
    print!("FOR is ");
    for i in 0..10 {
        if i == 5 {
            break;
        }
        print!("{} ", i );
    }
    println!("");

    // 途中で先頭に戻る
    print!("FOR is ");
    for i in 0..10 {
        if i % 2 == 0 {
            continue ;
        }
        print!("{} ", i );
    }
    println!("");

    // while を使う
    print!("WHILE is ");
    let mut i = 0;
    while i < 10 {
        print!("{} ", i );
        i += 2 ;
    }
    println!("");

    // loop を使う
    print!("LOOP is ");
    let mut i = 0;
    loop {
        if i >= 10 {
            break ;
        }
        print!("{} ", i );
        i += 1 ;
    }
    println!("");
}
