extern crate reqwest;
extern crate serde_json;

// GET呼び出し
fn _main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/blog/" ;
    println!("call {}", url );
    let mut res = reqwest::get( url )? ;
    let body = res.text()? ;
    println!("response is \n{}", body );
    Ok(())
}

// POST呼び出し
// 適当なPOST用のURLを用意しておく
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let params = [("name", "masuda"), ("age", "50")];
    let client = reqwest::Client::new();
    let url = "http://openccpm.com/blog/";
    println!("call {}", url );
    let mut res = client.post("http://openccpm.com/blog/")
        .form(&params)
        .send()?;
    let body = res.text()? ;
    println!("response is \n{}", body );
    Ok(())
}

