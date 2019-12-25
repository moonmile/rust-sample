fn main() {
    let mut x = 100 ;
    println!("x is {}", x ); 
    x = 200 ;
    println!("x is {}", x ); 


    let name = get_person_name();
    let age = get_person_age();
    //
    // 個々に色々な処理が入る
    // 年齢が50以上だったら、50に入れ直す
    if age > 50 {
        age = 50 ;
    }
    // 
    println!("name is {}, age {}", name, age ); 

}

fn get_person_name() -> &'static str { "masuda" }
fn get_person_age() -> i32 { 50 }

fn test() {
    let x = 100 ;
    println!("x is {}", x ); 
    let x = 200 ;
    println!("x is {}", x ); 
} 



