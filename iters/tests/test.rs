
extern crate my;

#[test]
fn test_iter() {
    let it = my::FibIter::new();
    for num in it.take(11) {
        println!("{:?}", num);
    }
}

#[test]
fn test_transform() {
    let it = my::FibIter::new();
    let vec = it.take(11)
        .filter(|num| { 0 == num % 2 })
        .map(|num| { (num/2).to_string() })
        .collect::<std::vec::Vec<std::string::String>>();
    println!("{:?}", vec);

}

