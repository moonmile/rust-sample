pub struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID: i32 = 0 ;

// メソッドを追加
impl Person {
    // コンソールに出力する
    pub fn print(&self) {
        println!("{}: {} ({}) in {}", 
            self.id, self.name, self.age, self.addr );
    }
    // 文字列へフォーマットする
    pub fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}", 
            self.id, self.name, self.age, self.addr );
        s
    }
    // age フィールドを加算する
    pub fn add_age(&mut self, n: i32) {
        self.age += n ;
    }
    // 新しい Person を作る
    pub fn new( name: &str, age: i32, addr: &str ) -> Person {
        // グローバル変数を使い、新しい id を作成
        let id = unsafe {
            PERSON_ID += 1 ; 
            PERSON_ID
        } ;
        // Person を返す
        Person {
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
}