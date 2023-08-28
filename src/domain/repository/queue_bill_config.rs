use crate::domain::model::QueueBillConfig;

#[async_trait::async_trait]
pub trait QueueBillConfigRepo: Send + Sync {
    async fn get_by_queue_id(&self, id: &str) -> anyhow::Result<QueueBillConfig>;
}
