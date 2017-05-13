
#[macro_use]
extern crate errloc_macros;

//#[derive(Debug)]
pub struct Worker {
    mx: std::sync::Arc<std::sync::Mutex<std::cell::Cell<std::string::String>>>,
    num: u32,
}

impl Worker {
    pub fn new(mx: std::sync::Arc<std::sync::Mutex<std::cell::Cell<std::string::String>>>,
               num: u32) -> Worker {
        Worker { mx, num }
    }

    pub fn do_work(&self) {
        // guard here is used to ensure that first and second
        // printed messages won't be intermingled
        let cell = self.mx.lock().expect(errloc!());
        let before = cell.take();
        println!("{} before: {}", self.num, before);
        cell.set(self.num.to_string());
        let after = cell.take();
        println!("{} after: {}", self.num, after);
        cell.set(after);
    }
}
