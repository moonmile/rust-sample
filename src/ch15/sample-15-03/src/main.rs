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
                cn.execute_batch(
                    "
                    CREATE TABLE users (id INTEGER, name TEXT, age INTEGER) ;
                    INSERT INTO users (id, name, age) VALUES (1, 'Kongo', 20) ;
                    INSERT INTO users (id, name, age) VALUES (2, 'Hieai', 20) ;
                    INSERT INTO users (id, name, age) VALUES (3, 'Haruna', 18) ;
                    INSERT INTO users (id, name, age) VALUES (4, 'Kirishima', 15) ;
                    "
                )? ;
                println!("create database." );
            },
            "select" => {
                // データ検索
                let age: i32 = args[2].parse::<i32>().unwrap() ;
                let mut stmt = cn.prepare("SELECT * FROM users WHERE age > ?")? ;
                let iter = stmt.query_map(params![age], |row| {
                    Ok(User{
                        id: row.get(0)?,
                        name: row.get(1)?,
                        age: row.get(2)?,
                    })
                })?;
                for it in iter {
                    println!("user is {:?}", it.unwrap());
                }
            },
            _ => {
                println!("error parameter." );
            },
        }
    }
    Ok(())
}
