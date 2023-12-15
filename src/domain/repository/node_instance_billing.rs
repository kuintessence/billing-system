use alice_architecture::repository::DBRepository;
use uuid::Uuid;

use crate::domain::model::NodeInstanceBilling;

#[async_trait::async_trait]
pub trait NodeInstanceBillingRepo: DBRepository<NodeInstanceBilling> + Send + Sync {
    async fn get_all_by_flow_instance_id(
        &self,
        id: Uuid,
    ) -> anyhow::Result<Vec<NodeInstanceBilling>>;
}
