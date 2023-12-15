use alice_architecture::repository::ReadOnlyRepository;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::domain::model::NodeInstance;
use crate::infrastructure::database::OrmRepo;

#[async_trait::async_trait]
impl ReadOnlyRepository<NodeInstance> for OrmRepo {
    async fn get_by_id(&self, uuid: Uuid) -> anyhow::Result<NodeInstance> {
        let model = database_model::prelude::NodeInstance::find_by_id(uuid)
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such Node Instance"))?;
        model.try_into()
    }
}
