use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    // パラメータで分岐
    let args = std::env::args().collect::<Vec<String>>();
    let cn = Connection::open("sample.db")?;
    if args.len() <= 1 {
        // 一覧を表示
        let mut stmt = cn.prepare("SELECT * FROM users")? ;
        for it in stmt.query_map(params![], |row| {
            Ok(User{
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        })? {
            println!("user is {:?}", it.unwrap());
        }
    } else {
        match args[1].as_str() {
            "init" => {
                cn.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)", params![] )?;
                println!("create database." );
            },
            "into" => {
                // データ挿入
                let id: i32 = args[2].parse::<i32>().unwrap() ;
                let name = &args[3];
                let age: i32 = args[4].parse::<i32>().unwrap() ;
                let mut stmt = cn.prepare("INSERT INTO users (id, name, age) VALUES (?1, ?2, ?3)")?;
                stmt.execute(params![id, name, age])?;
                println!("insert data." );
            },
            _ => {
                println!("error parameter." );
            },
        }
    }
    Ok(())
}
