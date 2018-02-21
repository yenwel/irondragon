use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectionProxied {
    In,
    Out,
    High,
    Low,
}

#[derive(Debug)]
pub enum ProxyError {
    MonErreur,
}

impl ::std::error::Error for ProxyError {
    fn description(&self) -> &str {
        "an error"
    }
}

impl Display for ProxyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "an error")
    }
}

pub type ProxyResult<T> = ::std::result::Result<T, ProxyError>;

//depend on abstractions not concretions lulz
pub trait PinProxyContract {
    fn new(pin_num: u64) -> Self;
    //FIXME :figure out mapping overloaded Result from gpio library
    fn export(&self) -> ProxyResult<()>;

    fn unexport(&self) -> ProxyResult<()>;

    fn set_value(&self, value: u8) -> ProxyResult<()>;

    fn set_direction(&self, dir: DirectionProxied) -> ProxyResult<()>;
}

pub trait PwmProxyContract {
    fn new(chip: u32, number: u32) -> ProxyResult<Self>
    where
        Self: Sized;
    //FIXME :figure out mapping overloaded Result from gpio library
    fn export(&self) -> ProxyResult<()>;

    fn unexport(&self) -> ProxyResult<()>;

    fn enable(&self, enable: bool) -> ProxyResult<()>;

    fn set_duty_cycle_ns(&self, duty_cycle_ns: u32) -> ProxyResult<()>;

    fn set_period_ns(&self, period_ns: u32) -> ProxyResult<()>;

    fn get_period_ns(&self) -> ProxyResult<u32>;

    fn increase_to_max(&self, step: f32);

    fn decrease_to_minimum(&self, step: f32);
}

#[cfg(target_os = "linux")]
#[path = "proxyimpl/linux.rs"]
pub mod proxyimpl;
#[cfg(not(target_os = "linux"))]
#[path = "proxyimpl/nonlinux.rs"]
pub mod proxyimpl;
