
extern crate sysfs_gpio;
extern crate sysfs_pwm;

use self::sysfs_gpio::{Direction, Pin};
use self::sysfs_pwm::{Pwm};
use ::std::error::Error;

use super::{PinProxyContract,PwmProxyContract,DirectionProxied,ProxyError,ProxyResult};

pub struct PinProxy {
	pin : Pin,
}

impl PinProxyContract for PinProxy
{
	fn new(pin_num: u64) -> PinProxy {
		PinProxy{ pin: Pin::new(pin_num) }
	}

	fn export(&self) ->  ProxyResult<()>{
		match self.pin.export()
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}

	fn unexport(&self) ->  ProxyResult<()> {
		match self.pin.unexport()
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}

	fn set_value(&self, value: u8) ->  ProxyResult<()> {
		match self.pin.set_value(value)
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}

	fn set_direction(&self, dir: DirectionProxied) ->  ProxyResult<()>{
		let dirmapped =  match dir {
										DirectionProxied::In =>  Direction::In,
										DirectionProxied::Out => Direction::Out,
										DirectionProxied::High => Direction::High,
										DirectionProxied::Low => Direction::Low,
									};
		match self.pin.set_direction(dirmapped)
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}
}

pub struct PwmProxy {
	pwm : Pwm,
}

impl PwmProxyContract for PwmProxy
{
	fn new(chip: u32, number: u32) -> ProxyResult<PwmProxy>{
		
		match Pwm::new(chip, number) {
			Ok(pwm) => Ok(PwmProxy{ pwm: pwm}),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}

	fn export(&self) ->  ProxyResult<()>{
		match self.pwm.export()
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}

	fn unexport(&self) ->  ProxyResult<()> {
		match self.pwm.unexport()
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}
	
	fn enable(&self, enable: bool) -> ProxyResult<()>
	{
		match self.pwm.enable(enable)
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{} caused by {}",result.description(),result.cause().unwrap());
				Err(ProxyError::MonErreur)},
		}
	}

	fn set_duty_cycle_ns(&self, duty_cycle_ns: u32) -> ProxyResult<()>
	{
		match self.pwm.set_duty_cycle_ns(duty_cycle_ns)
		{
			Ok(()) => {println!("setting dutycycle {}",duty_cycle_ns); Ok(())},
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}

	fn set_period_ns(&self, period_ns: u32) -> ProxyResult<()>
	{
		match self.pwm.set_period_ns(period_ns)
		{
			Ok(()) => Ok(()),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
	}
	
	fn get_period_ns(&self) -> ProxyResult<u32>
	{
		match self.pwm.get_period_ns()
		{
			Ok(result) => Ok(result),
			Err(result) => {println!("{}",result.description());
				Err(ProxyError::MonErreur)},
		}
		
	}
	
	fn increase_to_max(&self, duration_ms: u32, update_period_ms: u32)
	{
			let step: f32 = duration_ms as f32 / update_period_ms as f32;
			println!("step {}",step);
			let mut duty_cycle = 0.0;
			match self.pwm.get_period_ns()
			{
				Ok(period_ns) => {
				println!("period {}",period_ns);
					while duty_cycle < 1.0 {
						println!("duty cycle {}",duty_cycle);						
						println!("set duty cycle ns{}",(duty_cycle * period_ns as f32) as u32);
						self.pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32).unwrap();
						duty_cycle += step;
					}
					self.pwm.set_duty_cycle_ns(period_ns).unwrap()
				}
				Err(result) => {println!("{}",result.description())}
			}
	}
	
	fn decrease_to_minimum(&self, duration_ms: u32, update_period_ms: u32)
	{
		let step: f32 = duration_ms as f32 / update_period_ms as f32;
		println!("step {}",step);
		let mut duty_cycle = 1.0;
		match self.pwm.get_period_ns()
		{
			Ok(period_ns) => {
			println!("period {}",period_ns);
				while duty_cycle > 0.0 {
					println!("duty cycle {}",duty_cycle);
					println!("set duty cycle ns{}",(duty_cycle * period_ns as f32) as u32);
						self.pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32).unwrap();
						duty_cycle -= step;
				}
				self.pwm.set_duty_cycle_ns(0).unwrap()
			}
			Err(result) => {println!("{}",result.description())}
		}
	}
}