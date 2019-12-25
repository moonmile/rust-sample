fn main() {
    // タプルを作る
    let t = ("masuda", 30 );
    println!("name is {} age {}", t.0, t.1 );
    // 変数を使ってタプルを作る
    let name = "masuda" ;
    let age = 30 ;
    let t = ( name, age );
    println!("name is {} age {}", t.0, t.1 );


let a = ["春","夏","秋", "冬"] ;
println!("最初の季節 {}", a[0]) ;
println!("最後の季節 {}", a[3]) ;

// 添え字が即値の場合は、ビルド時にエラーになる
// println!("無効な季節 {}", a[10]) ;
    // 添え字が変数の場合は、実行時にエラーになる
let i = 10 ;
println!("無効な季節 {}", a[i]) ;
}
