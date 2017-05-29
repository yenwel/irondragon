use gpioaccess::{DirectionProxied,PinProxyContract,PwmProxyContract};
use gpioaccess::proxyimpl::{PinProxy,PwmProxy};

#[test]
fn it_works() {
    assert_eq!(1, 0+1);
}

#[cfg(unix)]
pub mod unixsysfs;
