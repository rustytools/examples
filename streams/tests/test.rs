
#[macro_use]
extern crate errloc_macros;

extern crate my;

#[test]
fn test() {
    use std::io::Read;
    let mut src = my::MySource::new();
    let mut sink = std::vec::Vec::new();
    src.read_to_end(&mut sink).expect(errloc!());
    let st = std::string::String::from_utf8(sink).expect(errloc!());
    println!("[{}]", st);
}
