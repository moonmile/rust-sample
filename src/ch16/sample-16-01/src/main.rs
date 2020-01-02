#[link(name="hello", kind="static")]
extern{
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}