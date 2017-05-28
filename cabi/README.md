Exposing C ABI
--------------

Build shared lib:

    cargo build

Run C client example:

    gcc -g -O0 client.c ./target/debug/libmy.so
    ./a.out

Run C example with memcheck:

    valgrind --leak-check=yes --show-reachable=yes --track-origins=yes ./a.out

Run Python client example:

    python client.py
