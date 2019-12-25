mod sub ;

fn main() {
    let s = "hello" ;
    let c = &s[0..0] ;
    let c1 = &s[1..] ;

    println!("s ptr is {:?}", s.as_ptr());
    println!("c ptr is {:?}", c.as_ptr());
    println!("c1 ptr is {:?}", c1.as_ptr());

    let s = "  Hello   ";
    println!("s is '{}'", s);
    println!("s.trim is '{}'", s.trim());

    let s = "  こんにちは   今日は  ";
    println!("s is '{}'", s);
    println!("s.trim is '{}'", s.trim());
 
    let s = "HELLO\nRUST\nWORLD." ;
    let mut l = s.lines();
    println!("1: {}", l.next().unwrap());
    println!("2: {}", l.next().unwrap());
    println!("3: {}", l.next().unwrap());

    let s = "こんにちは\nRUST\nの世界へ" ;
    let mut l = s.lines();
    println!("1: {:?}", l.next());
    println!("2: {:?}", l.next());
    println!("3: {:?}", l.next());


    let mut pa = sub::Person::new("masuda", 50, "Tokyo");
    pa.print();

    // 元の Vec には sum メソッドはないが、
    // 別名で定義した MyVec の sum が呼び出されるため
    // Vec.sum があるように見える
    let v = vec![1,2,3,4,5] ;
    let sum = v.sum();
    println!("sum is {}", sum );
}

// 同じクレートならば、そのまま impl でメソッドを追加できる
use crate::sub::Person;
impl Person {
    fn sum() -> i32 {
        0 
    }
}

// 別のクレートの場合は type でエイリアスを追加した後に、
// trait でインターフェースを増やせばよい
// C# の拡張メソッドに似た感じ
type VecEx = Vec<i32> ;
trait MyVecExtenstions {
    fn sum(&self) -> i32 ;
}

impl MyVecExtenstions for VecEx {
    fn sum(&self) -> i32 {
        let mut x: i32 = 0;
        for i in self {
            x += i ;
        }
        x 
    }
}

