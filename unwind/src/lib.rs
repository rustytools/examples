
#[macro_use]
extern crate errloc_macros;

pub fn start_panic() {
    panic!(errlocm!("Forty two"));
}
