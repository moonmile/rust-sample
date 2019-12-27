use sqlite::State ;

fn main() -> sqlite::Result<()> {
    // パラメータで分岐
    let args = std::env::args().collect::<Vec<String>>();
    let cn = sqlite::open("sample.db")? ;
    if args.len() <= 1 {
        // 一覧を表示
        let mut cursor = cn
            .prepare("SELECT * FROM users")?.cursor() ;
        while let Some(row) = cursor.next()? {
            println!("name = {}, age = {}", 
                row[0].as_string().unwrap(),
                row[1].as_integer().unwrap());
        }
    } else {
        match args[1].as_str() {
            "init" => {
                let cn = sqlite::open("sample.db")? ;
                cn.execute("CREATE TABLE users (name TEXT, age INTEGER);")? ;
                cn.execute(
                    "
                    INSERT INTO users (name, age) VALUES ('Kongo', 20);
                    INSERT INTO users (name, age) VALUES ('Hieai', 20);
                    INSERT INTO users (name, age) VALUES ('Haruna', 18);
                    INSERT INTO users (name, age) VALUES ('Kirishima', 15);
                    " )?;
                println!("create database." );
            },
            "select" => {
                // 検索
                let age: i64 = args[2].parse::<i64>().unwrap() ;
                let mut cursor = cn
                    .prepare("SELECT * FROM users WHERE age > ?")?
                    .cursor() ;
                cursor.bind(&[sqlite::Value::Integer(age)])? ;
                while let Some(row) = cursor.next()? {
                    println!("name = {}, age = {}", 
                        row[0].as_string().unwrap(),
                        row[1].as_integer().unwrap());
                }
            },
            _ => {
                println!("error parameter." );
            },
        }
    }
    Ok(())
}
