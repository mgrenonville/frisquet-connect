use std::error::Error;

use paho_mqtt::Message;
use serde::Serialize;

use crate::connect::Metadata;
use crate::output::ReportingClient;
use crate::rf::mqtt::MqttClient;
use crate::rf::SendError;

#[derive(Serialize)]
pub struct Report<T: Serialize> {
    from: u8,
    to: u8,
    data: T,
}

impl ReportingClient for MqttClient {
    fn forward<T: Serialize>(&mut self, metadata: Metadata, data: T) -> Result<(), Box<dyn Error>> {
        let value = Report {
            from: metadata.from_addr,
            to: metadata.to_addr,
            data,
        };
        let json = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
        let res = self
            .client
            .publish(Message::new("frisquet/sensors", json, 0));
        return match res {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(SendError::from(e.to_string()))),
        };
    }
}
