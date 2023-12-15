use alice_architecture::model::AggregateRoot;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, AggregateRoot)]
pub struct NodeInstanceBilling {
    pub id: Uuid,
    pub node_instance_id: Uuid,
    pub flow_instance_id: Uuid,
    pub cpu: i64,
    pub memory: i64,
    pub storage: i64,
    pub cpu_time: i64,
    pub wall_time: i64,
    pub price: Decimal,
    pub formula: String,
}

impl From<database_model::node_instance_billing::Model> for NodeInstanceBilling {
    fn from(model: database_model::node_instance_billing::Model) -> Self {
        let database_model::node_instance_billing::Model {
            id,
            node_instance_id,
            flow_instance_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            price,
            formula,
            created_time: _,
            modified_time: _,
        } = model;

        Self {
            id,
            node_instance_id,
            flow_instance_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            price,
            formula,
        }
    }
}
