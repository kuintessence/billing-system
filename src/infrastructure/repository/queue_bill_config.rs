use database_model::queue_bill_config;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    domain::{model::QueueBillConfig, repository::QueueBillConfigRepo},
    infrastructure::database::OrmRepo,
};

#[async_trait::async_trait]
impl QueueBillConfigRepo for OrmRepo {
    async fn get_by_queue_id(&self, id: &str) -> anyhow::Result<QueueBillConfig> {
        let mut model = queue_bill_config::Entity::find()
            .filter(queue_bill_config::Column::QueueId.eq(Uuid::from_str(id)?))
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such queue with id: {id}"))?;
        model.cpu.rescale(10);
        model.memory.rescale(10);
        model.storage.rescale(10);
        model.cpu_time.rescale(10);
        model.wall_time.rescale(10);
        Ok(model.into())
    }
}
