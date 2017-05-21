
#[macro_use]
extern crate errloc_macros;

pub fn listdir(dir: &str) {
    let paths = std::fs::read_dir(dir).expect(errloc!());
    for pa in paths {
        let direntry = pa.expect(errloc!());
        println!("{}", direntry.path().display());
    }
}

pub fn catfile(path: &str) {
    println!("cat {}", path);
    use std::io::BufRead;
    let fi = std::fs::File::open(path).expect(errloc!());
    let buf = std::io::BufReader::new(fi);
    for (i, li) in buf.lines().enumerate() {
        println!("{}: {}", i, li.expect(errloc!()));
    }
}
