fn main() {
    // ベクタを定義
    let v = vec![1,2,3,4,5] ;
    // for文で表示
    print!("for is ") ;
    for i in &v {
        print!("{} ", i ) ;
    }
    println!("");
    // for文とイテレーターの利用
    print!("for and iter is ") ;
    for i in v.iter() {
        print!("{} ", i ) ;
    }
    println!("");
    // nextメソッドの利用
    let mut p = v.iter();
    println!("p is {:?}", p);
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());    // 6回目はNoneになる
    // loop文で冗長に書く
    println!("by loop");
    let mut p = v.iter() ;
    loop {
        let x = p.next() ;
        if x == None {
            break;
        }
        println!("x is {}", x.unwrap());
    }
    // while文で書き直す
    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("x is {}", x);
    }
    // map を使う
    let lst = v.iter().map(|x| x * 10 ) ;
    for i in lst {
        println!("i is {}", i);
    }
}
