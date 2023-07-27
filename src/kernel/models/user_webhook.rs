use database_model::system::prelude::UserWebhookModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl alice_architecture::model::IAggregateRoot for UserWebhook {}
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct UserWebhook {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
}

impl From<UserWebhookModel> for UserWebhook {
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
