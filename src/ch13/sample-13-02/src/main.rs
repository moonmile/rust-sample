use std::fmt;

fn main() {
    let p = Person { 
        id: 100, name: 
        String::from("masuda"), 
        age: 50, addr: 
        String::from("Tokyo"),
    };
    println!("p is {:#?}", p );
    dbg!(p);

    let p = PersonX { 
        id: 200, name: 
        String::from("kato"), 
        age: 40, addr: 
        String::from("Osaka"),
    };
    println!("p is {:#?}", p );
    dbg!(p);
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

struct PersonX {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl fmt::Debug for PersonX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ PersonX is {}: {} in {} }}", 
            self.id, self.name, self.addr)
    }
}