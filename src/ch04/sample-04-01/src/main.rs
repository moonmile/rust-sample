fn main() {
    let ch = 'A' ;
    println!("ch is {}", ch ); 
    let ch = 'あ' ;
    println!("ch is {}", ch ); 
    let ch = '🐱' ;
    println!("ch is {}", ch ); 
    let ch: char = '🐶' ;
    println!("ch is {}", ch ); 
    let ch = '\u{1F431}' ;
    println!("ch is {}", ch ); 
    println!("char size of {}", std::mem::size_of::<char>()) ;

    test();
}

fn test() {
    let ch = 'A' ;
    println!("ch is {}", ch ); 
    let u  = ch as u8 ;
    println!("u is {}", u ); 
    let ch = u as char ;
    println!("ch is {}", ch ); 
}
