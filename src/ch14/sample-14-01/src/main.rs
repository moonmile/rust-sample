/**
 * $ sudo apt-get install pkg-config libssl-dev
 * 
 * 
[dependencies]
reqwest = "0.9.22"
serde_json = "1.0.44"
 *
 */


extern crate reqwest;
extern crate serde_json;

// URL呼び出し
fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/blog/" ;
    println!("call {}", url );
    let mut res = reqwest::get( url )? ;
    let body = res.text()? ;
    println!("response is \n{}", body );
    Ok(())
}

// クエリ文字列を使って呼び出し
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 7 ;
    let url = format!( "http://openccpm.com/blog/?p={}", n );
    println!("call {}", url );
    let mut res = reqwest::get( &url )? ;
    let body = res.text()? ;
    println!("response is \n{}", body );
    Ok(())
}


