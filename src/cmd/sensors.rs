use std::error::Error;

use crate::config::Config;
use crate::connect::gas::connect_gas_sensor;
use crate::connect::sensors::connect_sensors;
use crate::output::ReportingClient;
use crate::rf::{mqtt, RFClient};

pub fn run(rf: &mut Box<dyn RFClient>, config: &mut Config) -> Result<(), Box<dyn Error>> {
    let (_meta, _sensor) = connect_sensors(rf, config.frisquet()?)?;

    let mqtt_config = config.mqtt.as_mut().ok_or("no mqtt config")?;
    let mut mqtt_client = mqtt::new(&mqtt_config)?;
    mqtt_client.forward(_meta, "sensors", _sensor)?;

    let (_gas_meta, _gas_sensor) = connect_gas_sensor(rf, config.frisquet()?)?;
    mqtt_client.forward(_gas_meta, "gas_sensors", _gas_sensor)?;

    return Ok(());
}
