

extern crate my;
extern crate errloc_macros;

fn stop_panic() {
    let res = std::panic::catch_unwind(|| {
        my::start_panic();
    });
    assert!(res.is_err());
    res.unwrap_or_else(|e| {
        println!("{:?}", errloc_macros::msg(&e));
    });
}

#[test]
fn test_panic_closure() {
    // main thread
    stop_panic();
    // spawned thread
    let th = std::thread::spawn(move || {
        stop_panic();
    });
    let res = th.join();
    assert!(res.is_ok());
}

#[test]
fn test_panic_thread() {
    let th = std::thread::spawn(move || {
        my::start_panic();
    });
    let res = th.join();
    assert!(res.is_err());
    res.unwrap_or_else(|e| {
        println!("{:?}", errloc_macros::msg(&e));
    });
}
