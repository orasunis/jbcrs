//! Reads and prints the content of a class file to STDOUT.
extern crate jbcrs_basic;

use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    // get the file path
    let file_path = env::args().nth(1).expect("no file path provided");

    // first read the class file
    let mut file = File::open(file_path).expect("could not open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("could not read file");

    // then parse it
    let (pool, class) = jbcrs_basic::parse(buf.as_ref()).expect("could not parse class file");

    // now print it to stdout
    // the constant pool will be printed differently
    // to help the user get a constant pool item by index

    println!("Constant Pool:");
    for (index, item) in pool.iter() {
        println!("    #{}: {:?}", index, item);
    }

    // Class derives Debug,
    // the output might not be the best (compared to javap or krakatau-disassemble)
    // but it should be enough.
    println!("{:#?}", class);
}
