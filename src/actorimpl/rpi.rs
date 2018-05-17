extern crate robots;

use robots::actors::{Actor, ActorCell};
use std::sync::Mutex;
use std::any::Any;
use std::thread;
use std::time;
use super::super::gpioaccess::{DirectionProxied, PinProxyContract, PwmProxyContract, ProxyError};
use super::super::gpioaccess::proxyimpl::{PinProxy, PwmProxy};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PinCommands {
    Blink(u64),
    Switch,
}

pub struct PinActor {
    pinproxy: Mutex<PinProxy>,
}

impl Actor for PinActor {
    fn receive(&self, _message: Box<Any>, _context: ActorCell) {
        if let Ok(_message) = Box::<Any>::downcast::<PinCommands>(_message) {
            match *_message {
                PinCommands::Blink(times) => {
                    match Self::blink(self, times) {
                        Ok(()) => {}
                        _ => {}
                    }
                }
                PinCommands::Switch => {}
            }
        }
    }
}

   

impl PinActor {
    pub fn new(pin_number: u64) -> PinActor {
        PinActor {
            pinproxy: Mutex::new(PinProxy::new(pin_number)),
        }
    }

     fn blink(&self, times : u64) -> Result<(),ProxyError> {
        let pin = self.pinproxy.lock().unwrap();
        match pin.export() {
            Ok(()) => {
                println!("Pin exported");
                for _ in 1..times {
                    try!(pin.set_direction(DirectionProxied::Out));
                    try!(pin.set_direction(DirectionProxied::High));
                    thread::sleep(time::Duration::from_millis(400));
                    try!(pin.set_direction(DirectionProxied::Low));
                    thread::sleep(time::Duration::from_millis(400));
                    println!("Blink");
                }
                match pin.unexport() {
                    Ok(()) => {
                        println!("Pin unexported");
                        Ok(())
                    }
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PwmCommands {
    MoveToDegree(u16),
}

pub struct PwmActor {
    pwmproxy: Result<Mutex<PwmProxy>, ()>,
}

impl Actor for PwmActor {

    fn receive(&self, _message: Box<Any>, _context: ActorCell) {
        println!("pwm received message");
        if let Ok(_message) = Box::<Any>::downcast::<PwmCommands>(_message) {
            println!("pwm recognized command");
            match *_message {
                PwmCommands::MoveToDegree(_degree) => {
                    match Self::movetodegree(self) {
                        Ok(()) => {}
                        _ => {}
                    }
                }
            }
        }
    }
}

impl PwmActor {
    pub fn new(number: u32) -> PwmActor {
        match PwmProxy::new(0, number) {
            Ok(pwmproxy) => PwmActor {
                pwmproxy: Ok(Mutex::new(pwmproxy)),
            },
            _ => PwmActor { pwmproxy: Err(()) },
        }
    }

    pub fn movetodegree(&self) -> Result<(),ProxyError> {
        println!("pwm movetodegree");
        match self.pwmproxy {
            Ok(ref pwmproxyresult) => {
                let pwm = pwmproxyresult.lock().unwrap();
                try!(pwm.export());
                println!("Pwm exported");
                try!(pwm.set_period_ns(2_500_000));
                println!("Pwm setting period");
                try!(pwm.enable(true));
                println!("Pwm enable");
                for x in 1..10 {
                    println!("Pwm {}", x);
                    pwm.increase_to_max(0.25);
                    pwm.decrease_to_minimum(0.25);
                }
                try!(pwm.enable(false));
                println!("Pwm disable");
                try!(pwm.unexport());
                println!("pwm done");
                Ok(())
            }
            _ => Err(ProxyError::MonErreur)
        }
    }
}
