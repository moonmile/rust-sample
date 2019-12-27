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
                println!("create database." );
            },
            "into" => {
                // データ挿入
                let name = &args[2];
                let age: i64 = args[3].parse::<i64>().unwrap() ;
                let sql = format!(
                    "INSERT INTO users (name, age) VALUES ('{}', {});", name, age) ;
                cn.execute( sql )? ;
                println!("insert data." );
            },
            _ => {
                println!("error parameter." );
            },
        }
    }
    Ok(())
}
