#[macro_use] extern crate hierachical_log;
extern crate env_logger;

fn main () {
    env_logger::init().unwrap();

    info!("1");
    {
        register_logger_info!("Test");
        info!("2");
        register_logger_info!("Testing");
        Foo.foo();
    }
    info!("4");
}

#[derive(Debug)]
struct Foo;

impl Foo {
    fn foo(&self) {
        register_logger_info!("{:?}", self);
        info!("3");
    }
}
