use crate::domain::model::UserWebhook;
use alice_architecture::repository::IDBRepository;

#[async_trait::async_trait]
pub trait UserWebhookRepo: IDBRepository<UserWebhook> + Send + Sync {
    async fn get_url_by_user_id(&self, id: &str) -> anyhow::Result<String>;
}
