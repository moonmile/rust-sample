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
    println!("press enter key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}

fn main2() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("please wait." );
    handle.join().expect("caonnt join thread.");
    println!("program end." );
}


fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #2 count {}.", i);
            thread::sleep(Duration::from_millis(2000));
        }
    });
    let handle3 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #3 count {}.", i);
            thread::sleep(Duration::from_millis(1500));
        }
    });
    println!("please wait." );
    handle.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    println!("program end." );
}
