use crate::domain::model::FlowInstanceBilling;
use alice_architecture::repository::IDBRepository;

#[async_trait::async_trait]
pub trait FlowInstanceBillingRepo: IDBRepository<FlowInstanceBilling> + Sync + Send {
    async fn get_by_flow_instance_id(&self, id: &str) -> anyhow::Result<FlowInstanceBilling>;
    async fn insert_or_update(&self, entity: FlowInstanceBilling) -> anyhow::Result<()>;
}
