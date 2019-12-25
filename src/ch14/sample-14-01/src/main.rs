extern crate reqwest;
extern crate serde_json;

fn main() {
    let _ = go() ;
}

fn go() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}", url );
    let mut res = reqwest::get( url )? ;
    let body = res.text()? ;
    let v: serde_json::Value = serde_json::from_str(&body)? ;

    println!("projects[0] is {:#?}", v["projects"][0]["name"] );
    Ok(())
}