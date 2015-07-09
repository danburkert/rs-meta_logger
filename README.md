# Hierachical logger

Simple wrapper for the log crate that enables adding arbitrary information at
runtime for future calls to the debugging macros.

That magic behind this is a `thread_local!` `RefCell<Vec<&'static str>>` which
when populated will include the included identifiers, seperated with ": ".

```rust
#[macro_use] extern crate hierachical_log;
#[macro_use] extern crate log;
extern crate env_logger;

fn main () {
    env_logger::init().unwrap();

    meta_info!("1");
    {
        register_logger_info!("Test");
        meta_info!("2");
        register_logger_info!("Testing");
        foo();
    }
    meta_info!("4");
}

fn foo() {
    meta_info!("3");
}
```

Output sample:

```
INFO:main: 1
INFO:main: Test: 2
INFO:main: Test: Testing: 3
INFO:main: 4
```
