use alice_architecture::model::AggregateRoot;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Default, Debug, AggregateRoot)]
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

impl From<database_model::queue_bill_config::Model> for QueueBillConfig {
    fn from(model: database_model::queue_bill_config::Model) -> Self {
        let database_model::queue_bill_config::Model {
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
