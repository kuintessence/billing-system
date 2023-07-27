use chrono::Utc;
use database_model::system::prelude::ClusterIdSettingsModel;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl alice_architecture::model::IAggregateRoot for ClusterIdSettings {}
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ClusterIdSettings {
    pub id: Uuid,
    pub cluster_id: Uuid,
    pub cpu: Decimal,
    pub memory: Decimal,
    pub storage: Decimal,
    pub cpu_time: Decimal,
    pub wall_time: Decimal,
    pub formula: String,
}

impl From<ClusterIdSettingsModel> for ClusterIdSettings {
    fn from(model: ClusterIdSettingsModel) -> Self {
        let ClusterIdSettingsModel {
            id,
            cluster_id,
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
            cluster_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            formula,
        }
    }
}

impl From<ClusterIdSettings> for ClusterIdSettingsModel {
    fn from(value: ClusterIdSettings) -> Self {
        let ClusterIdSettings {
            id,
            cluster_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            formula,
        } = value;

        Self {
            id,
            cluster_id,
            cpu,
            memory,
            storage,
            cpu_time,
            wall_time,
            formula,
            created_time: Utc::now(),
            modified_time: Utc::now(),
        }
    }
}
