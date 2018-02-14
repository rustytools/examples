
const PATH: Option<&'static str> = option_env!("PATH");
const FOO: Option<&'static str> = option_env!("FOO");

fn get_compile_time_var(name: &str) -> &'static str {
    match name {
        "PATH" => match PATH {
            Some(val) => val,
            None => "DEFAULT_PATH"
        }
        "FOO" => match FOO {
            Some(val) => val,
            None => "DEFAULT_FOO"
        }
        _ => ""
    }
}

fn main() {
    println!("PATH: [{}]", get_compile_time_var("PATH"));
    println!("FOO: [{}]", get_compile_time_var("FOO"));
}
