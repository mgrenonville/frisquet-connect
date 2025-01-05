use colored::Colorize;
use deku::prelude::*;
use hex;
use std::fmt;

use crate::config;
use crate::connect::{filter, from_bytes, send_cmd, Cmd, ConnectError, Metadata};
use crate::rf::RFClient;

use super::Assert;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct GasSensorMsg {
    len: u8,
    data: [u8; 18],
    yesterday_ecs_consumption: i16,
    yesterday_chauffage_consumption: i16,
    data2: [u8; 14],
    data3: [u8; 10],
    data4: [u8; 10],
}

impl fmt::Display for GasSensorMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_bytes().map(hex::encode) {
            Ok(data) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    data[0..2].white(),
                    data[2..40].white(),
                    data[40..44].blue(),
                    data[44..48].red(),
                    data[48..114].white(),
                )?;

                write!(f, "\n    GasSensorsMessage")?;
                write!(
                    f,
                    "\n\t {}",
                    format!(
                        "Yesterday ECS Gas Consumption: {} kWh",
                        self.yesterday_ecs_consumption
                    )
                    .blue()
                )?;
                write!(
                    f,
                    "\n\t {}",
                    format!(
                        "Yesterday Chauffage Gas Consumption: {} kWh",
                        self.yesterday_chauffage_consumption
                    )
                    .red()
                )
            }
            Err(_) => write!(f, "ERROR"),
        }
    }
}

impl Assert for GasSensorMsg {
    fn assert(&self) -> bool {
        true
    }
}

pub fn connect_gas_sensor(
    rf: &mut Box<dyn RFClient>,
    config: &mut config::Frisquet,
) -> Result<(Metadata, GasSensorMsg), ConnectError> {
    rf.set_network_id(Vec::from(config.network_id()?))?;

    let req_id = config.next_req_id()?;
    // 7a18001c
    send_cmd(
        rf,
        0x7e, // from
        0x80, // to
        config.association_id()?,
        req_id,
        01,
        03,
        &Cmd {
            data: vec![0x7a, 0x18, 0x00, 0x1c],
        },
    )?;

    loop {
        match filter(&rf.recv()?, 0x80, 0x7e, config.association_id()?, req_id)? {
            Some(payload) => {
                let (meta, data) = from_bytes(&payload)?;
                println!("RECV {}{}", meta, data);
                return Ok((meta, data));
            }
            None => {}
        }
    }
}
