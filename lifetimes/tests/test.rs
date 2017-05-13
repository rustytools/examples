
extern crate my;

#[test]
fn test_function() {
    let second1 = "bar1";
    let second2 = "bar2";
    let chosen2;
    {
        let fval = "foo".to_string();
        let first = fval.as_str();
        let chosen1 = my::choose_first(first, second1);
        chosen2 = my::choose_second(first, second2);
        println!("{}", chosen1);
    }
    println!("{}", chosen2);
}

#[test]
fn test_struct() {
    let wst = "bar".to_string();
    let mut foo = my::Foo::new("foo");
    foo.print();
    //let wst = "bar".to_string();
    {
        //let wst = "bar".to_string();
        let st = wst.as_str();
        foo = my::Foo::new(st);
        foo.print();
    }
    foo.print();
}
