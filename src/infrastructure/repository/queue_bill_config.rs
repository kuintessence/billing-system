use database_model::{
    system::prelude::{QueueBillConfigColumn, QueueBillConfigEntity},
    utils::WithDecimalFileds,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    domain::{model::QueueBillConfig, repository::QueueBillConfigRepo},
    infrastructure::database::SeaOrmDbRepository,
};

#[async_trait::async_trait]
impl QueueBillConfigRepo for SeaOrmDbRepository {
    async fn get_by_queue_id(&self, id: &str) -> anyhow::Result<QueueBillConfig> {
        let mut model = QueueBillConfigEntity::find()
            .filter(QueueBillConfigColumn::QueueId.eq(Uuid::from_str(id)?))
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such queue with id: {id}"))?;
        model.rescale_all_to(10);
        Ok(model.into())
    }
}
