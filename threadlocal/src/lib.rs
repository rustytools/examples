
thread_local!(static THREAD_LOCAL_MESSAGE: std::cell::Cell<std::string::String> = std::cell::Cell::new("".to_string()));


#[derive(Debug)]
pub struct Worker {
    num: u32,
}

impl Worker {
    pub fn new(num: u32) -> Worker {
        Worker { num: num }
    }

    pub fn do_work(&self) {
        THREAD_LOCAL_MESSAGE.with(|cell| {
            let before = cell.take();
            println!("{} before: {}", self.num, before);
            cell.set(self.num.to_string());
            let after = cell.take();
            println!("{} after: {}", self.num, after);
            cell.set(after);
        });
    }
}
