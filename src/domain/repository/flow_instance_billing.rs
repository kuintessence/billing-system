use alice_architecture::repository::DBRepository;
use uuid::Uuid;

use crate::domain::model::FlowInstanceBilling;

#[async_trait::async_trait]
pub trait FlowInstanceBillingRepo: DBRepository<FlowInstanceBilling> + Sync + Send {
    async fn get_by_flow_instance_id(&self, id: Uuid) -> anyhow::Result<FlowInstanceBilling>;
    async fn insert_or_update(&self, entity: FlowInstanceBilling) -> anyhow::Result<()>;
}
