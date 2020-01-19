async fn foo(id: i32) {
    for i in 1..10 {
        println!("hi number {} in foo({}).", i, id);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
fn main() {
    let task = async {
        foo(10).await ;
        foo(20).await ;
        foo(30).await ;
    };
    println!("program start.");
    futures::executor::block_on(task);
    println!("program end.");
}


async fn go() {
    foo(10).await;
    foo(20).await;
    foo(30).await;
}
fn main2() {
    println!("program start.");
    futures::executor::block_on(go());
    println!("program end.");
}

use futures::executor::ThreadPool;
use std::io::Read ;

fn main3() {
    let mut pool = ThreadPool::new().unwrap();
    println!("program start.");
    pool.spawn_ok(foo(10));
    pool.spawn_ok(foo(20));
    pool.spawn_ok(foo(30));
    println!("press any key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}


fn main4() {
    let mut tpb = futures::executor::ThreadPoolBuilder::new();
    // スレッドプールを2つに制限するため
    // 最初の2つだけ同時に動く
    let pb = tpb.pool_size(2);
    let pool = pb.create().unwrap();
    println!("program start.");
    pool.spawn_ok(foo(10));
    pool.spawn_ok(foo(20));
    pool.spawn_ok(foo(30));
    println!("press any key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}

/*
async fn foo(id: i32) {
    for i in 1..10 {
        println!("hi number {} in foo({}).", i, id);
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    }
}
*/

#[tokio::main]
async fn main6() {
    println!("program start.");
    foo(10).await ;
    foo(20).await ;
    foo(30).await ;
    println!("program end.");
}

fn main7() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    println!("program start.");
    rt.block_on(async {
        foo(10).await ;
        foo(20).await ;
        foo(30).await ;
    });
    println!("program end.");
}

