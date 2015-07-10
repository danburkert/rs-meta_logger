#![allow(dead_code)]

#[macro_use] extern crate log;
#[macro_use] extern crate scoped_log;
extern crate env_logger;

fn main () {
    let _ = env_logger::init();

    scoped_info!("1");
    {
        push_log_scope!("outer-scope");
        scoped_info!("2: {}", "some args");
        push_log_scope!("inner-scope");
        scoped_info!(target: "some-target", "2: {}", "some args");
        Foo.foo();
    }
    scoped_info!("4");
}

// This function is not called, but by defining it the scoped_assert! macro is
// type checked.
fn fail() {
    scoped_assert!(false);
    scoped_assert!(false, "I failed!");
}

#[derive(Debug)]
struct Foo;

impl Foo {
    fn foo(&self) {
        push_log_scope!("{:?}-scope", self);
        scoped_info!("3");
    }
}
