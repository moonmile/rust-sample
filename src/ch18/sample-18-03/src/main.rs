use futures::executor::ThreadPool;
use std::io::Read ;

fn main() {
    let pool = ThreadPool::new().unwrap();
    let task = async {
        for j in 1..6 {
            let id = j * 10 ;
            pool.spawn_ok(async move {
                for i in 1..10 {
                    println!("hi number {} in id:{}.", i, id);
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            });
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    };
    println!("program start.");
    futures::executor::block_on(task);
    println!("press any key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}
