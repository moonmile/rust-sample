use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main1() -> Result<()> {
    // インメモリーデータベースの作成
    let cn = Connection::open_in_memory()?;
    // テーブル作成とデータ挿入
    cn.execute_batch(
        "
        CREATE TABLE users (id INTEGER, name TEXT, age INTEGER) ;
        INSERT INTO users (id, name, age) VALUES (1, 'Kongo', 20) ;
        INSERT INTO users (id, name, age) VALUES (2, 'Hieai', 20) ;
        INSERT INTO users (id, name, age) VALUES (3, 'Haruna', 18) ;
        INSERT INTO users (id, name, age) VALUES (4, 'Kirishima', 15) ;
        "
    )? ;
    // クエリの作成
    let mut stmt = cn.prepare("SELECT * FROM users WHERE age > 15")? ;
    let iter = stmt.query_map(params![], |row| {
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    // データを取得
    for it in iter {
        println!("user is {:?}", it.unwrap());
    }
    Ok(())
}

fn main() -> Result<()> {
    // インメモリーデータベースの作成
    let cn = Connection::open_in_memory()?;
    // テーブル作成とデータ挿入
    cn.execute_batch(
        "
        CREATE TABLE users (id INTEGER, name TEXT, age INTEGER) ;
        INSERT INTO users (id, name, age) VALUES (1, 'Kongo', 20) ;
        INSERT INTO users (id, name, age) VALUES (2, 'Hieai', 20) ;
        INSERT INTO users (id, name, age) VALUES (3, 'Haruna', 18) ;
        INSERT INTO users (id, name, age) VALUES (4, 'Kirishima', 15) ;
        "
    )? ;
    // クエリの作成
    let mut stmt = cn.prepare("SELECT * FROM users WHERE age > 15")? ;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let it = User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        };
        println!("it is {:?}", it ); 
    }
    Ok(())
}

fn main3() -> Result<()> {
    // インメモリーデータベースの作成
    let cn = Connection::open_in_memory()?;
    // テーブル作成
    cn.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)", params![] )?;
    // データ挿入
    let mut stmt = cn.prepare("INSERT INTO users (id, name, age) VALUES (?1, ?2, ?3)")?;
    stmt.execute(params![1, "Kongo", 20 ])?;
    stmt.execute(params![2, "Hieai", 20 ])?;
    stmt.execute(params![3, "Haruna", 18 ])?;
    stmt.execute(params![4, "Kirishima", 15 ])?;
    // クエリの作成
    let mut stmt = cn.prepare("SELECT * FROM users WHERE age > ?")? ;
    let iter = stmt.query_map(params![15], |row| {
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    // データを取得
    for it in iter {
        println!("user is {:?}", it.unwrap());
    }
    Ok(())
}
