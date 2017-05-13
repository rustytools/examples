
#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize, Debug)]
pub struct Foo {
    foo1: i32,
    foo2: std::string::String,
}

impl Foo {
    pub fn new(foo1: i32, foo2: std::string::String) -> Foo {
        Foo { foo1, foo2 }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Bar {
    foo: Foo,
    boo: std::string::String,
}

impl Bar {
    pub fn new(foo: Foo, boo: std::string::String) -> Bar {
        Bar { foo, boo }
    }
}
