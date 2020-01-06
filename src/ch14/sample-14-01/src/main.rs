/**
 * $ sudo apt-get install pkg-config libssl-dev
 * 
 * 
[dependencies]
reqwest = "0.10"
serde_json = "1.0"
tokio = { version = "0.2", features = ["full"] }
 *
 */
// URL呼び出し
#[tokio::main]
async fn main1() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/blog/" ;
    println!("call {}", url );
    let res = reqwest::get( url ).await? ;
    let body = res.text().await? ;
    println!("response is \n{}", body );
    Ok(())
}

// クエリ文字列を使って呼び出し
#[tokio::main]
async fn main2() -> Result<(), Box<dyn std::error::Error>> {
    let n = 7 ;
    let url = format!( "http://openccpm.com/blog/?p={}", n );
    println!("call {}", url );
    let res = reqwest::get( &url ).await? ;
    let body = res.text().await? ;
    println!("response is \n{}", body );
    Ok(())
}

// 存在しないページを検索
use reqwest::StatusCode ;
#[tokio::main]
async fn main3() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/unknown.txt" ;
    println!("call {}", url );
    let res = reqwest::get( url ).await? ;
    match res.status() {
        StatusCode::OK => {
            let body = res.text().await? ;
            println!("response is \n{}", body );
        },
        StatusCode::NOT_FOUND => {
            println!("error: 目的のページがありませんでした。" );
        },
        _ => {
            println!("error: その他のエラーが発生しました。" );
        },
    }
    Ok(())
}

// 存在しないサーバーで検索
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://unknown.openccpm.com/blog" ;
    println!("call {}", url );
    if let Ok(res) = reqwest::get( url ).await {
        let body = res.text().await? ;
        println!("response is \n{}", body );
    } else {
        println!("error: WEBサーバーが見つかりませんでした。" );
    }
    Ok(())
}
