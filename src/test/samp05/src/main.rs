
async fn foo(id: i32) {
    for i in 1..10 {
        println!("hi number {} in foo({}).", i, id);
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main2() {
    println!("program start.");
    foo(10).await ;
    foo(20).await ;
    foo(30).await ;
    println!("program end.");
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    println!("program start.");
    rt.block_on( async {
        foo(10).await ;
        foo(20).await ;
        foo(30).await ;
    });
    println!("program end.");
}

