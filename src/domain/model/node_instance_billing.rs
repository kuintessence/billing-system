use alice_architecture::IAggregateRoot;
use chrono::Utc;
use database_model::system::prelude::NodeInstanceBillingModel;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, IAggregateRoot)]
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

impl From<NodeInstanceBillingModel> for NodeInstanceBilling {
    fn from(model: NodeInstanceBillingModel) -> Self {
        let NodeInstanceBillingModel {
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

impl From<NodeInstanceBilling> for NodeInstanceBillingModel {
    fn from(value: NodeInstanceBilling) -> Self {
        let NodeInstanceBilling {
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
        } = value;

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
            created_time: Utc::now(),
            modified_time: Utc::now(),
        }
    }
}
