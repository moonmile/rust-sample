extern crate reqwest;
extern crate serde_json;

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}", url );
    let mut res = reqwest::get( url )? ;
    let body = res.text()? ;
    println!("response is \n{}", body );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}", url );
    let mut res = reqwest::get( url )? ;
    let body = res.text()? ;
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
