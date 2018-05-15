#[test]
fn it_works() {
    assert_eq!(1, 0 + 1);
}

#[cfg(target_os = "linux")]
pub mod linuxsysfs;
