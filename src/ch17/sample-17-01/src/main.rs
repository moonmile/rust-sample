fn main() {
    println!("use map" );
    let a = [1,2,3,4,5];
    let b = a.iter().map(|x| x*x );
    for it in b {
        println!("it is {}", it );
    }

    println!("use filter" );
    let a = [1,2,3,4,5];
    let b = a.iter().filter(|&x| x % 2 == 1 );
    for it in b {
        println!("it is {}", it );
    }

    println!("use find" );
    let a = [1,2,3,4,5];
    let b = a.iter().find(|&&x| x == 3 );
    let c = a.iter().find(|&&x| x > 10 );
    println!("b is {:?}", b );
    println!("c is {:?}", c );
    
    println!("use max and min" );
    let a = [1,2,3,4,5];
    let max = a.iter().max() ;
    let min = a.iter().min() ;
    println!("max is {:?}", max );
    println!("min is {:?}", min );

    println!("use zip" );
    let a = [1,2,3,4,5];
    let b = ["one","two","three","four","five"];
    let c: Vec<_> = a.iter().zip(b.iter()).collect();
    for (n,w) in c {
        println!("c is {} and {}", n, w );
    }
}
