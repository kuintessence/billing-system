use alice_architecture::IAggregateRoot;
use database_model::system::prelude::FlowInstanceModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, IAggregateRoot)]
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
