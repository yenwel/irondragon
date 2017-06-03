extern crate robots;

use robots::actors::{Actor,ActorCell};
use std::sync::Mutex;
use std::any::Any;
use std::thread;
use std::time;
use super::super::gpioaccess::{DirectionProxied,PinProxyContract,PwmProxyContract};
use super::super::gpioaccess::proxyimpl::{PinProxy,PwmProxy};


#[derive(Copy, Clone, PartialEq,Debug)]
pub enum PinCommands {
	Blink(u64),
	Switch,
}

pub struct PinActor {
	pinproxy :  Mutex<PinProxy>,
}

impl Actor for PinActor {
	fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<PinCommands>(_message){
			match *_message {
				PinCommands::Blink(times) => {
					let pin = self.pinproxy.lock().unwrap();
					match pin.export() {
						Ok(()) => {
							println!("Pin exported");
							for _ in 1..times {
								pin.set_direction(DirectionProxied::Out);
								pin.set_direction(DirectionProxied::High);
								thread::sleep(time::Duration::from_millis(400));
								pin.set_direction(DirectionProxied::Low);
								thread::sleep(time::Duration::from_millis(400));
								println!("Blink");
							}
							match pin.unexport() {
								Ok(()) => {
									println!("Pin unexported");
								}
								_ => {}
							}
						}
						_ => {}
					}
				},
				PinCommands::Switch => { }
			}
		}
	}
}


impl PinActor{
	pub fn new(pin_number: u64) -> PinActor {
		PinActor{ pinproxy : Mutex::new(PinProxy::new(pin_number)) }
	}
}


#[derive(Copy, Clone, PartialEq,Debug)]
pub enum PwmCommands {
	MoveToDegree(u16)
}

pub struct PwmActor {
	pwmproxy :  Result<Mutex<PwmProxy>,()>,
}

impl Actor for PwmActor {
	fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		println!("pwm received message");
		if let Ok(_message) = Box::<Any>::downcast::<PwmCommands>(_message){
			println!("pwm recognized command");
			match *_message {
				PwmCommands::MoveToDegree(degree) => {
					println!("pwm movetodegree");
					match self.pwmproxy {
						Ok(ref pwmproxyresult) =>
						{
							let pwm = pwmproxyresult.lock().unwrap();
							match pwm.export() {
								Ok(()) => {
									println!("Pwm exported");
									match pwm.set_period_ns(1_500_000)
									{
										Ok(()) => {
											println!("Pwm setting period");
											match pwm.enable(true)
											{
												Ok(()) => {
													println!("Pwm enable");
													for x in 1..10 {
														println!("Pwm {}",x);
														pwm.increase_to_max(0.05);
														pwm.decrease_to_minimum(0.05);
													}
													match pwm.enable(false)
													{
														Ok(()) => {
															println!("Pwm disable");
														}
														_ => println!("Pwm disabled failed")
													}
												}
												_ => println!("Pwm enabled failed")
											}
										}
										_ => println!("Pwm set period failed")
									}
									match pwm.unexport() {
										Ok(()) => {
											println!("Pwm unexported");
										}
										_ => println!("Pwm unexport failed")
									}
								}
								_ => println!("Pwm export failed")
							}
						}
						_ => println!("No Pwm")
					}
					
					println!("pwm done");
				}
			}
		}
	}
}

impl PwmActor{
	pub fn new(number: u32) -> PwmActor {
		match PwmProxy::new(0,number)
		{
			Ok(pwmproxy) =>  PwmActor{ pwmproxy : Ok(Mutex::new(pwmproxy)) },
			_ => PwmActor{ pwmproxy : Err(()) },
		}
	}
}