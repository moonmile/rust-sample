
#[tokio::main]
async fn main1() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}", url );
    let res = reqwest::get( url ).await? ;
    let body = res.text().await? ;
    println!("response is \n{}", body );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}", url );
    let res = reqwest::get( url ).await? ;
    let body = res.text().await? ;
    let json: serde_json::Value = serde_json::from_str(&body)? ;
    let projects = json["projects"].as_array().unwrap() ;
    // println!("projects is {:#?}", projects );
    for item in projects {
        let identifier = &item["identifier"].as_str().unwrap();
        let name = &item["name"].as_str().unwrap();
        println!("tag: {} {}", identifier, name ) ;
    }
    Ok(())
}
