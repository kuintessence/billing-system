use alice_architecture::model::IAggregateRoot;
use database_model::system::prelude::FlowInstanceModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl IAggregateRoot for FlowInstance {}

#[derive(Serialize, Deserialize)]
pub struct FlowInstance {
    pub id: Uuid,
    pub user_id: Uuid,
}

impl From<FlowInstanceModel> for FlowInstance {
    fn from(model: FlowInstanceModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
        }
    }
}
