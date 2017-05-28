Mutex usage example
-------------------

In this example an object, protected by mutex, is shared between multiple
threads that can modify the value of this object.

Run with:

    cargo test -- --nocapture
