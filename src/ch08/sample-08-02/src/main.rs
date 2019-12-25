// 構造体にメソッドを付ける
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
// グローバル変数
static mut PERSON_ID: i32 = 0 ;
// メソッドを追加
impl Person {
    // コンソールに出力する
    fn print(&self) {
        println!("{}: {} ({}) in {}", 
            self.id, self.name, self.age, self.addr );
    }
}

impl Person {
    fn print_t(&self, private: bool ) {
        if private == true {
            println!("{}: {}", 
            self.id, self.name );
        } else {
            println!("{}: {} ({}) in {}", 
            self.id, self.name, self.age, self.addr );
        }
    }
}

impl Person {
    // 文字列へフォーマットする
    fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}", 
            self.id, self.name, self.age, self.addr );
        s
    }
}

impl Person {
    // age フィールドを加算する
    fn add_age(&mut self, n: i32) {
        self.age += n ;
    }
}

impl Person {
    // 新しい Person を作る
    fn new( name: &str, age: i32, addr: &str ) -> Person {
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


fn main() {

    let pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print();

    let pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print_t( true );
    pa.print_t( false );


    let pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    let s = pa.to_str();
    println!("s is {}", s );



    let mut pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
	// メソッドの利用
    pa.print();
	pa.add_age(1);
    pa.print();

    let mut people = Vec::<Person>::new();
    people.push( Person::new("masuda", 50, "Tokyo" ));
    people.push( Person::new("kato", 30, "Osaka" ));
    people.push( Person::new("yamada", -1, "unkonwn" ));
    people.push( Person::new("sato", -1, "unkonwn" ));
    for p in &people {
        p.print();
    }
}

