use crate::{HelloWorld, HelloWorldInstance};
use ontio_std::mock::build_runtime;

#[test]
fn initialize() {
    let hw = HelloWorldInstance;
//    assert_eq!(hw.hello(), "hello world");

    assert_eq!(hw.hello2("world"), "hello world");

    assert_eq!(hw.save("key", "value"), true);

    assert_eq!(hw.get("key"), "value");
}


#[test]
fn test_hello() {
    use ontio_std::abi::Sink;
    let mut sink = Sink::new(16);
    sink.write("hello world");

    assert_eq!("..", format!("{:?}", sink.into()));
}