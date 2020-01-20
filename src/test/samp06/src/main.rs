use std::thread;
use std::time::Duration;
use std::io::Read ;

fn main1() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("please wait." );
    handle.join().unwrap();
    println!("program end." );
}

async fn foo(id: i32) -> i32 {
    for i in 0..10 {
        println!("thread #1 count {}.", i);
        thread::sleep(Duration::from_millis(1000));
    }
    id 
}
#[tokio::main]
async fn main2() {
    println!("program start.");
    foo(10).await ;
    foo(20).await ;
    foo(30).await ;
    println!("program end.");
}

#[tokio::main]
async fn main3() {
    // この時点では foo は実行されない
    let f10 = foo(10);
    let f20 = foo(20);
    let f30 = foo(30);
    println!("program start.");
    // .await をつけると実行される
    f10.await ;    
    f20.await ;    
    f30.await ;    
    println!("program end.");
}

/**
 * ちょうど thread::pack したと同じ状態で、処理が停止されている
 * .await を呼び出すと、unpark されて処理が実行される感じになる。
 */

fn main() {
    let handle = thread::Builder::new().spawn(|| {
        thread::park(); // 一時停止しておく
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    }).unwrap();

    println!("please wait 5 sec." );
    // 5秒待つ
    thread::sleep(Duration::from_millis(5000));
    println!("thread start." );
    handle.thread().unpark();
    handle.join().unwrap();
    println!("program end." );
}

