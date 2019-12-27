use std::fs::File;
use std::io::Write;

fn main() {
    // 1バイトずつ書き出す
    let path = "sample.txt" ;
    let mut file = File::create( path )
        .expect("file not found.");
    let s = "hello rust world.\n";
    for it in s.as_bytes() {
        file.write(&[*it])
            .expect("cannot write.");
        let ch = *it ;   // 実体を取得
        let ary = [ch];  // 配列に直す
        file.write(&ary) // 参照を渡す 
            .expect("cannot write.");
    }
}


fn _main() {
    // write!マクロで書き出す
    let path = "sample.txt" ;
    let mut file = File::create( path )
        .expect("file not found.");
    write!( file, "hello rust world.\n")
        .expect("cannot write.");

    // write メソッドで書き出す
    // [u8] のバイト配列なので、b で修飾する
    let path = "sample2.txt" ;
    let mut file = File::create( path )
        .expect("file not found.");
    file.write( b"hello rust world.\n")
        .expect("cannot write.");

    // &str 型を [u8] に直して渡しても良い
    let path = "sample3.txt" ;
    let mut file = File::create( path )
        .expect("file not found.");
    let s = "hello rust world.\n";
    file.write(s.as_bytes())
        .expect("cannot write.");

    // 1バイトずつ書き出す
    let path = "sample4.txt" ;
    let mut file = File::create( path )
        .expect("file not found.");
    let s = "hello rust world.\n";
    for i in s.as_bytes() {
        file.write(&[*i])
            .expect("cannot write.");
    }

    // std::io::Result を利用する
    println!("call sub function.");
    sub( "sample5.txt" ) ;
}

// 戻り値に std::io::Result を利用すると、
// .except 部分を ? を使って短く書ける
fn sub( path: &str ) -> std::io::Result<()> {
    let mut file = File::create( path )? ;
    file.write( b"hello rust world.\n")? ;
    Ok(())
}


