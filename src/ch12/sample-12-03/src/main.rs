use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        // 標準入力をそのまま書き出す
        let reader = BufReader::new(std::io::stdin()) ;
        do_print( reader );
    } else {
        // ファイル指定の場合は、ファイルをオープンする
        let file = File::open( &args[1] ) 
            .expect("file not found.");
        let reader = BufReader::new(file) ;
        do_print( reader );
    }
}

fn do_print<T>( reader: BufReader<T> ) where T: std::io::Read {
    let mut writer = BufWriter::new(std::io::stdout());
    for i in reader.bytes() {
        if let Ok(n) = i {
            writer.write( &[n] );
        }
    }
}
