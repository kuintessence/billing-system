use alice_architecture::model::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, AggregateRoot)]
pub struct UserWebhook {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
}

impl From<database_model::user_webhook::Model> for UserWebhook {
    fn from(model: UserWebhookModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            url: model.url,
        }
    }
}

impl From<UserWebhook> for UserWebhookModel {
    fn from(value: UserWebhook) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            url: value.url,
        }
    }
}
