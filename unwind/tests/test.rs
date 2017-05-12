
extern crate my;

use std::ops::DerefMut;

fn stringify_panic_info(info: &std::panic::PanicInfo) -> std::string::String {
    let (file, line) = match info.location() {
        Some(lc) => (lc.file(), lc.line()),
        None => ("unknown", 0),
    };
    let msg: &str = match info.payload().downcast_ref::<&str>() {
        Some(st) => st,
        None => {
            match info.payload().downcast_ref::<String>() {
                Some(stw) => stw,
                None => "",
            }
        },
    };
    let th = std::thread::current();
    let thname: &str = th.name().unwrap_or("");
    return String::from("Panic occurred:") +
            " thread: [" + thname + "]," +
            " location: [" + file + ":" + line.to_string().as_str() + "]," + 
            " payload: [" + msg + "]";
}

thread_local!(static LOCAL_EXC: std::cell::RefCell<std::string::String> = std::cell::RefCell::new("".to_string()));


#[test]
fn test_panic() {
    let mx = std::sync::Mutex::new(0);
	std::panic::set_hook(Box::new(move |info: &std::panic::PanicInfo| {
        let guard = mx.lock();
        if guard.is_err() {
            println!("Panic handler fatal error, exiting");
            std::process::exit(1);
        }
        let excmsg: std::string::String = stringify_panic_info(info);
        LOCAL_EXC.with(|cell| {
            let mut rm: std::cell::RefMut<std::string::String> = cell.borrow_mut();
            let target: &mut std::string::String = rm.deref_mut();
            if target.len() > 0 {
                target.push('\n');
            }
            target.push_str(excmsg.as_str());
        });
    }));

    println!("1");
    let res = std::panic::catch_unwind(|| {
        my::start_panic();
    });
    println!("2");
    match res {
        Ok(v) => println!("Okay: {:?}", v),
        Err(_) => {
            LOCAL_EXC.with(|cell| {
                println!("Error: {:?}", *cell.borrow());
            });
        }
    }
}
