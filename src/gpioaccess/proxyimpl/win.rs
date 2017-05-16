use super::{PinProxyContract,PwmProxyContract,DirectionProxied,ProxyError,ProxyResult};

pub struct PinProxy {
	pin_num: u64,
}

impl PinProxyContract for PinProxy
{
	fn new(pin_num: u64) -> PinProxy {
		PinProxy{ pin_num: pin_num }
	}
	fn export(&self) -> ProxyResult<()>
	{
		println!("exporting pin {}",self.pin_num);
		Ok::<(),ProxyError>(())
	}
	fn unexport(&self) -> ProxyResult<()>
	{
		println!("unexporting pin {}",self.pin_num);
		Ok::<(),ProxyError>(())
	}

	fn set_value(&self, value: u8) -> ProxyResult<()>
	{
		println!("setting value pin {}",value);
		Ok::<(),ProxyError>(())
	}

	fn set_direction(&self, dir: DirectionProxied) -> ProxyResult<()>
	{
		println!("setting direction pin");
		Ok::<(),ProxyError>(())
	}
}

pub struct PwmProxy {
	chip : u32,
	number : u32
}

impl PwmProxyContract for PwmProxy
{
	fn new(chip: u32, number: u32) -> ProxyResult<PwmProxy> {
		Ok(PwmProxy{ chip : chip, number : number })
	}

	fn export(&self) ->  ProxyResult<()>{
		Ok::<(),ProxyError>(())
	}

	fn unexport(&self) ->  ProxyResult<()> {
		Ok::<(),ProxyError>(())
	}
	
	fn enable(&self, enable: bool) -> ProxyResult<()>
	{
		Ok::<(),ProxyError>(())
	}

	fn set_duty_cycle_ns(&self, duty_cycle_ns: u32) -> ProxyResult<()>
	{
		Ok::<(),ProxyError>(())
	}

	fn set_period_ns(&self, period_ns: u32) -> ProxyResult<()>
	{
		Ok::<(),ProxyError>(())
	}
	
	fn get_period_ns(&self) -> ProxyResult<u32>
	{
		Ok::<u32,ProxyError>(1)
		
	}
	
	fn increase_to_max(&self, duration_ms: u32, update_period_ms: u32) {}

	fn decrease_to_minimum(&self, duration_ms: u32, update_period_ms: u32) {}
}