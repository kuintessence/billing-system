use super::SeaOrmDbRepository;
use crate::kernel::prelude::*;
use alice_architecture::repository::IReadOnlyRepository;
use database_model::system::prelude::*;
use sea_orm::EntityTrait;
use std::str::FromStr;
use uuid::Uuid;

#[async_trait::async_trait]
impl IReadOnlyRepository<FlowInstance> for SeaOrmDbRepository {
    async fn get_by_id(&self, uuid: &str) -> anyhow::Result<FlowInstance> {
        let model = FlowInstanceEntity::find_by_id(Uuid::from_str(uuid)?)
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
