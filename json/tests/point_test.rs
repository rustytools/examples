
extern crate serde_json;
extern crate my;

#[test]
fn test1() {
    let point = my::point::Point::new(42, 43);
    let json = serde_json::to_string(&point).expect("Serialization fail");
    println!("{}", &json);
}
