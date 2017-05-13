
pub fn choose_first<'a>(first: &'a str, _second: &'a str) -> &'a str {
    return first;
}

pub fn choose_second<'a, 'b>(_first: &'a str, second: &'b str) -> &'b str {
    return second;
}

pub struct Foo<'a> {
    val: &'a str,
}

impl<'a> Foo<'a> {
    pub fn new(val: &'a str) -> Foo<'a> {
        Foo { val: val }
    }

    pub fn print(&self) {
        println!("{}", self.val);
    }
}
