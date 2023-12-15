use alice_architecture::repository::ReadOnlyRepository;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::domain::model::FlowInstance;
use crate::infrastructure::database::OrmRepo;

#[async_trait::async_trait]
impl ReadOnlyRepository<FlowInstance> for OrmRepo {
    async fn get_by_id(&self, uuid: Uuid) -> anyhow::Result<FlowInstance> {
        let model = database_model::prelude::FlowInstance::find_by_id(uuid)
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such Flow Instance"))?;
        Ok(model.into())
    }

    /// 获取所有对象
    async fn get_all(&self) -> anyhow::Result<Vec<FlowInstance>> {
        unimplemented!()
    }
}
