use std::io::prelude::*;
// use std::io::BufRead ;
fn main() {
    println!("Enter a number: ");
    // 1行だけ読み込む
    if let Some(Ok(text)) = std::io::stdin().lock().lines().next() {
        println!("text is {}", text );
        let n: i32= text.trim().parse()
            .expect("error: no number.");
        println!("n is {}", n );
    }
}
