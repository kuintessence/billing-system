use std::str::FromStr;

use alice_architecture::repository::IReadOnlyRepository;
use database_model::system::prelude::*;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::domain::models::NodeInstance;
use crate::infrastructure::databases::SeaOrmDbRepository;

#[async_trait::async_trait]
impl IReadOnlyRepository<NodeInstance> for SeaOrmDbRepository {
    async fn get_by_id(&self, uuid: &str) -> anyhow::Result<NodeInstance> {
        let model = NodeInstanceEntity::find_by_id(Uuid::from_str(uuid)?)
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such Node Instance"))?;
        model.try_into()
    }

    /// 获取所有对象
    async fn get_all(&self) -> anyhow::Result<Vec<NodeInstance>> {
        unimplemented!()
    }
}
