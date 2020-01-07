// GET呼び出し
#[tokio::main]
async fn main1() -> Result<(), Box<dyn std::error::Error>> {
    let name = "masuda";
    let age = 50 ;
    let url = format!("http://localhost/rust/get.php?name={}&age={}", name, age );
    println!("call {}", url );
    let res = reqwest::get( &url ).await? ;
    let body = res.text().await? ;
    println!("response is \n{}", body );
    Ok(())
}

// POST呼び出し
// 適当なPOST用のURLを用意しておく
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = [("name", "masuda"), ("age", "50")];
    let client = reqwest::Client::new();
    let url = "http://localhost/rust/post.php";
    println!("call {}", url );
    let res = client.post(url)
        .form(&params)
        .send().await?;
    let body = res.text().await? ;
    println!("response is \n{}", body );
    Ok(())
}

