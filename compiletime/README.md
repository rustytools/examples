Capturing compile-time options
------------------------------

Run with:

    cargo run
    export FOO=bar
    cargo run
    cargo clean
    cargo run
    unset FOO
    cargo run
    cargo clean
    cargo run
