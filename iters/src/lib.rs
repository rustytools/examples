

pub struct FibIter {
    prev: u32,
    cur: u32
}

impl FibIter {
    pub fn new() -> Self {
        FibIter { prev: 1, cur: 1 }
    }
}

impl std::iter::Iterator for FibIter {
   type Item = u32;

   fn next(&mut self) -> std::option::Option<u32> {
        let res = self.cur;
        self.cur = self.prev + self.cur;
        self.prev = res;
        Some(res)
    }
}
