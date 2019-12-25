fn main() {
    // ベクタを定義
    let v = vec![1,2,3,4,5] ;
    println!("v[0] is {}", v[0] );
    println!("v[4] is {}", v[4] );
    println!("v.len is {}", v.len());

    // getメソッドを使う
    println!("v.get(0) is {:?}", v.get(0));
    println!("v.get(4) is {:?}", v.get(4));
    println!("v.get(0) is {}", v.get(0).unwrap());
    println!("v.get(4) is {}", v.get(4).unwrap());

    // 文字列のベクタ
    let v = vec!["one","two","three","four","five"] ;
    println!("v[0] is {}", v[0] );
    println!("v[4] is {}", v[4] );
    println!("v.len is {}", v.len());

    // 型を指定して初期化
    let mut v: Vec<i32> = Vec::new() ;
    // 次の書き方でもよい
    // let mut v = Vec::<i32>::new() ;
    // let mut v: Vec<i32> = vec![];
    println!("v.len is {}", v.len());
    // 要素を末尾に追加
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v.len is {}", v.len());
    // 要素を末尾から取得
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("v.len is {}", v.len());

    let mut v = vec![1,2,3,4,5] ;
    // 先頭の要素
    println!("v.first is {:?}", v.first());
    // 末尾の要素
    println!("v.last is {:?}", v.last());
    // getメソッドで取得
    println!("v.get(1) is {:?}", v.get(1));
    println!("v.get(10) is {:?}", v.get(10));

    println!("v.first is {}", v.first().unwrap());
    println!("v.last is {}", v.last().unwrap());

    // 先頭を削除
    println!("v.first is {:?}", v.first());
    println!("v.remove(0) is {:?}", v.remove(0));
    println!("v.first is {:?}", v.first());

    // 先頭に要素を追加
    println!("v.first is {:?}", v.first());
    v.insert(0, 10);
    println!("v.first is {:?}", v.first());
    // 末尾にも追加できる
    v.insert(v.len(), 99);
    println!("v.last is {:?}", v.last());

    // 2つのベクターを繋げる
    let a = vec![1,2,3] ;
    let b = vec![4,5] ;
    let v = [a,b].concat();
    println!("v.len is {}", v.len());
    for i in v {
        print!("{} ", i);
    }
    println!("");


    // 接続文字を指定してひとつに連結
    let v = vec!["one","two","three","four","five"] ;
    let x = v.join("-") ;
    println!("x is {}", x);

    // 文字を指定して分割 split
    let s = "one,two,three,four,five" ;
    let v = s.split(',');
    for x in v {
        print!("{} ", x ) ;
    }
    println!("");

    // ソート
    let mut v = vec!["one","two","three","four","five"] ;
    v.sort() ;
    let x = v.join(" ") ;
    println!("x is {}", x);
    // 逆順にする
    v.reverse() ;
    let x = v.join(" ") ;
    println!("x is {}", x);

    // ソート
    let mut v = vec!["aaa","abc","AAA","ABC","AaA"] ;
    v.sort() ;
    let x = v.join(" ") ;
    println!("x is {}", x);
    // 逆順にする
    v.reverse() ;
    let x = v.join(" ") ;
    println!("x is {}", x);

}


