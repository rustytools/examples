
extern crate argparse;

fn main() {
	let mut verbose = false;
    let mut name = std::string::String::from("World");
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose).add_option(&["-v", "--verbose"], argparse::StoreTrue, "Be verbose");
        ap.refer(&mut name).add_option(&["--name"], argparse::Store, "Name for the greeting");
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("log: specified name is: [{}]", name);
    }
    println!("Hello {}!", name);
}
