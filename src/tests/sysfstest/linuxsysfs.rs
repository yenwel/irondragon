extern crate sysfs_gpio;
extern crate sysfs_pwm;

use self::sysfs_pwm::Pwm;

#[test]
fn it_works() {
    assert_eq!(1, 0 + 1);
}

//http://www.jumpnowtek.com/rpi/Using-the-Raspberry-Pi-Hardware-PWM-timers.html
//https://github.com/rust-embedded/rust-sysfs-pwm/blob/master/examples/breathe.rs
#[test]
#[ignore]
fn test_pwm() {
    let pwm = Pwm::new(0, 0).unwrap();
    assert!(pwm.export().is_ok());
    assert!(pwm.enable(true).is_ok());
    assert!(pwm.set_period_ns(10000000).is_ok());
    assert!(pwm.set_duty_cycle_ns(8000000).is_ok());
    assert!(pwm.enable(false).is_ok());
    assert!(pwm.unexport().is_ok());
}
