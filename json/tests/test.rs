
#[macro_use]
extern crate errloc_macros;

extern crate my;
extern crate serde_json;

#[test]
fn test_serialize() {
    let foo = my::Foo::new(42, "foo".to_string());
    let bar = my::Bar::new(foo, "bar".to_string());
    let json = serde_json::to_string_pretty(&bar).expect(errloc!());
    println!("{}", &json);
}

#[test]
fn test_deserialize() {
    let st = r#"{
		"foo": {
			"foo1": 42,
			"foo2": "foo"
		},
		"boo": "bar"
    }"#;
    let bar: my::Bar = serde_json::from_str(st).expect(errloc!());
    println!("{:?}", &bar);
}
