
// 数値とクロージャを受け取る関数
fn call_with_one<F>(x: usize, func: F) -> usize
    where F: Fn(usize) -> usize {
    func(x)
}

// ベクタとクロージャを受け取る関数
fn call_with_vec<F>(v: &Vec::<usize>, func: F ) -> usize
    where F: Fn(usize) -> usize {
    let mut sum = 0 ;
    for it in v {
        sum += func(*it) ;
    }
    sum 
}

fn main() {
    let double = |x| x * 2;
    let triple = |x| x * 3;
    let a = call_with_one(100, double );
    let b = call_with_one(100, triple );
    println!("a is {}", a );
    println!("b is {}", b );

    let v = vec![1,2,3,4,5];
    let a = call_with_vec(&v, double );
    println!("a is {}", a );

    // map と sum を使って計算した場合
    let sum: usize = v.iter().map(|x| x * 2 ).sum();
    println!("sum is {}", sum );

}
