use alice_architecture::model::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AggregateRoot)]
pub struct FlowInstance {
    pub id: Uuid,
    pub user_id: Uuid,
}

impl From<database_model::flow_instance::Model> for FlowInstance {
    fn from(model: database_model::flow_instance::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
        }
    }
}
