use crate::domain::model::NodeInstanceBilling;
use alice_architecture::repository::IDBRepository;

#[async_trait::async_trait]
pub trait NodeInstanceBillingRepository: IDBRepository<NodeInstanceBilling> + Send + Sync {
    async fn get_all_by_flow_instance_id(
        &self,
        id: &str,
    ) -> anyhow::Result<Vec<NodeInstanceBilling>>;
}
