
#[macro_use]
extern crate errloc_macros;

extern crate my;


#[test]
fn test_tls() {
    let mut vec: std::vec::Vec<std::thread::JoinHandle<u32>> = std::vec::Vec::new();
    for i in 0..8 {
        let th = std::thread::spawn(move || -> u32 {
            my::Worker::new(i).do_work();
            i
        });
        vec.push(th);
    }
    while let Some(th) = vec.pop() {
        th.join().expect(errloc!());
    } 
}
