use crate::{ApiTest, ApiTestInstance};
use ontio_std::mock::build_runtime;

#[test]
fn initialize() {
    let mut api = ApiTestInstance;
    assert_eq!(api.timestamp(), 0);

    assert_eq!(api.blockheight(), 0);

    assert_ne!(api.selfaddress(), "");
}