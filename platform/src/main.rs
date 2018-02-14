
#[cfg(unix)]
mod platform_linux;
#[cfg(unix)]
use platform_linux as platform;

#[cfg(windows)]
mod platform_windows;
#[cfg(windows)]
use platform_windows as platform;

fn main() {
    println!("Hi, your platform is: [{}]", platform::say_hello());
}
