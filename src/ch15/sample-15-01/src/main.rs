use sqlite::State ;

fn main() -> sqlite::Result<()> {
    // インメモリーデータベースの作成
    let cn = sqlite::open(":memory:")? ;
    // テーブル作成とデータ挿入
    cn.execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users (name, age) VALUES ('Kongo', 20);
            INSERT INTO users (name, age) VALUES ('Hieai', 20);
            INSERT INTO users (name, age) VALUES ('Haruna', 18);
            INSERT INTO users (name, age) VALUES ('Kirishima', 15);
            " )?;
    // クエリの作成
    let mut statement = cn
        .prepare("SELECT * FROM users WHERE age > 15")? ;
    // データを取得
    while let Ok(State::Row) = statement.next() {
        println!("name = {}, age = {}", 
            statement.read::<String>(0)?,
            statement.read::<i64>(1)? );
    }
    Ok(())
}

fn _main() -> sqlite::Result<()> {
    // インメモリーデータベースの作成
    let cn = sqlite::open(":memory:")? ;
    // テーブル作成とデータ挿入
    cn.execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users (name, age) VALUES ('Kongo', 20);
            INSERT INTO users (name, age) VALUES ('Hieai', 20);
            INSERT INTO users (name, age) VALUES ('Haruna', 18);
            INSERT INTO users (name, age) VALUES ('Kirishima', 15);
            " )?;
    // カーソルを作成する
    let mut cursor = cn
        .prepare("SELECT * FROM users WHERE age > 15")? 
        .cursor();
    // データを取得
    while let Some(row) = cursor.next()? {
        println!("name = {}, age = {}", 
            row[0].as_string().unwrap(),
            row[1].as_integer().unwrap());
    }
    Ok(())
}