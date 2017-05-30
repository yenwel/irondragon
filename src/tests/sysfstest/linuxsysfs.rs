extern crate sysfs_gpio;
extern crate sysfs_pwm;

use self::sysfs_gpio::{Direction, Pin};
use self::sysfs_pwm::{Pwm};
use ::std::error::Error;

#[test]
fn it_works() {
    assert_eq!(1, 0+1);
}

//http://www.jumpnowtek.com/rpi/Using-the-Raspberry-Pi-Hardware-PWM-timers.html
//https://github.com/rust-embedded/rust-sysfs-pwm/blob/master/examples/breathe.rs
#[test]
#[ignore]
fn test_pwm() {
    let pwm = Pwm::new(0, 0).unwrap();
    assert!(pwm.export().unwrap().is_ok());
    assert!(pwm.enable(true).unwrap().is_ok());
    assert!(pwm.set_period_ns(10000000).unwrap().is_ok());
    assert!(pwm.set_duty_cycle_ns(8000000).unwrap().is_ok());
    assert!(pwm.enable(false).unwrap().is_ok());
    assert!(pwm.unexport().unwrap().is_ok());
}