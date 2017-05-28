
pub struct MySource {
    count: u32
}

impl MySource {
    pub fn new() -> Self {
        return Self { count: 0 }
    }
}

impl std::io::Read for MySource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.count < 42 {
            let mut st = std::string::String::new();
            self.count += 1;
            st.push_str(self.count.to_string().as_str());
            self.count += 1;
            st.push_str(self.count.to_string().as_str());
            let vec = st.into_bytes();
            let len = std::cmp::min(buf.len(), vec.len());
            // cannot borrow mutably
            //buf.write_all(&vec).expect(errloc!());
            unsafe {
                std::ptr::copy(vec.as_ptr(), buf.as_mut_ptr(), len);
            }
            Ok(len)
        } else {
            Ok(0)
        }
    }
}
