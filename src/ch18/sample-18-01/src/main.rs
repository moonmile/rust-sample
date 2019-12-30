use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("please wait." );
    handle.join().unwrap();
    println!("program end." );
}
