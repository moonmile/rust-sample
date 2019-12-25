// String型にトレイトで拡張メソッドを付ける

fn main() {

    let s = String::from("100");
    let n = s.to_i();
    println!("n is {}", n );
}

// 既存の String 型に後付けで to_i メソッドを追加できる
// C# の拡張メソッドのような形
// トレイトが後付けでメソッドを追加する方式なので、
// inteface のように class に付ける必要がない。
trait ToNumber {
    fn to_i(&self) -> i32 ;
}
impl ToNumber for String {
    fn to_i(&self) -> i32 {
        match self.parse::<i32>() {
            Ok(n) => n,
            Err(_) => -1,
        }
    }
}

