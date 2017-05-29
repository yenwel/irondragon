extern crate sysfs_gpio;
extern crate sysfs_pwm;

use self::sysfs_gpio::{Direction, Pin};
use self::sysfs_pwm::{Pwm};
use ::std::error::Error;

#[test]
fn it_works() {
    assert_eq!(1, 0+1);
}
