use std::error::Error;

use crate::config::Config;
use crate::connect::gas::connect_gas_sensor;
use crate::rf::RFClient;

pub fn run(rf: &mut Box<dyn RFClient>, config: &mut Config) -> Result<(), Box<dyn Error>> {
    let (_meta, _date) = connect_gas_sensor(rf, config.frisquet()?)?;

    Ok(())
}
