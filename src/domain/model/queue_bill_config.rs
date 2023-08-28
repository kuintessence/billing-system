use alice_architecture::IAggregateRoot;
use database_model::system::prelude::QueueBillConfigModel;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Default, Debug, IAggregateRoot)]
pub struct QueueBillConfig {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub cpu: Decimal,
    pub memory: Decimal,
    pub storage: Decimal,
    pub cpu_time: Decimal,
    pub wall_time: Decimal,
    pub formula: String,
}

impl From<QueueBillConfigModel> for QueueBillConfig {
    fn from(model: QueueBillConfigModel) -> Self {
        let QueueBillConfigModel {
            id,
            queue_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            formula,
            created_time: _,
            modified_time: _,
        } = model;

        Self {
            id,
            queue_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            formula,
        }
    }
}
