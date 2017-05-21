

//#[macro_use]
extern crate errloc_macros;

extern crate my;

#[test]
fn test_fs() {
    std::panic::catch_unwind(|| {
        my::listdir(".");
        my::catfile("Cargo.toml");
    }).unwrap_or_else(|e| {
        println!("Fail: {:?}", errloc_macros::msg(&e));
    });

}
