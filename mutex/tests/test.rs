
#[macro_use]
extern crate errloc_macros;

extern crate my;


#[test]
fn test_tls() {
    let mx = std::sync::Arc::new(std::sync::Mutex::new(std::cell::Cell::new(std::string::String::from(""))));
    let mut vec: std::vec::Vec<std::thread::JoinHandle<u32>> = std::vec::Vec::new();
    for i in 0..32 {
        let mxpass = mx.clone();
        let th = std::thread::spawn(move || -> u32 {
            my::Worker::new(mxpass, i).do_work();
            i
        });
        vec.push(th);
    }
    while let Some(th) = vec.pop() {
        th.join().expect(errloc!());
    } 
}
