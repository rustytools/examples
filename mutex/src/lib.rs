
#[macro_use]
extern crate errloc_macros;

//#[derive(Debug)]
pub struct Worker {
    mx: std::sync::Arc<std::sync::Mutex<std::string::String>>,
    num: u32,
}

impl Worker {
    pub fn new(mx: std::sync::Arc<std::sync::Mutex<std::string::String>>,
               num: u32) -> Worker {
        Worker { mx, num }
    }

    pub fn do_work(&self) {
        // guard here is used to ensure that first and second
        // printed messages won't be intermingled
        let mut st = self.mx.lock().expect(errloc!());
        println!("{} before: {}", self.num, *st);
        st.truncate(0);
        st.push_str(self.num.to_string().as_str());
        println!("{} after: {}", self.num, *st);
    }
}
