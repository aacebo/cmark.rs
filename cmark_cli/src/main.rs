use std::fs;

fn main() {
    for path in std::env::args().skip(1) {
        let data = match fs::read(path) {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        };

        println!("{}", String::from_utf8(data).unwrap());
    }
}
