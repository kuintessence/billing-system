use std::str::FromStr;

use alice_infrastructure::error::{AliceResult, AliceError, AliceCommonError};
use uuid::Uuid;

pub mod controller;
mod dto;

fn extract_uuid(s: &str) -> AliceResult<Uuid> {
    Uuid::from_str(s).map_err(|e| {
        AliceError::new(AliceCommonError::InvalidRequest {
            error_description: format!(r#"error when parse uuid from "{s}": {e}"#),
        })
    })
}
