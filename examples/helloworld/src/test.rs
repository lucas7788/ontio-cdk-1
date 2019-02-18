use crate::{HelloWorld, HelloWorldInstance};
use ontio_std::mock::build_runtime;

#[test]
fn initialize() {
    let hw = HelloWorldInstance;
    assert_eq!(hw.hello(), "hello world");
}


#[test]
fn test_hello() {
    use ontio_std::abi::Sink;
    let mut sink = Sink::new(16);
    sink.write("hello world");

    assert_eq!("..", format!("{:?}", sink.into()));
}