use std::error::Error;

use serde::Serialize;

use crate::connect::Metadata;

pub mod mqtt;

struct ReportingError {
    msg: String,
}

pub trait ReportingClient {
    fn forward<T: Serialize>(&mut self, metadata: Metadata, data: T) -> Result<(), Box<dyn Error>>;
}
