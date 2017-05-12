
pub fn start_panic() {
    let mut vec: std::vec::Vec<std::thread::JoinHandle<i32>> = std::vec::Vec::new();
    for i in 0..1 {
        let th = std::thread::spawn(move || -> i32 {
            let msg = std::string::String::from("ERROR_MESSAGE") + i.to_string().as_str();
            panic!(msg);
        });
        vec.push(th);
    }
    while let Some(th) = vec.pop() {
        th.join().expect("it will fail");
    }
}
