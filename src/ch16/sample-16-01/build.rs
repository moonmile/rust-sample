extern crate cc;

fn main(){
    cc::Build::new()
                .file("src/c/hello.c")
                .include("src")
                .compile("libhello.a");
}
