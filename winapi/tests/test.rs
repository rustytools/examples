
//#[macro_use]
extern crate errloc_macros;

extern crate my;


#[test]
fn test_winapi() {
    std::panic::catch_unwind(|| {
        my::callwin("Hello Windoze!");
    }).unwrap_or_else(|e| {
        println!("Fail: {:?}", errloc_macros::msg(&e));
    });
}
